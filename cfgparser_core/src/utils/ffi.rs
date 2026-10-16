struct FfiParams {
    key: Vec<u8>,
    enc_type_i32: i32,
    offset: usize,
}

/// helper function deisgned to take a key passed in as a C string
/// and convert it to a byte array.
fn convert_key_from_c(raw_key: &*const std::ffi::c_char) -> Vec<u8> {
    // if null is passed in as the key, use q as the default;
    // otherwise use the char* key passed in.
    if raw_key.is_null() {
        return "q".as_bytes().to_vec();
    }

    // convert the input to a CStr.
    //
    // this borrows the value from the memory that is owned
    // by the caller.
    let key_cstr: &std::ffi::CStr = unsafe { std::ffi::CStr::from_ptr(*raw_key) };

    // convert the CStr holding the key to bytes, then a Vec
    // and return it.
    key_cstr.to_bytes().to_vec()
}

/// helper function designed to format the C2 address held in the configuration
/// struct and return it as a *char that can be used by C.
fn format_address_c(configuration: crate::models::core::Configuration) -> *const std::ffi::c_char {
    let address: String = format!(
        "{}://{}:{}",
        configuration.scheme, configuration.host, configuration.port
    );

    // convert the String (rust) into a CString so it can be converted
    // into a char* and returned.
    let address_cstring: std::ffi::CString = match std::ffi::CString::new(address) {
        Ok(result) => result,
        Err(_) => return std::ptr::null(),
    };

    address_cstring.into_raw()
}

/// helper function designed to consolidate the parsing logic
/// of the FFI parameters.
fn parse_common_ffi_params(
    raw_key: *const std::ffi::c_char,
    enc_type: std::ffi::c_int,
    raw_offset: std::ffi::c_uint,
) -> Option<FfiParams> {
    let key: Vec<u8> = convert_key_from_c(&raw_key);

    let enc_type_i32: i32 = enc_type.try_into().ok()?;
    let offset: usize = raw_offset.try_into().ok()?;

    Some(FfiParams {
        key,
        enc_type_i32,
        offset,
    })
}

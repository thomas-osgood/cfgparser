use super::*;
use std::ffi::CString;

#[test]
fn test_convert_key_from_c_null() {
    let null_ptr: *const std::ffi::c_char = std::ptr::null();
    let key = convert_key_from_c(&null_ptr);
    // Defaults to "q" as bytes if null is passed
    assert_eq!(key, b"q");
}

#[test]
fn test_convert_key_from_c_valid() {
    let c_str = CString::new("secret_key").unwrap();
    let ptr = c_str.as_ptr();
    let key = convert_key_from_c(&ptr);
    assert_eq!(key, b"secret_key");
}

#[test]
fn test_parse_common_ffi_params_success() {
    let c_str = CString::new("my_key").unwrap();
    let params = parse_common_ffi_params(c_str.as_ptr(), 0, 10);
    assert!(params.is_some());
    let p = params.unwrap();
    assert_eq!(p.key, b"my_key");
    assert_eq!(p.enc_type_i32, 0);
    assert_eq!(p.offset, 10);
}

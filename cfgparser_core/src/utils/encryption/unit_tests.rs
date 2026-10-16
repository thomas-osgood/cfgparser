use super::*;

#[test]
fn test_get_decryptor_invalid_type() {
    // Pass an out-of-range integer for the encryption type
    let result = get_decryptor(b"key", 999);
    assert!(result.is_err());
}

#[test]
fn test_get_decryptor_xor() {
    // Assuming 0 corresponds to XOR based on EncryptionType mapping
    let result = get_decryptor(b"key", 0);
    assert!(result.is_ok());
    match result.unwrap() {
        cfgparser_encryption::DynamicCipher::XOR(_) => (),
        _ => panic!("Expected XOR cipher"),
    }
}

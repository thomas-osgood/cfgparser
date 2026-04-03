use super::*;

#[test]
/// function designed to test the JSON deserialization
/// functionality of deserialize_payload. this function is
/// designed to deserialize a string into a Configuration struct.
fn test_deserialize_payload() {
    let expected: crate::models::core::Configuration = crate::models::core::Configuration::new(
        "localhost".to_string(),
        443,
        crate::models::core::SchemeType::HTTPS,
    );
    let serialized: String =
        "{\"host\": \"localhost\", \"port\": 443, \"scheme\": \"https\"}".to_string();

    let deserialized: crate::models::core::Configuration =
        deserialize_payload(serialized).expect("error deserializing payload");

    let is_equal: bool =
        (deserialized.host == expected.host) && (deserialized.port == expected.port);

    assert_eq!(is_equal, true);
}

#[test]
/// function designed to test the transform_payload function
/// that XOR decrypts and base64 decodes the payload bytes.
///
/// the hard-coded "message" that is going to be transformed
/// into plaintext has been base64 encoded and XOR encrypted
/// using tools outside of rust.
fn test_transform_payload_xor() {
    let key: &[u8] = "testkey".as_bytes();
    let decryptor: cfgparser_encryption::xor::engine::XORCipher =
        cfgparser_encryption::xor::engine::XORCipher::new(key.to_vec());
    let expected: String = "This is a decrypted message".to_string();
    let message: [u8; 36] = [
        34, 34, 27, 4, 8, 28, 59, 4, 6, 10, 54, 3, 44, 62, 38, 9, 42, 71, 33, 80, 26, 60, 55, 31,
        46, 40, 39, 13, 46, 61, 61, 14, 50, 50, 29, 24,
    ];

    let result: Vec<u8> =
        transform_payload(decryptor, &message).expect("error transforming payload");
    let str_result: String =
        String::from_utf8(result).expect("error transforming result to string");

    assert_eq!(str_result, expected);
}

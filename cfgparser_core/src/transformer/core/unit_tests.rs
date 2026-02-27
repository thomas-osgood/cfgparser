use super::*;

#[test]
/// function designed to test the JSON deserialization
/// functionality of deserialize_payload. this function is
/// designed to deserialize a string into a Configuration struct.
fn test_deserialize_payload() {
    let expected: crate::models::core::Configuration =
        crate::models::core::Configuration::new("localhost".to_string(), 443);
    let serialized: String = "{\"host\": \"localhost\", \"port\": 443}".to_string();

    let deserialized: crate::models::core::Configuration =
        deserialize_payload(serialized).expect("error deserializing payload");

    let is_equal: bool =
        (deserialized.host == expected.host) && (deserialized.port == expected.port);

    assert_eq!(is_equal, true);
}

#[test]
fn test_transform_payload() {
    let key: &str = "testkey";
    let expected: String = "This is a decrypted message".to_string();
    let message: [u8; 36] = [
        34, 34, 27, 4, 8, 28, 59, 4, 6, 10, 54, 3, 44, 62, 38, 9, 42, 71, 33, 80, 26, 60, 55, 31,
        46, 40, 39, 13, 46, 61, 61, 14, 50, 50, 29, 24,
    ];

    let result: Vec<u8> =
        transform_payload(key.as_bytes(), &message).expect("error transforming payload");
    let str_result: String =
        String::from_utf8(result).expect("error transforming result to string");

    assert_eq!(str_result, expected);
}

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

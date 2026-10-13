use super::*;

#[test]
/// test designed to confirm the Default implementation works as expected
/// for the Configuration struct.
fn test_configuration_struct_default() {
    let expected: Configuration = Configuration {
        host: "localhost".to_string(),
        port: 80,
        scheme: SchemeType::default(),
    };
    let test_struct: Configuration = Configuration::default();
    assert_eq!(test_struct, expected);
}

#[test]
/// test designed to confirm the serde default value works as expected.
fn test_configuration_deserialize_default() {
    let test: &str = "{\"host\": \"localhost\"}";
    let expected: Configuration = Configuration {
        host: "localhost".to_string(),
        port: 80,
        scheme: SchemeType::default(),
    };
    let result: Configuration = serde_json::from_str(test).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn test_configuration_deserialize_custom() {
    let test: &str = "{\"host\": \"example.com\", \"port\": 443, \"scheme\": \"https\"}";
    let expected: Configuration = Configuration {
        host: "example.com".to_string(),
        port: 443,
        scheme: SchemeType::HTTPS,
    };
    let result: Configuration = serde_json::from_str(test).unwrap();

    assert_eq!(result, expected);
}

#[test]
fn test_scheme_type_display() {
    assert_eq!(SchemeType::HTTP.to_string(), "http");
    assert_eq!(SchemeType::HTTPS.to_string(), "https");
}

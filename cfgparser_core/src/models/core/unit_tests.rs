use super::*;

#[test]
/// test designed to confirm the serde default value works as expected.
fn test_deserialize_default() {
    let test: &str = "{\"host\": \"localhost\"}";
    let expected: Configuration = Configuration {
        host: "localhost".to_string(),
        port: 80,
    };
    let result: Configuration = serde_json::from_str(test).unwrap();

    assert_eq!(result, expected);
}

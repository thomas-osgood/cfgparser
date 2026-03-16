use super::*;

#[test]
/// test designed to confirm the "read()" function operates as
/// expected and returns the proper result.
fn test_read() -> Result<(), Box<dyn std::error::Error>> {
    let expected: models::core::Configuration = models::core::Configuration {
        host: "secrethost".to_string(),
        port: 8000,
        scheme: models::core::SchemeType::default(),
    };

    let unexpected: models::core::Configuration = models::core::Configuration {
        host: "secrethost22".to_string(),
        port: 80,
        scheme: models::core::SchemeType::default(),
    };

    let key: &str = "secret";
    let reader: extractor::core::TestExtractor = extractor::core::TestExtractor;

    let extracted: models::core::Configuration = read(reader, key.as_bytes())?;

    assert_eq!(extracted, expected);
    assert_ne!(extracted, unexpected);

    Ok(())
}

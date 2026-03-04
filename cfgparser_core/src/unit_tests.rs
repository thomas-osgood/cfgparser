use super::*;

#[test]
fn test_read() -> Result<(), Box<dyn std::error::Error>> {
    let expected: models::core::Configuration = models::core::Configuration {
        host: "secrethost".to_string(),
        port: 8000,
    };
    let key: &str = "secret";
    let reader: extractor::core::TestExtractor = extractor::core::TestExtractor;

    let extracted: models::core::Configuration = read(reader, key.as_bytes())?;

    assert_eq!(extracted, expected);

    Ok(())
}

use super::*;

/// helper type that represents the generic return type used
/// by most tests.
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
/// test designed to confirm the "read()" function operates as
/// expected and returns the proper result.
fn test_read() -> TestResult {
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
    let decryptor: cfgparser_encryption::xor::engine::XORCipher =
        cfgparser_encryption::xor::engine::XORCipher::new(key.as_bytes().to_vec());
    let reader: extractor::core::TestExtractor = extractor::core::TestExtractor;

    let extracted: models::core::Configuration = read(reader, decryptor)?;

    assert_eq!(extracted, expected);
    assert_ne!(extracted, unexpected);

    Ok(())
}

#[test]
/// test designed to confirm the "read()" function operates as
/// expected and returns the proper result when a ViginereCipher
/// decryptor is passed to it.
fn test_read_viginere() -> TestResult {
    let key: Vec<u8> = b"secretkey".to_vec();
    let cipher1: cfgparser_encryption::viginere::engine::ViginereCipher =
        match cfgparser_encryption::viginere::engine::ViginereCipher::new(key) {
            Ok(c) => c,
            Err(e) => return Err(e),
        };
    let expected: models::core::Configuration = models::core::Configuration::new(
        "secrethost".to_string(),
        1234,
        models::core::SchemeType::HTTP,
    );
    let ciphertext: Vec<u8> = b"hello there everybodywcLff3G0SnmyMpEpR3TpbYlxt3UbVGYagI9phVS6MBWcOqUlSGHrC2jcfPEmMaEkrLK0mGH9\x00\x00\x00\x00\x00\x00\x00H".to_vec();
    let reader: extractor::core::BytesExtractor = extractor::core::BytesExtractor::new(ciphertext);

    let plaintext: models::core::Configuration = match read(reader, cipher1) {
        Ok(result) => result,
        Err(e) => return Err(e),
    };

    assert_eq!(plaintext, expected);

    Ok(())
}

#[test]
/// test designed to confirm the "read()" function operates as expected
/// when passed a BytesExtractor struct.
fn test_read_bytesextractor() -> TestResult {
    let expected: models::core::Configuration = models::core::Configuration {
        host: "malserver".to_string(),
        port: 9999,
        scheme: models::core::SchemeType::HTTPS,
    };

    let reader: extractor::core::BytesExtractor = extractor::core::BytesExtractor::new(vec![
        116, 104, 105, 115, 32, 105, 115, 32, 97, 32, 98, 117, 110, 99, 104, 32, 111, 102, 32, 106,
        117, 110, 107, 32, 100, 97, 116, 97, 32, 116, 104, 97, 116, 32, 105, 115, 32, 103, 111,
        105, 110, 103, 32, 116, 111, 32, 98, 101, 32, 117, 115, 101, 100, 32, 116, 111, 32, 115,
        112, 111, 111, 102, 32, 97, 32, 102, 105, 108, 101, 22, 24, 40, 29, 7, 64, 47, 82, 59, 15,
        28, 6, 43, 31, 84, 27, 3, 42, 60, 9, 16, 15, 56, 30, 6, 26, 40, 17, 59, 38, 57, 22, 0, 65,
        47, 67, 40, 8, 29, 2, 60, 53, 9, 71, 42, 32, 22, 5, 59, 11, 61, 11, 3, 53, 51, 7, 59, 49,
        59, 83, 58, 34, 40, 29, 1, 59, 51, 21, 17, 28, 57, 88, 0, 0, 0, 0, 0, 0, 0, 72,
    ]);

    let key: &str = "sabre";
    let decryptor: cfgparser_encryption::xor::engine::XORCipher =
        cfgparser_encryption::xor::engine::XORCipher::new(key.as_bytes().to_vec());
    let extracted: models::core::Configuration = read(reader, decryptor)?;

    assert_eq!(extracted, expected);

    Ok(())
}

#[test]
/// test designed to confirm the "read_from_vec()" function operates as expected.
fn test_read_from_vec() -> TestResult {
    let bytes_vec: Vec<u8> = vec![
        116, 104, 105, 115, 32, 105, 115, 32, 97, 32, 98, 117, 110, 99, 104, 32, 111, 102, 32, 106,
        117, 110, 107, 32, 100, 97, 116, 97, 32, 116, 104, 97, 116, 32, 105, 115, 32, 103, 111,
        105, 110, 103, 32, 116, 111, 32, 98, 101, 32, 117, 115, 101, 100, 32, 116, 111, 32, 115,
        112, 111, 111, 102, 32, 97, 32, 102, 105, 108, 101, 22, 24, 40, 29, 7, 64, 47, 82, 59, 15,
        28, 6, 43, 31, 84, 27, 3, 42, 60, 9, 16, 15, 56, 30, 6, 26, 40, 17, 59, 38, 57, 22, 0, 65,
        47, 67, 40, 8, 29, 2, 60, 53, 9, 71, 42, 32, 22, 5, 59, 11, 61, 11, 3, 53, 51, 7, 59, 49,
        59, 83, 58, 34, 40, 29, 1, 59, 51, 21, 17, 28, 57, 88, 0, 0, 0, 0, 0, 0, 0, 72,
    ];
    let expected: models::core::Configuration = models::core::Configuration {
        host: "malserver".to_string(),
        port: 9999,
        scheme: models::core::SchemeType::HTTPS,
    };
    let key: &str = "sabre";
    let decryptor: cfgparser_encryption::xor::engine::XORCipher =
        cfgparser_encryption::xor::engine::XORCipher::new(key.as_bytes().to_vec());

    let extracted: models::core::Configuration = read_from_vec(bytes_vec, decryptor)?;

    assert_eq!(extracted, expected);

    Ok(())
}

use crate::{Decryptor, Encryptor};

use super::*;

#[test]
/// test designed to confirm the decrypt functionality of the
/// AESCipher is working as expected.
fn test_decrypt() -> Result<(), Box<dyn std::error::Error>> {
    let ciphertext: Vec<u8> = vec![
        66, 168, 189, 189, 234, 64, 241, 80, 19, 32, 71, 136, 110, 231, 230, 187, 172, 117, 86,
        127, 195, 68, 233, 166, 8, 215, 110, 4, 78, 29, 108, 177, 149, 241, 185, 183, 191, 78, 205,
        233, 246, 77, 10, 158,
    ];

    let key: Vec<u8> = vec![
        57, 72, 60, 6, 7, 247, 134, 240, 254, 56, 39, 120, 58, 56, 12, 209, 39, 26, 66, 154, 78,
        38, 106, 196, 105, 68, 79, 66, 220, 128, 101, 177,
    ];

    let expected: Vec<u8> = b"this is a secret".to_vec();
    let cipher: AESCipher = match AESCipher::new(key) {
        Ok(c) => c,
        Err(e) => return Err(e.to_string().into()),
    };

    let plaintext: Vec<u8> = cipher.decrypt(ciphertext)?;

    assert_eq!(plaintext, expected);

    Ok(())
}

#[test]
/// test designed to confirm the encrypt functionality of the
/// AESCipher is working as expected.
///
/// because the nonce is randomly generate each time, the test
/// is conducted by encrypting a known string and confirming
/// the decrypted result of the `encrypt()` function resolved to
/// the original plaintext.
fn test_encrypt() -> Result<(), Box<dyn std::error::Error>> {
    let key: Vec<u8> = vec![
        57, 72, 60, 6, 7, 247, 134, 240, 254, 56, 39, 120, 58, 56, 12, 209, 39, 26, 66, 154, 78,
        38, 106, 196, 105, 68, 79, 66, 220, 128, 101, 177,
    ];
    let cipher: AESCipher = match AESCipher::new(key) {
        Ok(c) => c,
        Err(e) => return Err(e.to_string().into()),
    };
    let plaintext: Vec<u8> = b"this is a secret".to_vec();

    let result: Vec<u8> = cipher.decrypt(cipher.encrypt(plaintext.clone())?)?;

    assert_eq!(result, plaintext);

    Ok(())
}

#[test]
/// test designed to confirm the key validation logic works
/// as expected.
fn test_validate_key() -> Result<(), AESError> {
    let test_key_1: Vec<u8> = vec![
        116, 104, 105, 115, 32, 105, 115, 32, 97, 110, 32, 105, 110, 118, 97, 108, 105, 100, 32,
        107, 101, 121,
    ];
    let test_key_2: Vec<u8> = vec![
        57, 72, 60, 6, 7, 247, 134, 240, 254, 56, 39, 120, 58, 56, 12, 209, 39, 26, 66, 154, 78,
        38, 106, 196, 105, 68, 79, 66, 220, 128, 101, 177,
    ];

    assert!(validate_key(test_key_1).is_err());
    assert!(validate_key(test_key_2).is_ok());

    Ok(())
}

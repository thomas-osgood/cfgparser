use crate::{Decryptor, Encryptor};

use super::*;

#[test]
/// test designed to make sure the encrypt_decrypt function logic
/// works as expected.
fn test_encrypt_decrypt() {
    let test_key: Vec<u8> = "mykey".to_string().into_bytes();
    let test_message: Vec<u8> = "this is a secret message".to_string().into_bytes();
    let result_full_cycle: Vec<u8> = encrypt_decrypt(
        test_key.clone(),
        encrypt_decrypt(test_key.clone(), test_message.clone()),
    );

    let test_key2: Vec<u8> = "msgpadkey".as_bytes().to_vec();
    let test_message2: Vec<u8> = "this is my secret message. you cannot get the contents of it."
        .as_bytes()
        .to_vec();
    let expected: Vec<u8> = vec![
        25, 27, 14, 3, 65, 13, 24, 69, 20, 20, 83, 20, 21, 2, 22, 14, 17, 89, 0, 22, 20, 3, 0, 3,
        14, 75, 89, 20, 28, 18, 80, 2, 5, 5, 11, 22, 25, 83, 0, 21, 21, 68, 31, 13, 28, 77, 16, 8,
        30, 21, 1, 5, 17, 10, 77, 28, 1, 80, 8, 16, 69,
    ];
    let result: Vec<u8> = encrypt_decrypt(test_key2, test_message2);

    let test_message3: Vec<u8> = vec![];
    let result_empty: Vec<u8> = encrypt_decrypt(test_key, test_message3);

    assert_eq!(result_full_cycle, test_message);
    assert_eq!(result, expected);
    assert_eq!(result_empty, vec![]);
}

#[test]
/// test designed to confirm all functions in the XORCipher are
/// operating as expected.
fn test_xorcipher() -> Result<(), Box<dyn std::error::Error>> {
    let key: Vec<u8> = "msgpadkey".as_bytes().to_vec();
    let cipher1: XORCipher = XORCipher::new(key);
    let test_message: Vec<u8> = "this is my secret message. you cannot get the contents of it."
        .as_bytes()
        .to_vec();
    let expected_encrypt: Vec<u8> = vec![
        25, 27, 14, 3, 65, 13, 24, 69, 20, 20, 83, 20, 21, 2, 22, 14, 17, 89, 0, 22, 20, 3, 0, 3,
        14, 75, 89, 20, 28, 18, 80, 2, 5, 5, 11, 22, 25, 83, 0, 21, 21, 68, 31, 13, 28, 77, 16, 8,
        30, 21, 1, 5, 17, 10, 77, 28, 1, 80, 8, 16, 69,
    ];

    let result_decrypt: Vec<u8> = cipher1.decrypt(expected_encrypt.clone())?;
    let result_encrypt: Vec<u8> = cipher1.encrypt(test_message.clone())?;

    assert_eq!(test_message, result_decrypt);
    assert_eq!(expected_encrypt, result_encrypt);

    Ok(())
}

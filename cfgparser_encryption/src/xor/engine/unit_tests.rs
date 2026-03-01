use super::*;

#[test]
/// test designed to make sure the full cycle (encrypt then decrypt) results
/// in the original message being returned.
fn test_encrypt_decrypt() {
    let test_key: Vec<u8> = "mykey".to_string().into_bytes();
    let test_message: Vec<u8> = "this is a secret message".to_string().into_bytes();
    let result_full_cycle: Vec<u8> = encrypt_decrypt(
        test_key.clone(),
        encrypt_decrypt(test_key.clone(), test_message.clone()),
    );

    assert_eq!(result_full_cycle, test_message);
}

#[test]
/// test designed to confirm the encrypt_decrypt function encrypts a message correctly.
fn test_encrypt() {
    let test_key: Vec<u8> = "msgpadkey".as_bytes().to_vec();
    let test_msg: Vec<u8> = "this is my secret message. you cannot get the contents of it."
        .as_bytes()
        .to_vec();
    let expected: Vec<u8> = vec![
        25, 27, 14, 3, 65, 13, 24, 69, 20, 20, 83, 20, 21, 2, 22, 14, 17, 89, 0, 22, 20, 3, 0, 3,
        14, 75, 89, 20, 28, 18, 80, 2, 5, 5, 11, 22, 25, 83, 0, 21, 21, 68, 31, 13, 28, 77, 16, 8,
        30, 21, 1, 5, 17, 10, 77, 28, 1, 80, 8, 16, 69,
    ];

    let result: Vec<u8> = encrypt_decrypt(test_key, test_msg);

    assert_eq!(result, expected);
}

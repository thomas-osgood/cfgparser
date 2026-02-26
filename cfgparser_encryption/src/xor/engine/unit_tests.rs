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

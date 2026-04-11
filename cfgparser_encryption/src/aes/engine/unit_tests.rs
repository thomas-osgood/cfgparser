use crate::Decryptor;

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

    println!("Plaintext: {:?}", plaintext);

    assert_eq!(plaintext, expected);

    Ok(())
}

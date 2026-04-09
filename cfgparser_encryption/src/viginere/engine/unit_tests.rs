use super::*;
use crate::{Decryptor, Encryptor};

#[test]
/// test designed to make sure the decrypt function works as expected.
///
/// the decrypt function uses multiple helper functions to carry out
/// its logic. this serves as a test of the logic and as a test to confirm
/// all functions call the others correctly.
fn test_decrypt() -> Result<(), Box<dyn std::error::Error>> {
    let key: Vec<u8> = "there".into();
    let ciphertext: Vec<u8> = "alpcs xciicuvhp".into();
    let ciphertext2: Vec<u8> = "Alpcs xciicuvhp".into();

    let expected: Vec<u8> = "hello everybody".into();
    let expected2: Vec<u8> = "Hello everybody".into();

    let cipher: ViginereCipher = ViginereCipher::new(key)?;

    let result: Vec<u8> = cipher.decrypt(ciphertext).unwrap();
    let result2: Vec<u8> = cipher.decrypt(ciphertext2).unwrap();

    assert_eq!(result, expected);
    assert_eq!(result2, expected2);

    Ok(())
}

#[test]
/// test designed to make sure the encrypt function (which uses the
/// rotate and is_letter functions) logic works as expected.
///
/// the encrypt function uses multiple helper functions to carry out
/// its logic. this serves as a test of the logic and as a test to confirm
/// all functions call the others correctly.
fn test_encrypt() -> Result<(), Box<dyn std::error::Error>> {
    let key: Vec<u8> = "there".into();
    let plaintext: Vec<u8> = "hello everybody".into();
    let plaintext2: Vec<u8> = "Hello everybody".into();

    let expected: Vec<u8> = "alpcs xciicuvhp".into();
    let expected2: Vec<u8> = "Alpcs xciicuvhp".into();

    let cipher: ViginereCipher = ViginereCipher::new(key)?;

    let result: Vec<u8> = cipher.encrypt(plaintext).unwrap();
    let result2: Vec<u8> = cipher.encrypt(plaintext2).unwrap();

    assert_eq!(result, expected);
    assert_eq!(result2, expected2);

    Ok(())
}

#[test]
/// test designed to make sure the is_letter function works as
/// expected and is able to determine if a given character is
/// in the alphabet.
fn test_isletter() {
    assert_eq!(is_letter(b'a'), true);
    assert_eq!(is_letter(b'z'), true);
    assert_eq!(is_letter(b'A'), true);
    assert_eq!(is_letter(b'Z'), true);
    assert_eq!(is_letter(b' '), false);
    assert_eq!(is_letter(b'-'), false);
    assert_eq!(is_letter(b'0'), false);
    assert_eq!(is_letter(b'9'), false);
}

#[test]
/// test function designed to check if a given character
/// is lowercase.
fn test_islower() {
    assert_eq!(is_lower(b'c'), true);
    assert_eq!(is_lower(b'C'), false);
    assert_eq!(is_lower(b'_'), false);
    assert_eq!(is_lower(b' '), false);
}

#[test]
/// test function designed to check if a given character
/// is uppercase.
fn test_isupper() {
    assert_eq!(is_upper(b'C'), true);
    assert_eq!(is_upper(b'c'), false);
    assert_eq!(is_upper(b'_'), false);
    assert_eq!(is_upper(b' '), false);
}

#[test]
fn test_adjustkey() {
    assert_eq!(adjust_key(b'a', b'c'), b'c');
    assert_eq!(adjust_key(b'A', b'c'), b'C');
    assert_eq!(adjust_key(b'a', b'C'), b'c');
    assert_eq!(adjust_key(b' ', b'c'), b'c');
    assert_eq!(adjust_key(b'-', b'c'), b'c');
    assert_eq!(adjust_key(b'A', b'C'), b'C');
}

#[test]
/// test designed to make sure the is_valid_key function works
/// as expected and is able to determine if a given key contains
/// one or more chars that are not in the alphabet and is not
/// an empty vector.
fn test_isvalidkey() {
    assert_eq!(is_valid_key("testkey".into()), true);
    assert_eq!(is_valid_key("TESTKEY".into()), true);
    assert_eq!(is_valid_key("i am a key".into()), false);
    assert_eq!(is_valid_key("123489".into()), false);
    assert_eq!(is_valid_key(vec![]), false);
}

#[test]
/// test designed to make sure the rotate function works as expected.
fn test_rotate() {
    assert_eq!(rotate(b'a', b'a', false), b'a');
    assert_eq!(rotate(b'm', b'c', false), b'o');
    assert_eq!(rotate(b'm', b'y', false), b'k');
    assert_eq!(rotate(b'-', b'x', false), b'-');
    assert_eq!(rotate(b'/', b'd', false), b'/');
    assert_eq!(rotate(b'5', b'd', false), b'5');
}

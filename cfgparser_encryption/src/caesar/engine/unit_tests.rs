use super::*;

#[test]
/// test designed to make sure the encrypt function (which uses the
/// rotate and is_letter functions) logic works as expected.
fn test_encrypt() {
    let key: Vec<u8> = "there".to_string().into_bytes();
    let plaintext: Vec<u8> = "hello everybody".to_string().into_bytes();

    let expected: Vec<u8> = "alpcs xciicuvhp".to_string().into_bytes();
    let result: Vec<u8> = encrypt(plaintext, key);

    assert_eq!(result, expected);
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
/// test designed to make sure the rotate function works as expected.
fn test_rotate() {
    assert_eq!(rotate(b'a', b'a'), b'a');
    assert_eq!(rotate(b'm', b'c'), b'o');
    assert_eq!(rotate(b'm', b'y'), b'k');
}

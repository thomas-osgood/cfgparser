use super::*;

#[test]
/// test designed to make sure the encrypt function (which uses the
/// rotate and is_letter functions) logic works as expected.
fn test_encrypt() {
    let key: Vec<u8> = "there".into();
    let plaintext: Vec<u8> = "hello everybody".into();
    let plaintext2: Vec<u8> = "Hello everybody".into();

    let expected: Vec<u8> = "alpcs xciicuvhp".into();
    let expected2: Vec<u8> = "Alpcs xciicuvhp".into();

    let result: Vec<u8> = encrypt(plaintext, key.clone());
    let result2: Vec<u8> = encrypt(plaintext2, key);

    assert_eq!(result, expected);
    assert_eq!(result2, expected2);
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
}

#[test]
/// test designed to make sure the rotate function works as expected.
fn test_rotate() {
    assert_eq!(rotate(b'a', b'a'), b'a');
    assert_eq!(rotate(b'm', b'c'), b'o');
    assert_eq!(rotate(b'm', b'y'), b'k');
    assert_eq!(rotate(b'-', b'x'), b'-');
    assert_eq!(rotate(b'/', b'd'), b'/');
    assert_eq!(rotate(b'5', b'd'), b'5');
}

#[cfg(test)]
mod unit_tests;

const ALPHABET_LEN: u8 = 26;
const LOWER_A: u8 = b'a';
const LOWER_Z: u8 = b'z' + 1;
const UPPER_A: u8 = b'A';
const UPPER_Z: u8 = b'Z' + 1;

/// function designed to take in a key and determine whether it
/// is a valid caesar/viginere cipher key.
pub fn is_valid_key(key: Vec<u8>) -> bool {
    if key.len() < 1 {
        return false;
    }

    for letter in key {
        if !is_letter(letter) {
            return false;
        }
    }
    true
}

/// function designed to implement a viginere cipher.
///
/// this will take in plaintext and a key and rotate each letter
/// in the plaintext using the associated key character.
pub fn encrypt(plaintext: Vec<u8>, key: Vec<u8>) -> Vec<u8> {
    let mut ciphertext: Vec<u8> = vec![];
    let mut key_pos: usize = 0;
    let len_key = key.len();

    if !is_valid_key(key.clone()) {}

    for (_, current) in plaintext.iter().enumerate() {
        // perform rotation and add result to ciphertext.
        ciphertext.push(rotate(*current, key[key_pos % len_key]));

        // only advance the key position if it has been used. this
        // only happens when the current character is a letter.
        if is_letter(*current) {
            key_pos += 1;
        }
    }

    return ciphertext;
}

/// simple helper function designed to determine if a given character
/// is in the alphabet.
fn is_letter(letter: u8) -> bool {
    match letter {
        LOWER_A..LOWER_Z => true,
        UPPER_A..UPPER_Z => true,
        _ => false,
    }
}

/// helper function designed to rotate a letter using a key letter.
///
/// if the letter is a..z it will be subtracted by 'a' to get it into
/// normal range for caesar cipher calculations then rotated and increased
/// by 'a' to get it back up to the char representation.
///
/// if the letter is A..Z it will be subtracted by 'A' to get it into
/// normal range for caesar cipher calculations then rotated and increased
/// by 'A' to get it back up to the char representation.
///
/// if the letter is not part of the alphabet, nothing will happen to it
/// and it will be returned as normal.
fn rotate(letter: u8, key: u8) -> u8 {
    let corrected_key: u8;
    let corrected_letter: u8;
    let rotated: u8;

    match letter {
        LOWER_A..LOWER_Z => {
            corrected_letter = letter - LOWER_A;
            corrected_key = key - LOWER_A;
            rotated = ((corrected_letter + corrected_key) % ALPHABET_LEN) + LOWER_A
        }
        UPPER_A..UPPER_Z => {
            corrected_letter = letter - UPPER_A;
            corrected_key = key - UPPER_A;
            rotated = ((corrected_letter + corrected_key) % ALPHABET_LEN) + UPPER_A
        }
        _ => rotated = letter,
    };

    rotated
}

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

/// function designed to adjust a key's case based on the
/// case of the letter.
fn adjust_key(letter: u8, key: u8) -> u8 {
    let correction: u8 = LOWER_A - UPPER_A;

    let letter_upper: bool = match letter {
        LOWER_A..LOWER_Z => false,
        UPPER_A..UPPER_Z => true,
        _ => return key,
    };

    let key_upper: bool = match key {
        LOWER_A..LOWER_Z => false,
        _ => true,
    };

    if letter_upper && !key_upper {
        key - correction
    } else if !letter_upper && key_upper {
        key + correction
    } else {
        key
    }
}

/// function designed to take in a character and correct its value
/// if it is a letter so it can be within the range 0..25. if the
/// character is not a letter, nothing is done to it.
fn correct_char(chr: u8) -> u8 {
    match chr {
        LOWER_A..LOWER_Z => chr - LOWER_A,
        UPPER_A..UPPER_Z => chr - UPPER_A,
        _ => chr,
    }
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
    let corrected_letter: u8 = correct_char(letter);
    let rotated: u8;

    corrected_key = adjust_key(corrected_letter, correct_char(key));

    match letter {
        LOWER_A..LOWER_Z => rotated = ((corrected_letter + corrected_key) % ALPHABET_LEN) + LOWER_A,
        UPPER_A..UPPER_Z => rotated = ((corrected_letter + corrected_key) % ALPHABET_LEN) + UPPER_A,
        _ => rotated = letter,
    };

    rotated
}

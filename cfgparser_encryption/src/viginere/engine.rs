#[cfg(test)]
mod unit_tests;

const ALPHABET_LEN: u8 = 26;
const LOWER_A: u8 = b'a';
const LOWER_Z: u8 = b'z' + 1;
const UPPER_A: u8 = b'A';
const UPPER_Z: u8 = b'Z' + 1;

/// constant holding difference between ascii "a" and ascii "A".
///
/// this can be used to change the case of a letter from upper to
/// lower or visa versa.
const LOWER_TO_UPPER_DIFF: u8 = LOWER_A - UPPER_A;

/// struct designed to implement encrypt and decrypt for a Viginere cipher.
pub struct ViginereCipher {
    pub key: Vec<u8>,
}

/// function designed to take in a key and determine whether it
/// is a valid caesar/viginere cipher key.
pub fn is_valid_key(key: Vec<u8>) -> bool {
    if key.len() < 1 {
        return false;
    }

    // check for the existence of any element not being a letter.
    //
    // this uses "!" (not) becuase if there are no non-letters in
    // the key, the key is valid.
    //
    // if the condition is true, the key is not valid.
    !key.into_iter().any(|letter| !is_letter(letter))
}

impl ViginereCipher {
    pub fn new(key: Vec<u8>) -> Result<ViginereCipher, Box<dyn std::error::Error>> {
        if key.len() < 1 {
            return Err("key must be of non-zero length".into());
        }
        Ok(ViginereCipher { key })
    }
}

impl crate::Decryptor for ViginereCipher {
    /// function designed to implement a viginere cipher.
    ///
    /// this will take in plaintext and a key and rotate each letter
    /// in the plaintext using the associated key character.
    fn decrypt(&self, ciphertext: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // let mut plaintext: Vec<u8> = vec![];
        let mut key_pos: usize = 0;
        let len_key = self.key.len();
        let rev: bool = true;

        if !is_valid_key(self.key.clone()) {
            return Err("invalid key passed in".into());
        }

        // go through each letter in the ciphertext and rotate it
        // by the inverse key value to convert it back to the
        // original plaintext value and generate the final plaintext vector.
        let plaintext: Vec<u8> = ciphertext
            .iter()
            .enumerate()
            .map(|(_, &current)| {
                let new: u8 = rotate(current, self.key[key_pos % len_key], rev);

                if is_letter(current) {
                    key_pos += 1;
                }

                return new;
            })
            .collect();

        return Ok(plaintext);
    }
}

impl crate::Encryptor for ViginereCipher {
    /// function designed to implement a viginere cipher.
    ///
    /// this will take in plaintext and a key and rotate each letter
    /// in the plaintext using the associated key character.
    fn encrypt(&self, plaintext: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut key_pos: usize = 0;
        let len_key = self.key.len();
        let rev: bool = false;

        if !is_valid_key(self.key.clone()) {
            return Err("invalid key passed in".into());
        }

        // go through each letter in the plaintext and rotate it
        // by the corresponding key value to generate the final
        // ciphertext vector.
        let ciphertext: Vec<u8> = plaintext
            .iter()
            .enumerate()
            .map(|(_, &current)| {
                let new = rotate(current, self.key[key_pos % len_key], rev);

                if is_letter(current) {
                    key_pos += 1;
                }

                return new;
            })
            .collect();

        return Ok(ciphertext);
    }
}

/// function designed to adjust a key's case based on the
/// case of the letter.
///
/// this will bring the key to the letter's case.
///
/// examples (l,k):
///
/// ```plaintext
/// a, C => a, c (c + adjustment)
/// A, c => A, C (c - adjustment)
/// a, c => a, c (no adjustment)
/// ```
fn adjust_key(letter: u8, key: u8) -> u8 {
    if !is_letter(letter) {
        return key;
    }

    if is_upper(letter) && is_lower(key) {
        key - LOWER_TO_UPPER_DIFF
    } else if is_lower(letter) && is_upper(key) {
        key + LOWER_TO_UPPER_DIFF
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

/// helper function designed to determine if a letter is lowercase.
fn is_lower(c: u8) -> bool {
    match c {
        LOWER_A..LOWER_Z => true,
        _ => false,
    }
}

/// helper function designed to determine if a letter is uppercase.
fn is_upper(c: u8) -> bool {
    match c {
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
fn rotate(letter: u8, key: u8, rev: bool) -> u8 {
    let mut corrected_key: u8;
    let corrected_letter: u8 = correct_char(letter);
    let rotated: u8;

    corrected_key = adjust_key(corrected_letter, correct_char(key));

    // use the inverse of the key if "rev" flag is specified.
    // this is used when rotating for decryption.
    if rev {
        corrected_key = (ALPHABET_LEN - corrected_key) % ALPHABET_LEN;
    }

    match letter {
        LOWER_A..LOWER_Z => rotated = ((corrected_letter + corrected_key) % ALPHABET_LEN) + LOWER_A,
        UPPER_A..UPPER_Z => rotated = ((corrected_letter + corrected_key) % ALPHABET_LEN) + UPPER_A,
        _ => rotated = letter,
    };

    rotated
}

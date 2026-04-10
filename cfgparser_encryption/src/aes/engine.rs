#[derive(Debug)]
pub enum AESError {
    KeyLength(String),
}

const ERR_KEY_LEN: &str = "invalid key length. must be 16, 24 or 32 bytes";

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AESCipher {
    key: Vec<u8>,
}

/// function designed to check whether a given AES key is valid.
fn validate_key(key: Vec<u8>) -> Result<(), AESError> {
    let valid_lengths: Vec<usize> = vec![16, 24, 32];
    let key_size: usize = key.len();

    if !valid_lengths.contains(&key_size) {
        return Err(AESError::KeyLength(ERR_KEY_LEN.to_string()));
    }

    Ok(())
}

impl AESCipher {
    pub fn new(key: Vec<u8>) -> Result<AESCipher, AESError> {
        validate_key(key.clone())?;
        Ok(AESCipher { key })
    }
}

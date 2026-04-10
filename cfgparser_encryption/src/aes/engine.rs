#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AESCipher {
    key: Vec<u8>,
}

/// function designed to check whether a given AES key is valid.
fn validate_key(key: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

impl AESCipher {
    pub fn new(key: Vec<u8>) -> Result<AESCipher, Box<dyn std::error::Error>> {
        validate_key(key.clone())?;
        Ok(AESCipher { key })
    }
}

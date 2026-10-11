#[cfg(test)]
mod unit_tests;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
/// struct designed to implement Decrypt and Encrypt for an XOR cipher.
pub struct XORCipher {
    key: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct XORCipherBuilder<'build_life> {
    key: Option<&'build_life [u8]>,
}

impl<'build_life> XORCipherBuilder<'build_life> {
    pub fn new() -> Self {
        XORCipherBuilder { key: None }
    }

    /// set the key the XOR cipher will use for encryption/decryption.
    pub fn with_key(mut self, key: &'build_life [u8]) -> Self {
        self.key = Some(key);
        self
    }

    pub fn build(self) -> XORCipher {
        let key: &[u8] = match self.key {
            Some(key) => key,
            None => &[],
        };

        XORCipher::new(key.to_vec())
    }
}

impl XORCipher {
    pub fn new(key: Vec<u8>) -> XORCipher {
        XORCipher { key }
    }
}

impl crate::Decryptor for XORCipher {
    /// implementation of the Decryptor trait for XORCipher.
    ///
    /// this will call the encrypt_decrypt function with the
    /// passed in ciphertext and the XORCipher's key.
    fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(encrypt_decrypt(self.key.clone(), ciphertext))
    }
}

impl crate::Encryptor for XORCipher {
    /// implementation of the Encryptor trait for XORCipher.
    ///
    /// this will call the encrypt_decrypt function with the
    /// passed in plaintext and the XORCipher's key.
    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(encrypt_decrypt(self.key.clone(), plaintext))
    }
}

/// function designed to XOR encrypt/decrypt data using a given key.
pub fn encrypt_decrypt(key: Vec<u8>, data: &[u8]) -> Vec<u8> {
    let key_len: usize = key.len();

    // if no key given, there is nothing to do to the
    // data, so return it as-is.
    if key_len < 1 {
        return data.to_vec();
    }

    // loop over the data and XOR each item with the corresponding
    // key item.
    //
    // this is the encrypt/decrypt step.
    let result: Vec<u8> = data
        .iter()
        .enumerate()
        .map(|(idx, &current)| current ^ key[idx % key_len])
        .collect();

    return result;
}

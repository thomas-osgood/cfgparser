#[cfg(test)]
mod unit_tests;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
/// struct designed to implement Decrypt and Encrypt for an XOR cipher.
pub struct XORCipher {
    pub key: Vec<u8>,
}

impl XORCipher {
    pub fn new(key: Vec<u8>) -> XORCipher {
        XORCipher { key }
    }
}

impl crate::Decryptor for XORCipher {
    fn decrypt(&self, ciphertext: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(encrypt_decrypt(self.key.clone(), ciphertext))
    }
}

impl crate::Encryptor for XORCipher {
    fn encrypt(&self, plaintext: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        Ok(encrypt_decrypt(self.key.clone(), plaintext))
    }
}

/// function designed to XOR encrypt/decrypt data using a given key.
pub fn encrypt_decrypt(key: Vec<u8>, data: Vec<u8>) -> Vec<u8> {
    let key_len: usize = key.len();

    // if no key given, there is nothing to do to the
    // data, so return it as-is.
    if key_len < 1 {
        return data;
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

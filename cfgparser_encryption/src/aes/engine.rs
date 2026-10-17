use aes_gcm::aead::{Aead, AeadCore, KeyInit};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::CipherResult;

#[cfg(test)]
mod unit_tests;

#[derive(Debug, PartialEq, Eq)]
pub enum AESError {
    EmptyKey,
    Generic(String),
    KeyLength(String),
}

impl std::fmt::Display for AESError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AESError::EmptyKey => write!(f, "{}", ERR_NO_KEY),
            AESError::Generic(s) => write!(f, "{}", s),
            AESError::KeyLength(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for AESError {}

const ERR_KEY_LEN: &str = "invalid key length. must be 16, 24 or 32 bytes";
const ERR_NO_KEY: &str = "no key specified";
const ERR_192: &str = "AES-192 is not currently supported";

const KEYLEN_128: usize = 16;
const KEYLEN_192: usize = 24;
const KEYLEN_256: usize = 32;

/// expected size of the nonce when decrypting.
pub const NONCE_SIZE: usize = 12;

#[derive(Clone, Debug, Default, PartialEq, Eq, Zeroize, ZeroizeOnDrop)]
pub struct AESCipher {
    key: Vec<u8>,
}

pub struct AESCipherBuilder<'build_life> {
    key: Option<&'build_life [u8]>,
}

/// function designed to check whether a given AES key is valid.
fn validate_key(key: &[u8]) -> Result<(), AESError> {
    let valid_lengths: &[usize] = &[KEYLEN_128, KEYLEN_192, KEYLEN_256];
    let key_size: usize = key.len();

    if !valid_lengths.contains(&key_size) {
        return Err(AESError::KeyLength(ERR_KEY_LEN.to_string()));
    }

    match key_size {
        KEYLEN_192 => Err(AESError::KeyLength(ERR_192.to_string())),
        _ => Ok(()),
    }
}

impl<'build_life> AESCipherBuilder<'build_life> {
    pub fn new() -> Self {
        Self { key: None }
    }

    /// set the encryption key for the cipher.
    pub fn with_key(mut self, key: &'build_life [u8]) -> Self {
        self.key = Some(key);
        self
    }

    /// validate the cipher options, then build and return
    /// an AESCipher.
    pub fn build(self) -> CipherResult<AESCipher, AESError> {
        let key: &[u8] = match self.key {
            Some(key) => key,
            None => return Err(AESError::EmptyKey),
        };

        AESCipher::new(key.to_vec())
    }
}

impl AESCipher {
    pub fn new(key: Vec<u8>) -> CipherResult<AESCipher, AESError> {
        validate_key(&key)?;
        Ok(AESCipher { key })
    }

    /// function designed to update the key used for encryption
    /// and decryption by the AESCipher.
    pub fn update_key(&mut self, key: Vec<u8>) -> CipherResult<(), AESError> {
        validate_key(&key)?;
        // zero out the old key data before update for
        // security best practices.
        self.key.zeroize();
        self.key = key;
        Ok(())
    }
}

impl crate::Decryptor for AESCipher {
    type Error = AESError;

    fn decrypt(&self, ciphertext: &[u8]) -> CipherResult<Vec<u8>, Self::Error> {
        // split the encrypted data into nonce and cipherbytes. this
        // will extract the nonce that will be used to decrypt the data
        // and the cipherbytes that will be decrypted.
        //
        // bytes 0 -> 11: nonce
        // bytes 12 -> n: cipherbytes
        let (nonce_arr, cipherbytes) = ciphertext.split_at(NONCE_SIZE);

        // rebuild the nonce using the bytes extracted from the
        // encrypted data above.
        let nonce = aes_gcm::Nonce::from_slice(nonce_arr);

        let decrypt_result = match self.key.len() {
            KEYLEN_128 => {
                let key: &aes_gcm::Key<aes_gcm::Aes128Gcm> =
                    aes_gcm::Key::<aes_gcm::Aes128Gcm>::from_slice(&self.key);
                let aescipher = aes_gcm::Aes128Gcm::new(key);
                aescipher.decrypt(nonce, cipherbytes)
            }
            KEYLEN_256 => {
                let key: &aes_gcm::Key<aes_gcm::Aes256Gcm> =
                    aes_gcm::Key::<aes_gcm::Aes256Gcm>::from_slice(&self.key);
                let aescipher = aes_gcm::Aes256Gcm::new(key);
                aescipher.decrypt(nonce, cipherbytes)
            }
            _ => return Err(AESError::KeyLength(ERR_KEY_LEN.to_string())),
        };

        match decrypt_result {
            Ok(plaintext) => Ok(plaintext),
            Err(e) => Err(AESError::Generic(e.to_string())),
        }
    }
}

impl crate::Encryptor for AESCipher {
    type Error = AESError;

    fn encrypt(&self, plaintext: &[u8]) -> CipherResult<Vec<u8>, Self::Error> {
        let mut ciphertext: Vec<u8>;
        let encrypt_result = match self.key.len() {
            KEYLEN_128 => {
                let key: &aes_gcm::Key<aes_gcm::Aes128Gcm> =
                    aes_gcm::Key::<aes_gcm::Aes128Gcm>::from_slice(&self.key);
                // generate a random, 12 byte nonce to use during encryption.
                let nonce = aes_gcm::Aes128Gcm::generate_nonce(&mut aes_gcm::aead::OsRng);
                // create a new AES cipher object using the encryption key.
                let aescipher = aes_gcm::Aes128Gcm::new(key);

                ciphertext = nonce.to_vec();
                aescipher.encrypt(&nonce, plaintext)
            }
            KEYLEN_256 => {
                let key: &aes_gcm::Key<aes_gcm::Aes256Gcm> =
                    aes_gcm::Key::<aes_gcm::Aes256Gcm>::from_slice(&self.key);
                // generate a random, 12 byte nonce to use during encryption.
                let nonce = aes_gcm::Aes256Gcm::generate_nonce(&mut aes_gcm::aead::OsRng);
                // create a new AES cipher object using the encryption key.
                let aescipher = aes_gcm::Aes256Gcm::new(key);

                ciphertext = nonce.to_vec();
                aescipher.encrypt(&nonce, plaintext)
            }
            _ => return Err(AESError::KeyLength(ERR_KEY_LEN.to_string())),
        };

        match encrypt_result {
            Ok(enc_result) => {
                // return a slice with the first 12 bytes being the
                // nonce and the remaining bytes being the encrypted
                // message. this allows the receiver to extract the
                // nonce from the message during decryption.
                ciphertext.extend_from_slice(&enc_result);
                Ok(ciphertext)
            }
            Err(e) => Err(AESError::Generic(e.to_string())),
        }
    }
}

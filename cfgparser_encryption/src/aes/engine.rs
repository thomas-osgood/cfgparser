use aes_gcm::aead::{Aead, AeadCore, KeyInit};

#[derive(Debug)]
pub enum AESError {
    KeyLength(String),
}

impl ToString for AESError {
    fn to_string(&self) -> String {
        match self {
            AESError::KeyLength(s) => s.to_string(),
        }
    }
}

const ERR_KEY_LEN: &str = "invalid key length. must be 16, 24 or 32 bytes";
const ERR_128: &str = "AES-128 is not currently supported";
const ERR_192: &str = "AES-192 is not currently supported";

/// expected size of the nonce when decrypting.
pub const NONCE_SIZE: usize = 12;

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

    match key_size {
        16 => Err(AESError::KeyLength(ERR_128.to_string())),
        24 => Err(AESError::KeyLength(ERR_192.to_string())),
        _ => Ok(()),
    }
}

impl AESCipher {
    pub fn new(key: Vec<u8>) -> Result<AESCipher, AESError> {
        validate_key(key.clone())?;
        Ok(AESCipher { key })
    }
}

impl crate::Decryptor for AESCipher {
    fn decrypt(&self, ciphertext: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let key: &aes_gcm::Key<aes_gcm::Aes256Gcm> =
            aes_gcm::Key::<aes_gcm::Aes256Gcm>::from_slice(&self.key);

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

        let aescipher = aes_gcm::Aes256Gcm::new(key);

        match aescipher.decrypt(nonce, cipherbytes) {
            Ok(plaintext) => Ok(plaintext),
            Err(e) => Err(e.to_string().into()),
        }
    }
}

impl crate::Encryptor for AESCipher {
    fn encrypt(&self, plaintext: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let key: &aes_gcm::Key<aes_gcm::Aes256Gcm> =
            aes_gcm::Key::<aes_gcm::Aes256Gcm>::from_slice(&self.key);

        // generate a random, 12 byte nonce to use during encryption.
        let nonce = aes_gcm::Aes256Gcm::generate_nonce(&mut aes_gcm::aead::OsRng);

        // create a new AES cipher object using the encryption key.
        let aescipher = aes_gcm::Aes256Gcm::new(key);

        match aescipher.encrypt(&nonce, &plaintext[..]) {
            Ok(enc_result) => {
                // return a slice with the first 12 bytes being the
                // nonce and the remaining bytes being the encrypted
                // message. this allows the receiver to extract the
                // nonce from the message during decryption.
                let mut ciphertext: Vec<u8> = nonce.to_vec();
                ciphertext.extend_from_slice(&enc_result);
                Ok(ciphertext)
            }
            Err(e) => Err(e.to_string().into()),
        }
    }
}

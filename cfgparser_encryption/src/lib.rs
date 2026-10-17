//! this is a library that contains simple encryption methods and functions.
//!
//! this is designed to be used by cfgparser_core for its main functionality
//! and obfuscation.

#[cfg(feature = "aes")]
pub mod aes;
pub mod viginere;
pub mod xor;

pub type CipherResult<T, E = DynamicCipherError> = Result<T, E>;

pub enum DynamicCipher {
    AES(aes::engine::AESCipher),
    XOR(xor::engine::XORCipher),
    Viginere(viginere::engine::ViginereCipher),
}

#[derive(Debug)]
pub enum DynamicCipherError {
    AES(aes::engine::AESError),
    Viginere(viginere::engine::ViginerCipherError),
    XOR(xor::engine::XORCipherError),
    Generic(String),
}

impl std::fmt::Display for DynamicCipherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DynamicCipherError::AES(e) => write!(f, "AES Error: {}", e),
            DynamicCipherError::Viginere(e) => write!(f, "Viginere Error: {}", e),
            DynamicCipherError::XOR(e) => write!(f, "XOR Error: {}", e),
            DynamicCipherError::Generic(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for DynamicCipherError {}

impl From<aes::engine::AESError> for DynamicCipherError {
    fn from(err: aes::engine::AESError) -> Self {
        DynamicCipherError::AES(err)
    }
}

impl From<viginere::engine::ViginerCipherError> for DynamicCipherError {
    fn from(err: viginere::engine::ViginerCipherError) -> Self {
        DynamicCipherError::Viginere(err)
    }
}

impl From<xor::engine::XORCipherError> for DynamicCipherError {
    fn from(err: xor::engine::XORCipherError) -> Self {
        DynamicCipherError::XOR(err)
    }
}

#[derive(Default, Debug)]
/// enum defining the various forms of encryption that are supported
/// by this library.
pub enum EncryptionType {
    #[default]
    Xor,
    Viginere,
    Aes,
}

#[derive(Debug)]
pub enum EncryptionTypeError {
    UnknownType,
}

impl std::fmt::Display for EncryptionTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncryptionTypeError::UnknownType => write!(f, "unknown encryption type"),
        }
    }
}

impl std::error::Error for EncryptionTypeError {}

impl TryFrom<i32> for EncryptionType {
    type Error = EncryptionTypeError;

    fn try_from(value: i32) -> CipherResult<Self, Self::Error> {
        match value {
            aes if aes == EncryptionType::Aes as i32 => Ok(EncryptionType::Aes),
            vig if vig == EncryptionType::Viginere as i32 => Ok(EncryptionType::Viginere),
            xor if xor == EncryptionType::Xor as i32 => Ok(EncryptionType::Xor),
            _ => Err(EncryptionTypeError::UnknownType),
        }
    }
}

/// generic trait designed to describe a structure that can decrypt
/// ciphertext during a configuration extraction.
pub trait Decryptor {
    type Error: std::error::Error;

    /// function designed to take in a ciphertext vector, manipulate
    /// it and return a plaintext vector.
    ///
    /// this function's logic will differ between the various implementations
    /// of the trait, but the overall process (ciphertext in, plaintext out)
    /// should be consistent.
    fn decrypt(&self, ciphertext: &[u8]) -> CipherResult<Vec<u8>, Self::Error>;
}

/// generic trait designed to describe a structure that can decrypt
/// ciphertext during a configuration extraction.
pub trait Encryptor {
    type Error: std::error::Error;

    /// function designed to take in a plaintext vector, manipulate
    /// it and return a ciphertext vector.
    ///
    /// this function's logic will differ between the various implementations
    /// of the trait, but the overall process (plaintext in, ciphertext out)
    /// should be consistent.
    fn encrypt(&self, plaintext: &[u8]) -> CipherResult<Vec<u8>, Self::Error>;
}

impl Decryptor for DynamicCipher {
    type Error = DynamicCipherError;

    fn decrypt(&self, ciphertext: &[u8]) -> CipherResult<Vec<u8>> {
        match self {
            DynamicCipher::AES(cipher) => Ok(cipher.decrypt(ciphertext)?),
            DynamicCipher::Viginere(cipher) => Ok(cipher.decrypt(ciphertext)?),
            DynamicCipher::XOR(cipher) => Ok(cipher.decrypt(ciphertext)?),
        }
    }
}

//! this is a library that contains simple encryption methods and functions.
//!
//! this is designed to be used by cfgparser_core for its main functionality
//! and obfuscation.

#[cfg(feature = "aes")]
pub mod aes;
pub mod viginere;
pub mod xor;

#[derive(Default, Debug)]
/// enum defining the various forms of encryption that are supported
/// by this library.
pub enum EncryptionType {
    #[default]
    Xor,
    Viginere,
    Aes,
}

/// generic trait designed to describe a structure that can decrypt
/// ciphertext during a configuration extraction.
pub trait Decryptor {
    /// function designed to take in a ciphertext vector, manipulate
    /// it and return a plaintext vector.
    ///
    /// this function's logic will differ between the various implementations
    /// of the trait, but the overall process (ciphertext in, plaintext out)
    /// should be consistent.
    fn decrypt(&self, ciphertext: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}

/// generic trait designed to describe a structure that can decrypt
/// ciphertext during a configuration extraction.
pub trait Encryptor {
    /// function designed to take in a plaintext vector, manipulate
    /// it and return a ciphertext vector.
    ///
    /// this function's logic will differ between the various implementations
    /// of the trait, but the overall process (plaintext in, ciphertext out)
    /// should be consistent.
    fn encrypt(&self, plaintext: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>>;
}

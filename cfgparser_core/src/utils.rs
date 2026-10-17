pub mod encryption;
pub mod ffi;

#[derive(Debug)]
pub enum UtilsError {
    DecryptorCreation(String),
    InvalidDecryptorType(String),
}

impl From<cfgparser_encryption::aes::engine::AESError> for UtilsError {
    fn from(err: cfgparser_encryption::aes::engine::AESError) -> Self {
        UtilsError::DecryptorCreation(err.to_string())
    }
}

impl From<cfgparser_encryption::EncryptionTypeError> for UtilsError {
    fn from(err: cfgparser_encryption::EncryptionTypeError) -> Self {
        UtilsError::InvalidDecryptorType(err.to_string())
    }
}

impl From<cfgparser_encryption::viginere::engine::ViginerCipherError> for UtilsError {
    fn from(err: cfgparser_encryption::viginere::engine::ViginerCipherError) -> Self {
        UtilsError::DecryptorCreation(err.to_string())
    }
}

impl From<cfgparser_encryption::xor::engine::XORCipherError> for UtilsError {
    fn from(err: cfgparser_encryption::xor::engine::XORCipherError) -> Self {
        UtilsError::DecryptorCreation(err.to_string())
    }
}

impl From<cfgparser_encryption::DynamicCipherError> for UtilsError {
    fn from(err: cfgparser_encryption::DynamicCipherError) -> Self {
        UtilsError::DecryptorCreation(err.to_string())
    }
}

impl std::fmt::Display for UtilsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UtilsError::DecryptorCreation(msg) => write!(f, "Decryptor Creation Error: {}", msg),
            UtilsError::InvalidDecryptorType(msg) => write!(f, "Invalid Decryptor Type: {}", msg),
        }
    }
}

impl std::error::Error for UtilsError {}

pub type UtilsResult<T> = Result<T, UtilsError>;

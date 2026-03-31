//! this is a library that contains simple encryption methods and functions.
//!
//! this is designed to be used by cfgparser_core for its main functionality
//! and obfuscation.

pub mod viginere;
pub mod xor;

#[derive(Debug)]
pub enum EncryptionType {
    Xor,
    Viginere,
}

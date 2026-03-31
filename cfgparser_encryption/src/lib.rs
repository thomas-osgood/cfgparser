//! this is a library that contains simple encryption methods and functions.
//!
//! this is designed to be used by cfgparser_core for its main functionality
//! and obfuscation.

pub mod viginere;
pub mod xor;

#[derive(Debug)]
/// enum defining the various forms of encryption that are supported
/// by this library.
pub enum EncryptionType {
    Xor,
    Viginere,
}

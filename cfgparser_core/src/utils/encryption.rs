use super::*;

#[cfg(test)]
mod unit_tests;

/// helper function designed to build and return a decryptor
/// based on a key and encryption type passed in.
pub fn get_decryptor(
    key: &[u8],
    enc_type_i32: i32,
) -> UtilsResult<cfgparser_encryption::DynamicCipher> {
    // transform the i32 encryption type definition into a cfgparser_encryption
    // encryption type to use for the match statement.
    let enc_type: cfgparser_encryption::EncryptionType =
        cfgparser_encryption::EncryptionType::try_from(enc_type_i32)?;
    // create the decryptor based on the user's input.
    Ok(cfgparser_encryption::DynamicCipher::new(enc_type, key)?)
}

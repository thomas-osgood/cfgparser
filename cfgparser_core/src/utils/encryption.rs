/// helper function designed to build and return a decryptor
/// based on a key and encryption type passed in.
pub fn get_decryptor(
    key: &[u8],
    enc_type_i32: i32,
) -> Result<cfgparser_encryption::DynamicCipher, Box<dyn std::error::Error>> {
    // transform the i32 encryption type definition into a cfgparser_encryption
    // encryption type to use for the match statement.
    let enc_type: cfgparser_encryption::EncryptionType =
        cfgparser_encryption::EncryptionType::try_from(enc_type_i32)?;

    // create the decryptor based on the user's input. this will compare
    // the int with the enum to determine what kind of decryptor to create.
    //
    // if an invalid int is passed in, or an error occurs, an error is returned.
    let decryptor: cfgparser_encryption::DynamicCipher = match enc_type {
        cfgparser_encryption::EncryptionType::Xor => {
            let cipher: cfgparser_encryption::xor::engine::XORCipher =
                cfgparser_encryption::xor::engine::XORCipher::new(key.to_vec())?;
            cfgparser_encryption::DynamicCipher::XOR(cipher)
        }
        cfgparser_encryption::EncryptionType::Viginere => {
            let cipher: cfgparser_encryption::viginere::engine::ViginereCipher =
                cfgparser_encryption::viginere::engine::ViginereCipher::new(key.to_vec())?;
            cfgparser_encryption::DynamicCipher::Viginere(cipher)
        }
        cfgparser_encryption::EncryptionType::Aes => {
            let cipher: cfgparser_encryption::aes::engine::AESCipher =
                cfgparser_encryption::aes::engine::AESCipher::new(key.to_vec())?;
            cfgparser_encryption::DynamicCipher::AES(cipher)
        }
    };

    Ok(decryptor)
}

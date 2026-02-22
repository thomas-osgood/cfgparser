use base64::{self, Engine};
use cfgparser_encryption;

#[no_mangle]
pub extern "C" fn read_cfg() {
    let key: &[u8] = "test".as_bytes();
    let msg = base64::engine::general_purpose::STANDARD_NO_PAD.encode("hello");

    let ciphertext: Vec<u8> =
        cfgparser_encryption::xor::engine::encrypt_decrypt(key.to_vec(), msg.into());

    match transform_payload(key, &ciphertext) {
        Ok(decoded) => println!("DECODED: {:#}", String::from_utf8_lossy(&decoded)),
        Err(e) => println!("ERROR DECODING: {:#}", e),
    }
}

/// function designed to convert the raw bytes read from
/// the end of the file into a configuration.
///
/// process:
///
/// 1. decrypt bytes
/// 2. base64-decode plaintext
fn transform_payload(key: &[u8], raw_payload: &[u8]) -> Result<Vec<u8>, base64::DecodeError> {
    // XOR decrypt the raw_payload bytes.
    let plaintext: Vec<u8> =
        cfgparser_encryption::xor::engine::encrypt_decrypt(key.to_vec(), raw_payload.to_vec());

    // base64-decode the payload and save the result
    base64::engine::general_purpose::STANDARD_NO_PAD.decode(plaintext)
}

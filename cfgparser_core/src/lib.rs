use base64::{self, Engine};
use cfgparser_encryption;

mod models;

#[no_mangle]
pub extern "C" fn read_cfg() {
    let key: &[u8] = "test".as_bytes();
    let msg = base64::engine::general_purpose::STANDARD_NO_PAD
        .encode("{\"host\": \"localhost\", \"port\": 80}");

    let ciphertext: Vec<u8> =
        cfgparser_encryption::xor::engine::encrypt_decrypt(key.to_vec(), msg.into());

    let decoded: String;
    match transform_payload(key, &ciphertext) {
        Ok(result) => decoded = String::from_utf8_lossy(&result).to_string(),
        Err(e) => {
            println!("ERROR DECODING: {:#}", e);
            return;
        }
    }

    println!("DECODED: {:#}", decoded);

    let configuration: models::core::Configuration = match deserialize_payload(decoded) {
        Ok(result) => result,
        Err(e) => {
            println!("ERROR DESERIALIZING JSON: {:#}", e);
            return;
        }
    };

    println!("CONFIGURATION: {:#?}", configuration);
}

/// function designed to JSON deserialize a given string into
/// a Configuration struct and return the struct.
///
/// if there is an error, a `serde_json::Error` will be returned.
fn deserialize_payload(payload: String) -> Result<models::core::Configuration, serde_json::Error> {
    match serde_json::from_str(&payload) {
        Ok(result) => Ok(result),
        Err(e) => Err(e),
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

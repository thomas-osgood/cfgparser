use base64::{self, Engine};
use cfgparser_encryption;

mod models;
mod transformer;

#[no_mangle]
/// function designed to read the configuration bytes and
/// return the C2 address. this will read the data from the
/// end of the binary, XOR decrypt it, Base64 decode,
/// JSON decode it, then grab the address and port from the
/// Configuration struct that resulted from the JSON decoding.
pub extern "C" fn read_cfg() {
    let key: &[u8] = "test".as_bytes();
    let msg = base64::engine::general_purpose::STANDARD_NO_PAD
        .encode("{\"host\": \"localhost\", \"port\": 80}");

    let ciphertext: Vec<u8> =
        cfgparser_encryption::xor::engine::encrypt_decrypt(key.to_vec(), msg.into());

    let decoded: String;
    match transformer::core::transform_payload(key, &ciphertext) {
        Ok(result) => decoded = String::from_utf8_lossy(&result).to_string(),
        Err(e) => {
            println!("ERROR DECODING: {:#}", e);
            return;
        }
    }

    println!("DECODED: {:#}", decoded);

    let configuration: models::core::Configuration =
        match transformer::core::deserialize_payload(decoded) {
            Ok(result) => result,
            Err(e) => {
                println!("ERROR DESERIALIZING JSON: {:#}", e);
                return;
            }
        };

    println!("CONFIGURATION: {:#?}", configuration);

    let address: String = format!("{}:{}", configuration.host, configuration.port);
    println!("ADDRESS: {:#}", address);
}

/// function designed to read the end of the current binary
/// and extract the configuration bytes. these bytes will then
/// be processed in a later step to transform them into a
/// Configuration struct to allow access to all necessary information.
fn read_cfg_bytes() {}

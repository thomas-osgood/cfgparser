#[cfg(test)]
mod unit_tests;

/// function designed to XOR encrypt/decrypt data using a given key.
pub fn encrypt_decrypt(key: Vec<u8>, data: Vec<u8>) -> Vec<u8> {
    let key_len: usize = key.len();

    // if no key given, there is nothing to do to the
    // data, so return it as-is.
    if key_len < 1 {
        return data;
    }

    // loop over the data and XOR each item with the corresponding
    // key item.
    //
    // this is the encrypt/decrypt loop.
    let result: Vec<u8> = data
        .iter()
        .enumerate()
        .map(|(idx, &current)| current ^ key[idx % key_len])
        .collect();

    return result;
}

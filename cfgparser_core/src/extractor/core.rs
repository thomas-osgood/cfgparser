use std::io::{Read, Seek};

/// function designed to extract the configuration
/// bytes from the current binary.
pub fn extract_cfg_bytes() -> std::io::Result<Vec<u8>> {
    let current_binary: std::path::PathBuf = std::env::current_exe()?;
    let mut fptr: std::fs::File = std::fs::File::open(current_binary)?;

    // allocate buffer that will hold the size bytes.
    let mut buf_sz: [u8; 8] = [0; 8];
    // jumpt to the start of the size bytes (end - 8 bytes).
    let _: u64 = fptr.seek(std::io::SeekFrom::End(-8))?;

    fptr.read_exact(&mut buf_sz)?;

    // convert the bytes read into i64.
    //
    // TODO: check to make sure it should be i64 and not u64.
    let sz_payload: i64 = i64::from_be_bytes(buf_sz);

    // calculate how far back to go from the end of the file
    // to reach the beginning byte of the config.
    let cfg_offset: i64 = (8 + sz_payload) * -1;

    // jump to the start of the config.
    fptr.seek(std::io::SeekFrom::End(cfg_offset))?;

    // allocate buffer for payload bytes.
    let mut payload_buf: Vec<u8> = vec![0u8; sz_payload as usize];

    // read the payload bytes into the payload buffer.
    fptr.read_exact(&mut payload_buf)?;

    Ok(payload_buf)
}

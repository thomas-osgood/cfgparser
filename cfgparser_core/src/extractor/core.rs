use std::io::{Read, Seek};

/// constant size of the byte buffer that will be used
/// to hold the bytes representing the size of the
/// configuration array.
const SZ_SIZEBUFF: usize = 8;

/// generic trait defining a CfgExtractor. this is defined
/// so mocking can be done during testing.
///
/// the main implementation of this is in SelfExtractor.
pub trait CfgExtractor {
    fn extract_cfg_bytes(&self) -> std::io::Result<Vec<u8>>;
}

/// struct designed to extract configuration bytes from the
/// current binary. this implements the CfgExtractor trait.
pub struct SelfExtractor;
impl CfgExtractor for SelfExtractor {
    /// function designed to extract the configuration
    /// bytes from the current binary.
    fn extract_cfg_bytes(&self) -> std::io::Result<Vec<u8>> {
        let current_binary: std::path::PathBuf = std::env::current_exe()?;
        let mut fptr: std::fs::File = std::fs::File::open(current_binary)?;
        let size_start: i64 = SZ_SIZEBUFF as i64 * -1;

        // allocate buffer that will hold the size bytes.
        let mut buf_sz: [u8; SZ_SIZEBUFF as usize] = [0; SZ_SIZEBUFF];
        // jumpt to the start of the size bytes (end - 8 bytes).
        let _: u64 = fptr.seek(std::io::SeekFrom::End(size_start))?;

        fptr.read_exact(&mut buf_sz)?;

        // convert the bytes read into i64.
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
}

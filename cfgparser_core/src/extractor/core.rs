#[cfg(test)]
mod unit_tests;

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

/// generic function designed to read the configuration bytes from
/// a file and return either a Vec<u8> or an error.
fn read_bytes<R>(mut reader: R) -> std::io::Result<Vec<u8>>
where
    R: std::io::Read + std::io::Seek,
{
    let size_start: i64 = SZ_SIZEBUFF as i64 * -1;

    // allocate buffer that will hold the size bytes.
    let mut buf_sz: [u8; SZ_SIZEBUFF as usize] = [0; SZ_SIZEBUFF];
    // jumpt to the start of the size bytes (end - 8 bytes).
    let _: u64 = reader.seek(std::io::SeekFrom::End(size_start))?;

    reader.read_exact(&mut buf_sz)?;

    // convert the bytes read into i64.
    let sz_payload: i64 = i64::from_be_bytes(buf_sz);

    // calculate how far back to go from the end of the file
    // to reach the beginning byte of the config.
    let cfg_offset: i64 = (8 + sz_payload) * -1;

    // jump to the start of the config.
    reader.seek(std::io::SeekFrom::End(cfg_offset))?;

    // allocate buffer for payload bytes.
    let mut payload_buf: Vec<u8> = vec![0u8; sz_payload as usize];

    // read the payload bytes into the payload buffer.
    reader.read_exact(&mut payload_buf)?;

    Ok(payload_buf)
}

/// struct designed to extract configuration bytes from the
/// current binary. this implements the CfgExtractor trait.
pub struct SelfExtractor;
impl CfgExtractor for SelfExtractor {
    /// function designed to extract the configuration
    /// bytes from the current binary.
    fn extract_cfg_bytes(&self) -> std::io::Result<Vec<u8>> {
        let current_binary: std::path::PathBuf = std::env::current_exe()?;
        let fptr: std::fs::File = std::fs::File::open(current_binary)?;
        // read and return the  bytes from the file.
        read_bytes(fptr)
    }
}

#[derive(Debug, Default)]
/// struct designed to extract configuration bytes from a
/// given file.
pub struct FileExtractor {
    pub filename: String,
}

impl FileExtractor {
    pub fn new(filename: String) -> FileExtractor {
        FileExtractor { filename }
    }
}

impl CfgExtractor for FileExtractor {
    fn extract_cfg_bytes(&self) -> std::io::Result<Vec<u8>> {
        let fptr: std::fs::File = std::fs::File::open(&self.filename)?;
        read_bytes(fptr)
    }
}

#[cfg(test)]
/// extractor struct meant to be used in unit tests only.
pub struct TestExtractor;

#[cfg(test)]
/// implementation of the CfgExtractor for the TestExtractor.
impl CfgExtractor for TestExtractor {
    /// dummy extract_cfg_bytes function that will be used by
    /// the TestExtractor. this function is designed for testing
    /// only and is meant to simulate a configuration that has
    /// been json-encoded, base64-encoded and xor-encrypted.
    ///
    /// config: {"host": "secrethost", "port": 8000}
    /// key: "secret"
    fn extract_cfg_bytes(&self) -> std::io::Result<Vec<u8>> {
        Ok(vec![
            22, 28, 41, 29, 7, 71, 61, 85, 42, 24, 10, 19, 58, 11, 45, 30, 60, 71, 57, 9, 7, 53,
            13, 2, 16, 86, 50, 27, 41, 55, 50, 12, 0, 53, 92, 13, 23, 38, 42, 68, 44, 48, 20, 18,
            46, 54, 39, 77,
        ])
    }
}

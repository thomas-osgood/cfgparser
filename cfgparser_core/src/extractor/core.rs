#[cfg(test)]
mod unit_tests;

const ERR_INVALID_PAYLOAD_SIZE: &str = "sz_payload must be >= 0";
const ERR_NO_PAYLOAD: &str = "no payload specified for extractor";
const ERR_OFFSET_TOO_LARGE: &str = "the offset exceeds the maximum allowed size for this reader";

/// Maximum allowable configuration payload size (e.g., 10 MB)
const MAX_PAYLOAD_SIZE: usize = 10 * 1024 * 1024;

/// constant representing a value of no offset.
///
/// this will inidicate to read from the end of
/// the file/stream/etc.
pub const OFFSET_NONE: usize = 0;

/// constant size of the byte buffer that will be used
/// to hold the bytes representing the size of the
/// configuration array.
const SZ_SIZEBUFF: usize = 8;

/// generic trait defining a CfgExtractor. this is defined
/// so mocking can be done during testing.
///
/// the main implementation of this is in SelfExtractor.
pub trait CfgExtractor {
    fn extract_cfg_bytes(&self, offset: usize) -> std::io::Result<Vec<u8>>;
    fn len(&self) -> std::io::Result<u64>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CfgExtractorError {
    InvalidPayloadSize,
    NoPayload,
    OffsetTooLarge,
    PayloadTooSmall,
    Unknown(String),
}

impl std::fmt::Display for CfgExtractorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CfgExtractorError::InvalidPayloadSize => write!(f, "{}", ERR_INVALID_PAYLOAD_SIZE),
            CfgExtractorError::NoPayload => write!(f, "{}", ERR_NO_PAYLOAD),
            CfgExtractorError::OffsetTooLarge => write!(f, "{}", ERR_OFFSET_TOO_LARGE),
            CfgExtractorError::PayloadTooSmall => {
                write!(f, "payload must be at least {} bytes", SZ_SIZEBUFF)
            }
            CfgExtractorError::Unknown(msg) => write!(f, "Unknown Extraction Error: {}", msg),
        }
    }
}

impl std::error::Error for CfgExtractorError {}

impl CfgExtractorError {
    /// Helper method to cleanly map errors to their corresponding io::ErrorKind
    pub fn kind(&self) -> std::io::ErrorKind {
        match self {
            CfgExtractorError::OffsetTooLarge
            | CfgExtractorError::InvalidPayloadSize
            | CfgExtractorError::PayloadTooSmall => std::io::ErrorKind::InvalidData,
            CfgExtractorError::NoPayload | CfgExtractorError::Unknown(_) => {
                std::io::ErrorKind::Other
            }
        }
    }
}

impl From<CfgExtractorError> for std::io::Error {
    fn from(err: CfgExtractorError) -> Self {
        std::io::Error::new(err.kind(), err)
    }
}

/// helper function designed to safely calculate the
/// starting position of the cfg block.
fn calc_cfg_block_start(
    buf_sz: [u8; SZ_SIZEBUFF],
    offset: usize,
    reader_size: u64,
) -> Result<i64, std::io::Error> {
    // calculate the payload size
    let sz_payload: usize = calc_sz_payload(buf_sz)?;

    let cfg_offset_abs: usize = SZ_SIZEBUFF
        .checked_add(sz_payload)
        .and_then(|m| m.checked_add(offset))
        .ok_or_else(|| CfgExtractorError::OffsetTooLarge)?;

    if cfg_offset_abs as u64 > reader_size {
        return Err(CfgExtractorError::OffsetTooLarge.into());
    }

    // calculate how far back to go from the end of the file
    // to reach the beginning byte of the config.
    Ok(-i64::try_from(cfg_offset_abs).map_err(|_| CfgExtractorError::OffsetTooLarge)?)
}

/// helper function designed to safely calculate the starting
/// position of the size block.
fn calc_size_block_start(offset: usize) -> Result<i64, std::io::Error> {
    // Safely calculate the size bytes starting position using checked math
    let combined_offset: usize = SZ_SIZEBUFF
        .checked_add(offset)
        .ok_or_else(|| CfgExtractorError::OffsetTooLarge)?;

    Ok(-i64::try_from(combined_offset).map_err(|_| CfgExtractorError::OffsetTooLarge)?)
}

/// helper function designed to carry out converting the payload
/// size from raw bytes to usize safely.
fn calc_sz_payload(buf_sz: [u8; SZ_SIZEBUFF]) -> Result<usize, std::io::Error> {
    // convert the bytes read into i64.
    let sz_payload: i64 = i64::from_be_bytes(buf_sz);

    // validate the sz_payload size is valid for a usize conversion.
    if sz_payload < 0 {
        return Err(CfgExtractorError::InvalidPayloadSize.into());
    }

    let sz_payload_usize: usize =
        usize::try_from(sz_payload).map_err(|_| CfgExtractorError::InvalidPayloadSize)?;

    // confirm the payload size is not greater than the
    // max payload size defined above.
    if sz_payload_usize > MAX_PAYLOAD_SIZE {
        return Err(CfgExtractorError::InvalidPayloadSize.into());
    }

    Ok(sz_payload_usize)
}

/// function designed to determine the size (in byte) of a reader.
///
/// this seeks to the end of the reader, saves the location, then
/// seeks back to the very beginning.
fn get_reader_size<R>(mut reader: R) -> std::io::Result<u64>
where
    R: std::io::Seek,
{
    let file_size: u64 = reader.seek(std::io::SeekFrom::End(0))?;
    let _ = reader.seek(std::io::SeekFrom::Start(0))?;
    Ok(file_size)
}

/// generic function designed to read the configuration bytes from
/// a reader that implements the Read + Seek traits and return
/// either a Vec<u8> or an error.
///
/// note: the `offset` is the offset from the end of the `reader` where
/// the *end* of the size block is. to calculate the beginning of the
/// size block, this function will add `SZ_SIZEBUFF` to the offset.
fn read_bytes<R>(mut reader: R, offset: usize) -> std::io::Result<Vec<u8>>
where
    R: std::io::Read + std::io::Seek,
{
    let reader_size: u64 = get_reader_size(&mut reader)?;

    validate_offset(reader_size, offset)?;

    // safely read the size bytes.
    let buf_sz: [u8; SZ_SIZEBUFF] = read_size_bytes(&mut reader, offset)?;

    read_payload_bytes(reader, buf_sz, reader_size, offset)
}

/// function designed to safely read and return the main
/// payload bytes.
fn read_payload_bytes<R>(
    mut reader: R,
    buf_sz: [u8; SZ_SIZEBUFF],
    reader_size: u64,
    offset: usize,
) -> Result<Vec<u8>, std::io::Error>
where
    R: std::io::Read + std::io::Seek,
{
    // calculate the payload size
    let sz_payload: usize = calc_sz_payload(buf_sz)?;
    // calculate how far back to go from the end of the file
    // to reach the beginning byte of the config.
    let cfg_offset: i64 = calc_cfg_block_start(buf_sz, offset, reader_size)?;
    // jump to the start of the config.
    reader.seek(std::io::SeekFrom::End(cfg_offset))?;
    // allocate buffer for payload bytes.
    let mut payload_buf: Vec<u8> = vec![0u8; sz_payload as usize];
    // read the payload bytes into the payload buffer.
    reader.read_exact(&mut payload_buf)?;

    Ok(payload_buf)
}

/// helper function designed to safely read the block of bytes
/// that hold the size of the payload.
fn read_size_bytes<R>(mut reader: R, offset: usize) -> Result<[u8; SZ_SIZEBUFF], std::io::Error>
where
    R: std::io::Read + std::io::Seek,
{
    // Safely calculate the size bytes starting position
    let size_start: i64 = calc_size_block_start(offset)?;
    // allocate buffer that will hold the size bytes.
    let mut buf_sz: [u8; SZ_SIZEBUFF] = [0; SZ_SIZEBUFF];
    // jumpt to the start of the size bytes (end - 8 bytes).
    let _: u64 = reader.seek(std::io::SeekFrom::End(size_start))?;
    // read the bytes into the buffer
    reader.read_exact(&mut buf_sz)?;

    Ok(buf_sz)
}

/// helper function designed to insure the offset does not
/// cause an overflow/out of bounds read when extracting
/// the config from the reader's data.
fn validate_offset(reader_size: u64, offset: usize) -> Result<(), std::io::Error> {
    let total_size: usize = SZ_SIZEBUFF
        .checked_add(offset)
        .ok_or_else(|| CfgExtractorError::OffsetTooLarge)?;
    if total_size < SZ_SIZEBUFF {
        return Err(CfgExtractorError::PayloadTooSmall.into());
    }
    if reader_size <= total_size as u64 {
        return Err(CfgExtractorError::OffsetTooLarge.into());
    }
    Ok(())
}

/// struct designed to extract configuration bytes from the
/// current binary. this implements the CfgExtractor trait.
#[derive(Debug, Clone, Copy, Default)]
pub struct SelfExtractor;
impl CfgExtractor for SelfExtractor {
    /// function designed to extract the configuration
    /// bytes from the current binary.
    fn extract_cfg_bytes(&self, offset: usize) -> std::io::Result<Vec<u8>> {
        let current_binary: std::path::PathBuf = std::env::current_exe()?;
        let fptr: std::fs::File = std::fs::File::open(current_binary)?;
        // read and return the  bytes from the file.
        read_bytes(fptr, offset)
    }

    /// function designed to return the size (in bytes)
    /// of the current binary.
    fn len(&self) -> std::io::Result<u64> {
        let current_binary: std::path::PathBuf = std::env::current_exe()?;
        let metadata: std::fs::Metadata = std::fs::metadata(current_binary)?;
        Ok(metadata.len())
    }
}

#[derive(Debug, Default)]
/// struct designed to extract configuration bytes from a
/// given file.
pub struct FileExtractor {
    path: std::path::PathBuf,
}

#[derive(Debug, Default)]
pub struct FileExtractorBuilder {
    path: Option<std::path::PathBuf>,
}

impl FileExtractorBuilder {
    pub fn new() -> Self {
        Self { path: None }
    }

    pub fn with_path<P: Into<std::path::PathBuf>>(mut self, path: P) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn build(self) -> Result<FileExtractor, CfgExtractorError> {
        let path: std::path::PathBuf = self.path.ok_or(CfgExtractorError::NoPayload)?;
        Ok(FileExtractor { path })
    }
}

impl FileExtractor {
    pub fn new<P: AsRef<std::path::Path>>(path: P) -> Self {
        FileExtractor {
            path: path.as_ref().into(),
        }
    }
}

impl CfgExtractor for FileExtractor {
    fn extract_cfg_bytes(&self, offset: usize) -> std::io::Result<Vec<u8>> {
        let fptr: std::fs::File = std::fs::File::open(&self.path)?;
        read_bytes(fptr, offset)
    }

    fn len(&self) -> std::io::Result<u64> {
        let metadata: std::fs::Metadata = std::fs::metadata(&self.path)?;
        Ok(metadata.len())
    }
}

#[derive(Debug, Default)]
/// struct designed to extract configuration bytes from
/// a byte vector (`Vec<u8>`).
pub struct BytesExtractor {
    stream: Vec<u8>,
}

pub struct BytesExtractorBuilder {
    stream: Option<Vec<u8>>,
}

impl BytesExtractorBuilder {
    pub fn new() -> Self {
        Self { stream: None }
    }

    pub fn with_stream<T: Into<Vec<u8>>>(mut self, stream: T) -> Self {
        self.stream = Some(stream.into());
        self
    }

    pub fn build(self) -> Result<BytesExtractor, CfgExtractorError> {
        let stream: Vec<u8> = self.stream.ok_or(CfgExtractorError::NoPayload)?;
        Ok(BytesExtractor { stream })
    }
}

impl BytesExtractor {
    pub fn new(stream: Vec<u8>) -> BytesExtractor {
        BytesExtractor { stream }
    }

    /// function designed to return the data being held
    /// in the extractor's stream vector.
    pub fn get_stream(&self) -> &[u8] {
        &self.stream
    }

    /// function designed to overwrite the ByteExtractor's
    /// stream with a new stream.
    pub fn set_stream(&mut self, stream: Vec<u8>) {
        self.stream = stream;
    }
}

impl CfgExtractor for BytesExtractor {
    fn extract_cfg_bytes(&self, offset: usize) -> std::io::Result<Vec<u8>> {
        let stream: std::io::Cursor<&[u8]> = std::io::Cursor::new(&self.stream);
        read_bytes(stream, offset)
    }

    fn len(&self) -> std::io::Result<u64> {
        let len: u64 = match self.stream.len().try_into() {
            Ok(len) => len,
            Err(e) => return Err(CfgExtractorError::Unknown(e.to_string()).into()),
        };

        Ok(len)
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
    fn extract_cfg_bytes(&self, _offset: usize) -> std::io::Result<Vec<u8>> {
        Ok(vec![
            22, 28, 41, 29, 7, 71, 61, 85, 42, 24, 10, 19, 58, 11, 45, 30, 60, 71, 57, 9, 7, 53,
            13, 2, 16, 86, 50, 27, 41, 55, 50, 12, 0, 53, 92, 13, 23, 38, 42, 68, 44, 48, 20, 18,
            46, 54, 39, 77,
        ])
    }

    fn len(&self) -> std::io::Result<u64> {
        Ok(48)
    }
}

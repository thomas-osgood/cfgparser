mod extractor_utils;
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
    let reader_size: u64 = extractor_utils::read_utils::get_reader_size(&mut reader)?;

    extractor_utils::validations::validate_offset(reader_size, offset)?;

    // safely read the size bytes.
    let buf_sz: [u8; SZ_SIZEBUFF] =
        extractor_utils::read_utils::read_size_bytes(&mut reader, offset)?;

    extractor_utils::read_utils::read_payload_bytes(reader, buf_sz, reader_size, offset)
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

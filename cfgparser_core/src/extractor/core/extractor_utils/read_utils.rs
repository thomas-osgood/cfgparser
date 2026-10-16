use super::*;

#[cfg(test)]
mod unit_tests;

/// function designed to determine the size (in byte) of a reader.
///
/// this seeks to the end of the reader, saves the location, then
/// seeks back to the very beginning.
pub fn get_reader_size<R>(mut reader: R) -> std::io::Result<u64>
where
    R: std::io::Seek,
{
    let file_size: u64 = reader.seek(std::io::SeekFrom::End(0))?;
    let _ = reader.seek(std::io::SeekFrom::Start(0))?;
    Ok(file_size)
}

/// function designed to safely read and return the main
/// payload bytes.
pub fn read_payload_bytes<R>(
    mut reader: R,
    buf_sz: [u8; SZ_SIZEBUFF],
    reader_size: u64,
    offset: usize,
) -> Result<Vec<u8>, std::io::Error>
where
    R: std::io::Read + std::io::Seek,
{
    // calculate the payload size
    let sz_payload: usize = calc_utils::calc_sz_payload(buf_sz)?;
    // calculate how far back to go from the end of the file
    // to reach the beginning byte of the config.
    let cfg_offset: i64 = calc_utils::calc_cfg_block_start(buf_sz, offset, reader_size)?;
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
pub fn read_size_bytes<R>(mut reader: R, offset: usize) -> Result<[u8; SZ_SIZEBUFF], std::io::Error>
where
    R: std::io::Read + std::io::Seek,
{
    // Safely calculate the size bytes starting position
    let size_start: i64 = calc_utils::calc_size_block_start(offset)?;
    // allocate buffer that will hold the size bytes.
    let mut buf_sz: [u8; SZ_SIZEBUFF] = [0; SZ_SIZEBUFF];
    // jumpt to the start of the size bytes (end - 8 bytes).
    let _: u64 = reader.seek(std::io::SeekFrom::End(size_start))?;
    // read the bytes into the buffer
    reader.read_exact(&mut buf_sz)?;

    Ok(buf_sz)
}

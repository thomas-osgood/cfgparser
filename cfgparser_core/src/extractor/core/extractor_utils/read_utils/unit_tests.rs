
use super::*;
use std::io::Cursor;

#[test]
fn test_get_reader_size() {
    let data = vec![0u8; 64];
    let cursor = Cursor::new(data);
    let size = get_reader_size(cursor).unwrap();
    assert_eq!(size, 64);
}

#[test]
fn test_read_size_bytes() {
    // Create a 16-byte stream where the last 8 bytes represent a payload size of 5
    let mut data = vec![0u8; 8];
    let mut size_bytes = 5_i64.to_be_bytes().to_vec();
    data.append(&mut size_bytes);

    let cursor = Cursor::new(data);
    let sz_buf = read_size_bytes(cursor, 0).unwrap();
    assert_eq!(i64::from_be_bytes(sz_buf), 5);
}

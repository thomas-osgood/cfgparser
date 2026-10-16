use super::*;

#[test]
fn test_calc_sz_payload_valid() {
    // 100 bytes in big-endian
    let buf = 100_i64.to_be_bytes();
    let result = calc_sz_payload(buf);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 100);
}

#[test]
fn test_calc_sz_payload_negative() {
    let buf = (-1_i64).to_be_bytes();
    let result = calc_sz_payload(buf);
    assert!(result.is_err());
}

#[test]
fn test_calc_sz_payload_too_large() {
    // Exceeds MAX_PAYLOAD_SIZE (10 MB)
    let too_large: i64 = 11 * 1024 * 1024;
    let result = calc_sz_payload(too_large.to_be_bytes());
    assert!(result.is_err());
}

#[test]
fn test_calc_size_block_start() {
    let result = calc_size_block_start(0);
    assert!(result.is_ok());
    // SZ_SIZEBUFF is 8, so offset 0 yields -8
    assert_eq!(result.unwrap(), -8);
}

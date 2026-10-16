
use super::*;

#[test]
fn test_validate_offset_success() {
    // reader_size = 100, offset = 0, SZ_SIZEBUFF = 8
    // total_size = 8. difference = 100 - 8 = 92 (which is >= 1)
    let result = validate_offset(100, 0);
    assert!(result.is_ok());
}

#[test]
fn test_validate_offset_too_large() {
    // reader_size = 5, total_size = 8 -> difference < 1
    let result = validate_offset(5, 0);
    assert!(result.is_err());
}

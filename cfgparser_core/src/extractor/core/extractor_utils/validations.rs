use super::*;

#[cfg(test)]
mod unit_tests;

/// helper function designed to insure the offset does not
/// cause an overflow/out of bounds read when extracting
/// the config from the reader's data.
pub fn validate_offset(reader_size: u64, offset: usize) -> Result<(), std::io::Error> {
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

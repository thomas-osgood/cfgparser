use super::*;

#[cfg(test)]
mod unit_tests;

/// helper function designed to safely calculate the
/// starting position of the cfg block.
pub fn calc_cfg_block_start(
    buf_sz: [u8; SZ_SIZEBUFF],
    offset: usize,
    reader_size: u64,
) -> std::io::Result<i64> {
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
pub fn calc_size_block_start(offset: usize) -> std::io::Result<i64> {
    // Safely calculate the size bytes starting position using checked math
    let combined_offset: usize = SZ_SIZEBUFF
        .checked_add(offset)
        .ok_or_else(|| CfgExtractorError::OffsetTooLarge)?;

    Ok(-i64::try_from(combined_offset).map_err(|_| CfgExtractorError::OffsetTooLarge)?)
}

/// helper function designed to carry out converting the payload
/// size from raw bytes to usize safely.
pub fn calc_sz_payload(buf_sz: [u8; SZ_SIZEBUFF]) -> std::io::Result<usize> {
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

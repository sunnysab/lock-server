#[inline]
#[allow(dead_code)]
pub fn bytes_to_u32(bytes: &[u8; 4]) -> u32 {
    return ((bytes[0] as u32) << 24)
        + ((bytes[1] as u32) << 16)
        + ((bytes[2] as u32) << 8)
        + bytes[3] as u32;
}

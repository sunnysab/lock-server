#[inline]
#[allow(dead_code)]
pub fn bytes_to_u32(bytes: &[u8]) -> u32 {
    if bytes.len() != 4 {
        panic!("bytes_to_u32 must accept an slice with 4 elements.");
    }

    return ((bytes[0] as u32) << 24)
        + ((bytes[1] as u32) << 16)
        + ((bytes[2] as u32) << 8)
        + bytes[3] as u32;
}

#[inline]
#[allow(dead_code)]
pub fn u32_to_byte(num: u32) -> Vec<u8> {
    vec![
        (num >> 24) as u8,
        (num >> 16 & 0xFF) as u8,
        (num >> 8 & 0xFF) as u8,
        (num & 0xFF) as u8,
    ]
}

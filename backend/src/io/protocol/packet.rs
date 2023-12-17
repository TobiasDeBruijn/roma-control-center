pub fn encode_u32(v: u32) -> [u8; 4] {
    [
        ((v >> 24) & 0xFF) as u8,
        ((v >> 16) & 0xFF) as u8,
        ((v >> 8) & 0xFF) as u8,
        (v & 0xFF) as u8,
    ]
}

pub fn encode_u16(v: u16) -> [u8; 2] {
    [((v >> 8) & 0xFF) as u8, (v & 0xFF) as u8]
}

pub fn decode_u32(v: &[u8]) -> u32 {
    v[3] as u32 | (v[2] as u32) << 8 | (v[1] as u32) << 16 | (v[0] as u32) << 24
}

pub fn decode_u16(v: &[u8]) -> u16 {
    v[1] as u16 | (v[0] as u16) << 8
}

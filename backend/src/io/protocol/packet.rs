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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn en_u32() {
        assert_eq!(encode_u32(0), [0_u8, 0, 0, 0]);
        assert_eq!(encode_u32(0x3B9ACAEC), [0x3B_u8, 0x9A, 0xCA, 0xEC]);
        assert_eq!(encode_u32(u32::MAX), [0xFF, 0xFF, 0xFF, 0xFF]);
    }

    #[test]
    fn en_u16() {
        assert_eq!(encode_u16(0), [0_u8, 0]);
        assert_eq!(encode_u16(0x3B9A), [0x3B_u8, 0x9A]);
        assert_eq!(encode_u16(u16::MAX), [0xFF, 0xFF]);
    }

    #[test]
    fn de_u32() {
        assert_eq!(decode_u32(&[0, 0, 0, 0]), 0_u32);
        assert_eq!(decode_u32(&[0x3B, 0x9A, 0xCA, 0xEC]), 0x3B9ACAEC);
        assert_eq!(decode_u32(&[0xFF, 0xFF, 0xFF, 0xFF]), u32::MAX);
    }

    #[test]
    fn de_u16() {
        assert_eq!(decode_u16(&[0, 0]), 0_u16);
        assert_eq!(decode_u16(&[0x3B, 0x9A]), 0x3B9A);
        assert_eq!(decode_u16(&[0xFF, 0xFF]), u16::MAX);
    }
}
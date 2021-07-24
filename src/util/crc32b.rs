use crc::{Crc, CRC_32_BZIP2};

pub fn hash(val: u128) -> u16 {
    let crc = Crc::<u32>::new(&CRC_32_BZIP2);
    crc.checksum(&val.to_le_bytes()) as u16 & 0x3ff
}

fn remove_hash(val: u128) -> u128 {
    val & 0xfff6007fffffffffffffffffffffffff
}

pub fn add_hash(val: u128, hash: u16) -> u128 {
    val | ((hash as u128) << 103)
}

pub fn get_hash(val: u128) -> u16 {
    ((val >> 103) as u16) & 0x3ff
}

pub fn check_hash(val: u128) -> bool {
    hash(remove_hash(val)) == get_hash(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY2: u128 = u128::from_le_bytes([
        0xf6, 0x06, 0x00, 0x00, 0x00, 0x00, 0x78, 0xd6,
        0x9d, 0xdc, 0xfa, 0x86, 0x83, 0x26, 0x01, 0x00
    ]);
    const KEY3: u128 = u128::from_le_bytes([
        0xf6, 0x06, 0x00, 0x00, 0x00, 0x00, 0x78, 0xd6,
        0x9d, 0xdc, 0xfa, 0x86, 0x03, 0x00, 0x00, 0x00
    ]);
    const HASH: u16 = 0x024d;
    const CHECKSUM: u16 = 0x024d;

    #[test]
    fn test_hash() {
        assert_eq!(hash(KEY3), HASH);
    }

    #[test]
    fn test_remove_hash() {
        assert_eq!(remove_hash(KEY2), KEY3);
    }

    #[test]
    fn test_add_hash() {
        assert_eq!(add_hash(KEY3, CHECKSUM), KEY2);
    }

    #[test]
    fn test_get_hash() {
        assert_eq!(get_hash(KEY2), CHECKSUM);
    }

    #[test]
    fn test_check_hash() {
        assert!(check_hash(KEY2));
    }
}

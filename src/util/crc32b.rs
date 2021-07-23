use crc::{Crc, CRC_32_BZIP2};

pub fn hash(val: u128) -> u16 {
    let crc = Crc::<u32>::new(&CRC_32_BZIP2);
    crc.checksum(&val.to_le_bytes()) as u16 & 0x3ff
}

fn remove_hash(val: u128) -> u128 {
    val & !(0x7ff as u128)
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
        0xff, 0x42, 0x9d, 0x00, 0xc4, 0xd6, 0x1f, 0xfc,
        0x2f, 0x1f, 0x2e, 0x52, 0xa2, 0x95, 0x00, 0x00
    ]);
    const KEY3: u128 = u128::from_le_bytes([
        0xff, 0x42, 0x9d, 0x00, 0xc4, 0xd6, 0x1f, 0xfc,
        0x2f, 0x1f, 0x2e, 0x52, 0x22, 0x00, 0x00, 0x00
    ]);
    const HASH: u16 = 0x008a;
    const CHECKSUM: u16 = 0x012b;

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
        assert_eq!(check_hash(KEY2), false);
    }
}

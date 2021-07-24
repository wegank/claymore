use crc::{Crc, CRC_32_BZIP2};

pub fn hash(val: u128) -> u16 {
    let crc = Crc::<u32>::new(&CRC_32_BZIP2);
    crc.checksum(&val.to_le_bytes()) as u16 & 0x3ff
}

fn remove_checksum(val: u128) -> u128 {
    val & 0xfff6007fffffffffffffffffffffffff
}

fn get_checksum(val: u128) -> u16 {
    ((val >> 103) as u16) & 0x3ff
}

pub fn check(val: u128) -> bool {
    hash(remove_checksum(val)) == get_checksum(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY_VAL: u128 = u128::from_le_bytes([
        0xf6, 0x06, 0x00, 0x00, 0x00, 0x00, 0x78, 0xd6,
        0x9d, 0xdc, 0xfa, 0x86, 0x83, 0x26, 0x01, 0x00
    ]);
    const KEY_VAL_NO_CRC: u128 = u128::from_le_bytes([
        0xf6, 0x06, 0x00, 0x00, 0x00, 0x00, 0x78, 0xd6,
        0x9d, 0xdc, 0xfa, 0x86, 0x03, 0x00, 0x00, 0x00
    ]);
    const CHECKSUM: u16 = 0x24d;

    #[test]
    fn test_hash() {
        assert_eq!(hash(KEY_VAL_NO_CRC), CHECKSUM);
    }

    #[test]
    fn test_remove_checksum() {
        assert_eq!(remove_checksum(KEY_VAL), KEY_VAL_NO_CRC);
    }

    #[test]
    fn test_get_checksum() {
        assert_eq!(get_checksum(KEY_VAL), CHECKSUM);
    }

    #[test]
    fn test_check() {
        assert!(check(KEY_VAL));
    }
}

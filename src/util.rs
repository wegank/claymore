mod base24;
mod crc32b;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct KeyInfo {
    group_id: u32,
    key_id: u32,
    secret: u64,
    hash: u16
}

impl KeyInfo {
    pub fn load(key: &String) -> KeyInfo {
        let val = base24::decode(key);
        KeyInfo {
            group_id: (val & 0xfffff) as u32,
            key_id: ((val >> 20) & 0x3fffffff) as u32,
            secret: ((val >> 50) & 0x1fffffffffffff) as u64,
            hash: ((val >> 103) as u16) & 0x3ff,
        }
    }

    fn serialize(&self) -> u128 {
        self.group_id as u128
            | (self.key_id as u128) << 20
            | (self.secret as u128) << 50
            | (self.hash as u128) << 103
    }

    pub fn check(&self) -> bool {
        crc32b::check_hash(self.serialize())
    }

    pub fn hash(&self) -> KeyInfo {
        let mut key_info = self.clone();
        key_info.hash = 0;
        key_info.hash = crc32b::hash(key_info.serialize());
        key_info
    }

    pub fn save(&self) -> String {
        base24::encode(self.serialize())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: &str = "JWNY3-GG8KP-GBFXR-P8HHH-67PK9";
    const KEY_VAL: u128 = u128::from_le_bytes([
        0xff, 0x42, 0x9d, 0x00, 0xc4, 0xd6, 0x1f, 0xfc,
        0x2f, 0x1f, 0x2e, 0x52, 0xa2, 0x95, 0x00, 0x00
    ]);
    const KEY_INFO: KeyInfo = KeyInfo {
        group_id: 0x000d42ff,
        key_id: 0x3d6c4009,
        secret: 0x0008948b87cbff07,
        hash: 0x012b,
    };
    const HASH: u16 = 0x008a;

    #[test]
    fn test_load() {
        assert_eq!(KeyInfo::load(&KEY.to_string()), KEY_INFO);
    }

    #[test]
    fn test_serialize() {
        assert_eq!(KEY_INFO.serialize(), KEY_VAL);
    }

    #[test]
    fn test_check() {
        assert_eq!(KEY_INFO.check(), false);
    }

    #[test]
    fn test_hash() {
        let mut key_info = KEY_INFO;
        key_info.hash = HASH;
        assert_eq!(KEY_INFO.hash(), key_info);
    }

    #[test]
    fn test_save() {
        assert_eq!(KEY_INFO.save(), KEY.to_string());
    }
}

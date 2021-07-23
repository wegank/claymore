const BASE24_STR: &str = "BCDFGHJKMPQRTVWXY2346789";

fn main() {
    println!("Hello, world!");
}

fn to_bytes(key: &String) -> [u8; 25] {
    let mut key: Vec<char> = key.chars().collect();
    if key.len() != 29 {
        panic!("Your key must be 29 characters long.");
    } else if vec![5, 11, 17, 23].iter().any(|&i| key[i] != '-') {
        panic!("Incorrect hyphens.")
    } else if key.iter().filter(|&&i| i == 'N').count() > 1 {
        panic!("There may only be one N in a key.")
    } else if key.iter().filter(|&&i| i == 'N').count() == 0 {
        panic!("The character N must be in the product key.")
    } else if key[28] == 'N' {
        panic!("The last character must not be an N.")
    }

    let mut bytes: [u8; 25] = [0; 25];

    key.retain(|&c| c != '-');
    match key.iter().position(|&c| c == 'N') {
        Some(pos) => bytes[0] = pos as u8,
        _ => panic!("Invalid character in key.")
    }

    key.retain(|&c| c != 'N');
    for i in 1..25 {
        match BASE24_STR.find(key[i - 1]) {
            Some(pos) => bytes[i] = pos as u8,
            _ => panic!("Invalid character in key.")
        }
    }

    bytes
}

fn decode(bytes: &[u8; 25]) -> u128 {
    let mut le_bytes: [u8; 16] = [0; 16];
    for i in 0..25 {
        let mut c = bytes[i] as u16;
        for j in 0..16 {
            c += (le_bytes[j] as u16) * 24;
            le_bytes[j] = c as u8;
            c >>= 8;
        }
    }
    u128::from_le_bytes(le_bytes)
}

fn get_hash_table() -> [u32; 256] {
    let mut table: [u32; 256] = [0; 256];
    for i in 0..256 {
        let mut k: u32 = (i as u32) << 24;
        for _ in 0..8 {
            k = match k & 0x80000000 {
                0 => k << 1,
                _ => (k << 1) ^ 0x04c11db7
            }
        }
        table[i] = k;
    }
    table
}

fn remove_hash(val: u128) -> u128 {
    val & 0xfff6007fffffffffffffffffffffffff
}

fn calculate_hash(val: u128) -> u32 {
    let hash_table = get_hash_table();
    let le_bytes: [u8; 16] = u128::to_le_bytes(remove_hash(val));
    let mut hash: u32 = 0xffffffff;
    for i in 0..16 {
        let index = (((hash >> 24) as u8) ^ le_bytes[i]) & 0xff;
        hash = (hash << 8) ^ hash_table[index as usize];
    }
    !hash & 0x3ff
}

fn get_hash(val: u128) -> u32 {
    ((val >> 103) as u32) & 0x3ff
}

fn check_hash(val: u128) -> bool {
    calculate_hash(val) == get_hash(val)
}

fn get_group_id(val: u128) -> u32 {
    (val & 0xfffff) as u32
}

fn get_key_id(val: u128) -> u32 {
    ((val >> 20) & 0x3fffffff) as u32
}

fn get_secret(val: u128) -> u64 {
    ((val >> 50) & 0x1fffffffffffff) as u64
}


#[cfg(test)]
mod tests {
    use super::*;

    const KEY0: &str = "JWNY3-GG8KP-GBFXR-P8HHH-67PK9";
    const KEY1: [u8; 25] = [
        0x02, 0x06, 0x0e, 0x10, 0x12,
        0x04, 0x04, 0x16, 0x07, 0x09,
        0x04, 0x00, 0x03, 0x0f, 0x0b,
        0x09, 0x16, 0x05, 0x05, 0x05,
        0x14, 0x15, 0x09, 0x07, 0x17
    ];
    const KEY2: [u8; 16] = [
        0xff, 0x42, 0x9d, 0x00, 0xc4, 0xd6, 0x1f, 0xfc,
        0x2f, 0x1f, 0x2e, 0x52, 0xa2, 0x95, 0x00, 0x00
    ];
    const KEY3: [u8; 16] = [
        0xff, 0x42, 0x9d, 0x00, 0xc4, 0xd6, 0x1f, 0xfc,
        0x2f, 0x1f, 0x2e, 0x52, 0x22, 0x00, 0x00, 0x00
    ];
    const GROUP_ID: u32 = 0x000d42ff;
    const KEY_ID: u32 = 0x3d6c4009;
    const SECRET: u64 = 0x0008948b87cbff07;
    const HASH: u32 = 0x0000008a;
    const CHECKSUM: u32 = 0x0000012b;

    #[test]
    fn test_to_bytes() {
        assert_eq!(to_bytes(&KEY0.to_string()), KEY1);
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode(&KEY1).to_le_bytes(), KEY2);
    }

    #[test]
    fn test_remove_hash() {
        assert_eq!(remove_hash(u128::from_le_bytes(KEY2)).to_le_bytes(), KEY3);
    }

    #[test]
    fn test_calculate_hash() {
        assert_eq!(calculate_hash(u128::from_le_bytes(KEY2)), HASH);
    }

    #[test]
    fn test_get_hash() {
        assert_eq!(get_hash(u128::from_le_bytes(KEY2)), CHECKSUM);
    }

    #[test]
    fn test_check_hash() {
        assert_eq!(check_hash(u128::from_le_bytes(KEY2)), false);
    }

    #[test]
    fn test_get_group_id() {
        assert_eq!(get_group_id(u128::from_le_bytes(KEY3)), GROUP_ID);
    }

    #[test]
    fn test_get_key_id() {
        assert_eq!(get_key_id(u128::from_le_bytes(KEY3)), KEY_ID);
    }

    #[test]
    fn test_get_secret() {
        assert_eq!(get_secret(u128::from_le_bytes(KEY3)), SECRET);
    }
}

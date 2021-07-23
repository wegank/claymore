const BASE24_STR: &str = "BCDFGHJKMPQRTVWXY2346789";

pub fn decode(key: &String) -> u128 {
    b24decode(&to_bytes(key))
}

pub fn encode(val: u128) -> String {
    from_bytes(&b24encode(val))
}

fn b24decode(bytes: &[u8; 25]) -> u128 {
    bytes.iter().rev().enumerate()
        .map(|(i, &k)| (k as u128) * u128::pow(24, i as u32))
        .sum()
}

fn b24encode(val: u128) -> [u8; 25] {
    let mut val = val;
    let mut bytes: [u8; 25] = [0; 25];
    for i in (0..25).rev() {
        bytes[i] = (val % 24) as u8;
        val = val / 24;
    }
    bytes
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
        _ => panic!("Invalid character in key."),
    }

    key.retain(|&c| c != 'N');
    for i in 1..25 {
        match BASE24_STR.find(key[i - 1]) {
            Some(pos) => bytes[i] = pos as u8,
            _ => panic!("Invalid character in key."),
        }
    }

    bytes
}

fn from_bytes(bytes: &[u8; 25]) -> String {
    let mut key: String = bytes[1..25].iter()
        .map(|&i| BASE24_STR.as_bytes()[i as usize] as char).collect();
    key.insert(bytes[0] as usize, 'N');
    for &i in [5, 11, 17, 23].iter() {
        key.insert(i, '-');
    }
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    const KEY: &str = "JWNY3-GG8KP-GBFXR-P8HHH-67PK9";
    const KEY1: [u8; 25] = [
        0x02, 0x06, 0x0e, 0x10, 0x12,
        0x04, 0x04, 0x16, 0x07, 0x09,
        0x04, 0x00, 0x03, 0x0f, 0x0b,
        0x09, 0x16, 0x05, 0x05, 0x05,
        0x14, 0x15, 0x09, 0x07, 0x17
    ];
    const KEY2: u128 = u128::from_le_bytes([
        0xff, 0x42, 0x9d, 0x00, 0xc4, 0xd6, 0x1f, 0xfc,
        0x2f, 0x1f, 0x2e, 0x52, 0xa2, 0x95, 0x00, 0x00
    ]);

    #[test]
    fn test_decode() {
        assert_eq!(decode(&KEY.to_string()), KEY2);
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode(KEY2), KEY.to_string());
    }

    #[test]
    fn test_b24decode() {
        assert_eq!(b24decode(&KEY1), KEY2);
    }

    #[test]
    fn test_b24encode() {
        assert_eq!(b24encode(KEY2), KEY1);
    }

    #[test]
    fn test_to_bytes() {
        assert_eq!(to_bytes(&KEY.to_string()), KEY1);
    }

    #[test]
    fn test_from_bytes() {
        assert_eq!(from_bytes(&KEY1), KEY.to_string());
    }
}

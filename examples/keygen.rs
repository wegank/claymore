use claymore::KeyInfo;

fn main() {
    let mut key_info = KeyInfo {
        group_id: 1785,
        serial_number: 0,
        fst_security_value: 0,
        snd_security_value: 0,
        checksum: 0,
        upgrade_bit: 0
    };
    println!("{}", keygen(&mut key_info));
}

fn keygen(key_info: &mut KeyInfo) -> String {
    key_info.serial_number = rand::random::<u32>() % 53501100;
    key_info.fst_security_value = rand::random::<u64>() & 0xffffffffff;
    key_info.snd_security_value = rand::random::<u32>() & 0x1fff;
    key_info.hash().save()
}

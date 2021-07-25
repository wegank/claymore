use claymore::KeyInfo;
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let key_info = KeyInfo {
        group_id: 1785,
        serial_number: rng.gen_range(0..=53501099),
        fst_security_value: rng.gen_range(0..=0xffffffffff),
        snd_security_value: rng.gen_range(0..=0x1fff),
        checksum: 0,
        upgrade_bit: 0
    };
    let key = key_info.hash().save();
    println!("{}", key);
}

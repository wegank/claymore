use claymore::{KeyInfo, PKeyInfo, VKeyInfo};

const KEY: &str = "RR3BN-3YY9P-9D7FC-7J4YF-QGJXW";
const PATH: &str = "examples/resources/pkeyconfig-win8.xrm-ms";

fn main() {
    let key_info = KeyInfo::load(KEY).unwrap();
    let pkey_config = PKeyInfo::load_from_file(PATH).unwrap();
    assert!(key_info.is_valid());
    assert!(pkey_config.is_valid(key_info.group_id, key_info.serial_number));

    let pkey_info = pkey_config.query(key_info.group_id, key_info.serial_number, key_info.upgrade_bit).unwrap();
    println!("{:#?}", pkey_info);

    let pid = &pkey_info.extended_pid;
    let vkey_info = VKeyInfo::load_one(pid).unwrap();
    assert!(!vkey_info.is_valid(pid));
}

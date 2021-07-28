use claymore::{KeyInfo, PKeyInfo};
// use claymore::VKeyInfo;

const KEY: &str = "RR3BN-3YY9P-9D7FC-7J4YF-QGJXW";
const PATH: &str = "tests/resources/pkeyconfig-win8.xrm-ms";

#[test]
fn test() {
    let key_info = KeyInfo::load(KEY).unwrap();
    let pkey_config = PKeyInfo::load_from_file(PATH).unwrap();
    assert!(key_info.is_valid());
    assert!(pkey_config.is_valid(key_info.group_id, key_info.serial_number));
    /*
    let pkey_info = pkey_config.query(key_info.group_id, key_info.serial_number, key_info.upgrade_bit).unwrap();
    let pid = &pkey_info.extended_pid;
    let vkey_info = VKeyInfo::load_one(pid).unwrap();
    assert!(!vkey_info.is_valid(pid))
     */
}

use claymore::{KeyInfo, PKeyConfig};
// use claymore::VAMTLite;

const KEY: &str = "RR3BN-3YY9P-9D7FC-7J4YF-QGJXW";
const PATH: &str = "tests/resources/pkeyconfig-win8.xrm-ms";

#[test]
fn test() {
    let key_info = KeyInfo::load(KEY).unwrap();
    let pkey_config = PKeyConfig::load_from_file(PATH).unwrap();
    assert!(key_info.is_valid());
    assert!(pkey_config.is_valid(key_info.group_id, key_info.serial_number));
    /*
    let pkey_config_info = pkey_config.query(key_info.group_id, key_info.serial_number, key_info.upgrade_bit).unwrap();
    let pid = &pkey_config_info.extended_pid;
    let vamt_lite = VAMTLite::load_one(pid).unwrap();
    assert!(!vamt_lite.is_valid(pid))
     */
}

use claymore;
use claymore::{KeyInfo, PKeyConfig};

const KEY: &str = "RR3BN-3YY9P-9D7FC-7J4YF-QGJXW";
const PATH: &str = "resources/pkeyconfig-win8.xrm-ms";

#[test]
fn test() {
    let key_info = KeyInfo::load(&KEY.to_string()).unwrap();
    let pkey_config = PKeyConfig::load_from_file(&PATH.to_string()).unwrap();
    assert!(key_info.is_valid());
    assert!(pkey_config.is_valid(key_info.group_id, key_info.serial_number));
}

use claymore::{KeyInfo, ProductInfo, LicenseInfo};

const KEY: &str = "RR3BN-3YY9P-9D7FC-7J4YF-QGJXW";
const PATH: &str = "resources/pkeyconfig-win8.xrm-ms";

fn main() {
    let key_info = KeyInfo::load(KEY).unwrap();
    let product_config = ProductInfo::load_from_file(PATH).unwrap();
    assert!(key_info.is_valid());
    assert!(product_config.is_valid(key_info.group_id, key_info.serial_number));

    let pkey_info = product_config.query(key_info.group_id, key_info.serial_number, key_info.upgrade_bit).unwrap();
    println!("{:#?}", pkey_info);

    let pid = &pkey_info.extended_pid;
    let license_info = LicenseInfo::load_one(pid).unwrap();
    assert!(!license_info.is_valid(pid));
}

use claymore::{ProductInfo, LicenseInfo};
use claymore::ProductConfigUsingKeys;

const KEY: &str = "RR3BN-3YY9P-9D7FC-7J4YF-QGJXW";
const PATH: &str = "resources/pkeyconfig-win8.xrm-ms";

fn main() {
    let product_config = ProductInfo::load_from_file(PATH).unwrap();
    assert!(product_config.is_valid_key(KEY));

    let pkey_info = product_config.query_key(KEY).unwrap();
    println!("{:#?}", pkey_info);

    let pid = &pkey_info.extended_pid;
    let license_info = LicenseInfo::load_one(pid).unwrap();
    assert!(!license_info.is_valid(pid));
}

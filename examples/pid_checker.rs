use claymore::{ProductInfo, ProductConfigUsingKeys};
use claymore::LicenseInfo;

const KEY: &str = "RR3BN-3YY9P-9D7FC-7J4YF-QGJXW";
const TEXT: &str = include_str!("../resources/pkeyconfig-win8.xrm-ms");

fn main() {
    let product_config = ProductInfo::load(TEXT).unwrap();
    product_config.print_key_info(KEY);
    if !product_config.is_valid_key(KEY) {
        return;
    }
    let product_info = product_config.query_key(KEY).unwrap();
    if !product_info.product_key_type.contains("Volume") {
        return;
    }
    let pid = product_config.query_key(KEY).unwrap().extended_pid;
    let license_info = LicenseInfo::load_one(&pid).unwrap();
    license_info.print_key_info(&pid);
}

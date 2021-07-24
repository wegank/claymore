use claymore;

fn main() {
    let key = String::from("RR3BN-3YY9P-9D7FC-7J4YF-QGJXW");
    let key_info = claymore::KeyInfo::load(&key).unwrap();
    let pkey_config = claymore::PKeyConfig::load_from_file("resources/pkeyconfig-win8.xrm-ms".to_string()).unwrap();
    println!("{:#?}", key_info);
    println!("{}", pkey_config.is_valid(key_info.group_id, key_info.serial_number));
}

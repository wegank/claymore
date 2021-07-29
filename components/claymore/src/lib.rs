pub use claymore_key::KeyInfo;
pub use claymore_product::ProductConfig;
pub use claymore_product::ProductInfo;
pub use claymore_license::LicenseInfo;

pub trait ProductConfigUsingKeys {
    fn query_key(&self, key: &str) -> Result<ProductInfo, String>;
    fn is_valid_key(&self, key: &str) -> bool;
    fn print_key_info(&self, key: &str);
}

impl ProductConfigUsingKeys for ProductConfig {
    fn query_key(&self, key: &str) -> Result<ProductInfo, String> {
        let key_info = KeyInfo::load(key)?;
        self.query(key_info.group_id, key_info.serial_number, key_info.upgrade_bit)
    }

    fn is_valid_key(&self, key: &str) -> bool {
        self.query_key(key).is_ok()
    }

    fn print_key_info(&self, key: &str) {
        println!("Product Key     : {}", key);
        print!(  "Key Status      : ");
        match self.query_key(key) {
            Ok(product_info) => {
                println!("Valid");
                println!("{}", product_info);
            },
            _ => println!("Invalid"),
        }
    }
}

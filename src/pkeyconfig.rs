mod decode;

use decode::{Configuration, KeyRange, PublicKey};

#[derive(Debug)]
pub struct PKeyConfig {
    configurations: Vec<Configuration>,
    key_ranges: Vec<KeyRange>,
    public_keys: Vec<PublicKey>,
}

impl PKeyConfig {
    pub fn load(xml: &String) -> Result<PKeyConfig, String> {
        let product_key_configuration = decode::decode(xml)?;
        Ok(PKeyConfig {
            configurations: product_key_configuration.configurations.configurations,
            key_ranges: product_key_configuration.key_ranges.key_ranges,
            public_keys: product_key_configuration.public_keys.public_keys,
        })
    }

    pub fn load_from_file(path: String) -> Result<PKeyConfig, String> {
        match std::fs::read_to_string(path) {
            Ok(xml) => PKeyConfig::load(&xml),
            Err(error) => Err(error.to_string()),
        }
    }

    pub fn is_valid(&self, group_id: u32, serial_number: u32) -> bool {
        let configurations: Vec<&Configuration> =
            self.configurations
                .iter().filter(|&config| config.ref_group_id == group_id).collect();
        if configurations.is_empty() {
            return false;
        }
        let public_keys: Vec<&PublicKey> =
            self.public_keys.iter().filter(|&key| key.group_id == group_id).collect();
        if public_keys.is_empty()
            || public_keys.get(0).unwrap().algorithm_id != "msft:rm/algorithm/pkey/2009" {
            return false;
        }
        let configuration = configurations.get(0).unwrap();
        let key_ranges: Vec<&KeyRange> =
            self.key_ranges.iter().filter(|&key_range|
                key_range.ref_act_config_id == configuration.act_config_id
                && key_range.is_valid
                && (key_range.start <= serial_number)
                && (serial_number <= key_range.end)
            ).collect();
        !key_ranges.is_empty()
    }
}
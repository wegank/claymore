mod decode;
use decode::{Configuration, KeyRange, PublicKey};

const PKEY_INVALID: &str = "Invalid product key.";

#[derive(Debug)]
pub struct PKeyConfig {
    pub configurations: Vec<Configuration>,
    pub key_ranges: Vec<KeyRange>,
    pub public_keys: Vec<PublicKey>,
}

#[derive(Debug)]
pub struct PKeyConfigInfo {
    pub act_config_id: String,
    pub edition_id: String,
    pub product_description: String,
    pub part_number: String,
    pub product_key_type: String,
    pub eula_type: String,
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

    pub fn load_from_file(path: &String) -> Result<PKeyConfig, String> {
        match std::fs::read_to_string(path) {
            Ok(xml) => PKeyConfig::load(&xml),
            Err(error) => Err(error.to_string()),
        }
    }

    pub fn query(&self, group_id: u32, serial_number: u32) -> Result<PKeyConfigInfo, String> {
        let configuration = match self.configurations.iter()
            .filter(|&config| config.ref_group_id == group_id)
            .collect::<Vec<_>>().get(0) {
            Some(&configuration) => configuration,
            _ => return Err(PKEY_INVALID.to_string()),
        };
        let key_range = match self.key_ranges.iter()
            .filter(|&key_range|
                key_range.ref_act_config_id == configuration.act_config_id
                    && key_range.is_valid
                    && key_range.start <= serial_number
                    && serial_number <= key_range.end)
            .collect::<Vec<_>>().get(0) {
            Some(&key_range) => key_range,
            _ => return Err(PKEY_INVALID.to_string()),
        };
        match self.public_keys.iter()
            .filter(|&public_key|
                public_key.group_id == group_id
                    && public_key.algorithm_id == "msft:rm/algorithm/pkey/2009")
            .collect::<Vec<_>>().get(0) {
            Some(_) => (),
            _ => return Err(PKEY_INVALID.to_string()),
        };
        let act_config_id = configuration.act_config_id.clone();
        let edition_id = configuration.edition_id.clone();
        let product_description = configuration.product_description.clone();
        let product_key_type = configuration.product_key_type.clone();
        let part_number = key_range.part_number.clone();
        let eula_type = key_range.eula_type.clone();
        Ok(PKeyConfigInfo {
            act_config_id,
            edition_id,
            product_description,
            product_key_type,
            part_number,
            eula_type,
        })
    }

    pub fn is_valid(&self, group_id: u32, serial_number: u32) -> bool {
        self.query(group_id, serial_number).is_ok()
    }
}

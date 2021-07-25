extern crate serde;
extern crate quick_xml;

use serde::Deserialize;
use quick_xml::de::from_str;

const PKEY_CONFIG_INVALID: &str = "Invalid PKeyConfig file.";

pub fn decode(xml: &String) -> Result<ProductKeyConfiguration, String> {
    let license_group = decode_stage1(xml)?;
    let xml = decode_stage2(&license_group)?;
    decode_stage3(&xml)
}

#[derive(Debug, Deserialize)]
struct LicenseGroup {
    license: License,
}

#[derive(Debug, Deserialize)]
struct License {
    #[serde(rename = "otherInfo")]
    other_info: OtherInfo,
}

#[derive(Debug, Deserialize)]
struct OtherInfo {
    #[serde(rename = "infoTables")]
    info_tables: InfoTables,
}

#[derive(Debug, Deserialize)]
struct InfoTables {
    #[serde(rename = "infoList")]
    info_list: InfoList,
}

#[derive(Debug, Deserialize)]
struct InfoList {
    #[serde(rename = "infoBin")]
    info_bin: String,
}

fn decode_stage1(xml: &String) -> Result<LicenseGroup, String> {
    match from_str(xml) {
        Ok(pkey_config_data) => Ok(pkey_config_data),
        _ => Err(PKEY_CONFIG_INVALID.to_string()),
    }
}

fn decode_stage2(license_group: &LicenseGroup) -> Result<String, String> {
    match base64::decode(&license_group.license.other_info.info_tables.info_list.info_bin) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(xml) => Ok(xml),
            Err(err) => Err(err.to_string()),
        },
        _ => Err(PKEY_CONFIG_INVALID.to_string()),
    }
}

#[derive(Debug, Deserialize)]
pub struct ProductKeyConfiguration {
    #[serde(rename = "Configurations")]
    pub configurations: Configurations,
    #[serde(rename = "KeyRanges")]
    pub key_ranges: KeyRanges,
    #[serde(rename = "PublicKeys")]
    pub public_keys: PublicKeys,
}

#[derive(Debug, Deserialize)]
pub struct Configurations {
    #[serde(rename = "Configuration")]
    pub configurations: Vec<Configuration>,
}

#[derive(Debug, Deserialize)]
pub struct KeyRanges {
    #[serde(rename = "KeyRange")]
    pub key_ranges: Vec<KeyRange>,
}

#[derive(Debug, Deserialize)]
pub struct PublicKeys {
    #[serde(rename = "PublicKey")]
    pub public_keys: Vec<PublicKey>,
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    #[serde(rename = "ActConfigId")]
    pub act_config_id: String,
    #[serde(rename = "RefGroupId")]
    pub ref_group_id: u32,
    #[serde(rename = "EditionId")]
    pub edition_id: String,
    #[serde(rename = "ProductDescription")]
    pub product_description: String,
    #[serde(rename = "ProductKeyType")]
    pub product_key_type: String,
    #[serde(rename = "IsRandomized")]
    pub is_randomized: bool,
}

#[derive(Debug, Deserialize)]
pub struct KeyRange {
    #[serde(rename = "RefActConfigId")]
    pub ref_act_config_id: String,
    #[serde(rename = "PartNumber")]
    pub part_number: String,
    #[serde(rename = "EulaType")]
    pub eula_type: String,
    #[serde(rename = "IsValid")]
    pub is_valid: bool,
    #[serde(rename = "Start")]
    pub start: u32,
    #[serde(rename = "End")]
    pub end: u32,
}

#[derive(Debug, Deserialize)]
pub struct PublicKey {
    #[serde(rename = "GroupId")]
    pub group_id: u32,
    #[serde(rename = "AlgorithmId")]
    pub algorithm_id: String,
    #[serde(rename = "PublicKeyValue")]
    pub public_key_value: String,
}

fn decode_stage3(xml: &String) -> Result<ProductKeyConfiguration, String> {
    match from_str(xml) {
        Ok(product_key_configuration) => Ok(product_key_configuration),
        _ => Err(PKEY_CONFIG_INVALID.to_string()),
    }
}

pub mod request;
pub mod deserialize;

use std::collections::HashMap;

const PID_NOT_FOUND: &str = "Extended PID not found.";

pub struct LicenseInfo {
    pub responses: HashMap<String, i32>,
}

impl LicenseInfo {
    pub fn load(pid_list: &Vec<String>) -> Result<LicenseInfo, String> {
        let pid_list: Vec<String> = pid_list.into_iter()
            .map(|pid| pid.replace("XXXXX", "12345")).collect();
        match request::request(&pid_list) {
            Ok(xml) => match deserialize::parse(&xml) {
                Ok(envelope) => Ok(LicenseInfo {
                    responses: envelope.body
                        .batch_activate_response
                        .batch_activate_result.response_xml
                        .activation_response.responses.responses
                        .into_iter().map(|response|
                        ("XXXXX".to_string() + &response.pid[5..], response.activation_remaining))
                        .collect(),
                }),
                Err(err) => Err(err),
            },
            Err(err) => Err(format!("{:#?}", err)),
        }
    }

    pub fn load_one(pid: &str) -> Result<LicenseInfo, String> {
        LicenseInfo::load(&vec![pid.into()])
    }

    pub fn query(&self, pid: &str) -> Result<i32, String> {
        match self.responses.get(pid) {
            Some(&activation_remaining) => Ok(activation_remaining),
            _ => Err(PID_NOT_FOUND.to_string()),
        }
    }

    pub fn is_valid(&self, pid: &str) -> bool {
        match self.query(pid) {
            Ok(activation_remaining) => activation_remaining > 0,
            _ => false,
        }
    }
}

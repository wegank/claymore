pub mod request;
pub mod deserialize;

use deserialize::Response;

pub struct VAMTLite {}

impl VAMTLite {
    pub fn query(pid_list: &Vec<String>) -> Result<Vec<Response>, String> {
        match request::request(pid_list) {
            Ok(xml) => match deserialize::parse(&xml) {
                Ok(envelope) => Ok(envelope.body
                    .batch_activate_response
                    .batch_activate_result.response_xml
                    .activation_response.responses.responses),
                Err(err) => Err(err),
            },
            Err(err) => Err(format!("{:#?}", err)),
        }
    }

    pub fn query_one(pid: &str) -> Result<Vec<Response>, String> {
        VAMTLite::query(&vec![pid.into()])
    }
}

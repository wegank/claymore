pub mod request;
pub mod deserialize;

pub struct VAMTLite {}

impl VAMTLite {
    pub fn query(pid: &str) -> Result<i32, String> {
        let pid = pid.replace("XXXXX", "12345");
        let responses = match request::request(&pid) {
            Ok(xml) => match deserialize::parse(&xml) {
                Ok(envelope) => envelope.body
                    .batch_activate_response
                    .batch_activate_result.response_xml
                    .activation_response.responses.responses,
                Err(err) => return Err(err),
            },
            Err(err) => return Err(format!("{:#?}", err)),
        };
        let responses = responses.into_iter()
            .filter(|response| response.pid == pid).collect::<Vec<_>>();
        match responses.get(0) {
            Some(response) => Ok(response.activation_remaining),
            _ => Ok(-1),
        }
    }
}

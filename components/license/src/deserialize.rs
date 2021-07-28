use serde::Deserialize;
use quick_xml::de::from_str;

#[derive(Debug, Deserialize)]
pub struct Envelope {
    #[serde(rename = "Body")]
    pub body: Body,
}

#[derive(Debug, Deserialize)]
pub struct Body {
    #[serde(rename = "BatchActivateResponse")]
    pub batch_activate_response: BatchActivateResponse,
}

#[derive(Debug, Deserialize)]
pub struct BatchActivateResponse {
    #[serde(rename = "BatchActivateResult")]
    pub batch_activate_result: BatchActivateResult,
}

#[derive(Debug, Deserialize)]
pub struct BatchActivateResult {
    #[serde(rename = "ResponseXml")]
    pub response_xml: ResponseXml,
}

#[derive(Debug, Deserialize)]
pub struct ResponseXml {
    #[serde(rename = "ActivationResponse")]
    pub activation_response: ActivationResponse,
}

#[derive(Debug, Deserialize)]
pub struct ActivationResponse {
    #[serde(rename = "Responses")]
    pub responses: Responses,
}

#[derive(Debug, Deserialize)]
pub struct Responses {
    #[serde(rename = "Response")]
    pub responses: Vec<Response>,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    #[serde(rename = "PID")]
    pub pid: String,
    #[serde(rename = "ActivationRemaining")]
    pub activation_remaining: i32,
}

pub fn parse(xml: &String) -> Result<Envelope, String> {
    match from_str(&xml.replace("&lt;", "<").replace("&gt;", ">")) {
        Ok(envelope) => Ok(envelope),
        Err(err) => Err(err.to_string()),
    }
}

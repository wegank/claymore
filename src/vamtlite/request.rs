use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;

pub fn inner_xml(pid_list: &Vec<String>) -> String {
    let url = r#""http://www.microsoft.com/DRM/SL/BatchActivationRequest/1.0""#;
    let mut xml = "\
    <ActivationRequest xmlns=".to_string() + url + ">\
        <VersionNumber>2.0</VersionNumber>\
        <RequestType>2</RequestType>\
        <Requests>";
    for pid in pid_list.iter() {
        xml = xml + "\
            <Request>\
                <PID>" + &pid.replace("XXXXX", "12345") + "</PID>\
            </Request>";
    }
    xml += "\
        </Requests>\
    </ActivationRequest>";
    xml.chars().map(|c| c.to_string() + "\0").collect::<Vec<_>>().join("")
}

fn digest(xml: &String) -> String {
    const KEY: [u8; 32] = [
        0xfe, 0x31, 0x98, 0x75, 0xfb, 0x48, 0x84, 0x86,
        0x9c, 0xf3, 0xf1, 0xce, 0x99, 0xa8, 0x90, 0x64,
        0xab, 0x57, 0x1f, 0xca, 0x47, 0x04, 0x50, 0x58,
        0x30, 0x24, 0xE2, 0x14, 0x62, 0x87, 0x79, 0xa0
    ];
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(&KEY).unwrap();
    mac.update(xml.as_bytes());
    base64::encode(mac.finalize().into_bytes())
}

pub fn envelope(xml: &String) -> String {
    let url = r#""http://www.microsoft.com/BatchActivationService""#;
    r#"<?xml version="1.0" encoding="utf-8"?>"#.to_string() +
    r#"<soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/""#
        +               r#" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance""#
        +               r#" xmlns:xsd="http://www.w3.org/2001/XMLSchema">"#
        +  "<soap:Body>\
                <BatchActivate xmlns=" + url + ">\
                    <request>\
                        <Digest>" + &digest(xml) + "</Digest>\
                        <RequestXml>" + &base64::encode(xml) + "</RequestXml>\
                    </request>\
                </BatchActivate>\
            </soap:Body>\
        </soap:Envelope>"
}

#[tokio::main]
pub async fn request(pid: &Vec<String>) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true).build()?;
    let xml = client.post("https://activation.sls.microsoft.com/BatchActivation/BatchActivation.asmx")
        .header("Content-Type", "text/xml; charset=utf-8")
        .header("SOAPAction", "http://www.microsoft.com/BatchActivationService/BatchActivate")
        .header("Host", "activation.sls.microsoft.com")
        .body(envelope(&inner_xml(pid)))
        .send().await?.text().await?;
    println!("{:#?}", xml);
    Ok(xml)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;
    use sha2::Digest;

    const PID: &str = "XXXXX-01785-029-888334-03-1033-9200.0000-2072021";

    #[test]
    fn test_inner_xml() {
        let mut hasher = sha2::Sha256::new();
        hasher.update(base64::encode(inner_xml(&vec![PID.into()])));
        assert_eq!(hasher.finalize()[..], hex!(
            "df54f6dead20b673234aea2c0ae81efd306262b4939ed4451d3dec35e23de490")[..]);
    }

    #[test]
    fn test_digest() {
        assert_eq!(digest(&inner_xml(&vec![PID.into()])),
                   "7dGAT9dw3id1wXkdKa6J8Lk/K1eb43a++KAsMxVd3yc=");
    }

    #[test]
    fn test_envelope() {
        let mut hasher = sha2::Sha256::new();
        hasher.update(envelope(&inner_xml(&vec![PID.into()])));
        assert_eq!(hasher.finalize()[..], hex!(
            "0c8cc5d5223b4fbed9119f0016552bccb6b5614ffca3e5e0dd1e947919fb7c5f")[..]);
    }
}
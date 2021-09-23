/*
 * Twilio - Wireless
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListSimResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListCommandResponseMeta>>,
    #[serde(rename = "sims", skip_serializing_if = "Option::is_none")]
    pub sims: Option<Vec<crate::models::WirelessV1Sim>>,
}

impl ListSimResponse {
    pub fn new() -> ListSimResponse {
        ListSimResponse {
            meta: None,
            sims: None,
        }
    }
}



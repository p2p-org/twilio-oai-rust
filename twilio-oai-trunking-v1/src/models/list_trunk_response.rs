/*
 * Twilio - Trunking
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListTrunkResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListTrunkResponseMeta>>,
    #[serde(rename = "trunks", skip_serializing_if = "Option::is_none")]
    pub trunks: Option<Vec<crate::models::TrunkingV1Trunk>>,
}

impl ListTrunkResponse {
    pub fn new() -> ListTrunkResponse {
        ListTrunkResponse {
            meta: None,
            trunks: None,
        }
    }
}

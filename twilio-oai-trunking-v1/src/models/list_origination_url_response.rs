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
pub struct ListOriginationUrlResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListTrunkResponseMeta>>,
    #[serde(rename = "origination_urls", skip_serializing_if = "Option::is_none")]
    pub origination_urls: Option<Vec<crate::models::TrunkingV1TrunkOriginationUrl>>,
}

impl ListOriginationUrlResponse {
    pub fn new() -> ListOriginationUrlResponse {
        ListOriginationUrlResponse {
            meta: None,
            origination_urls: None,
        }
    }
}

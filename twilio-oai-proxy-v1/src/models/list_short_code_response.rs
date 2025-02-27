/*
 * Twilio - Proxy
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListShortCodeResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListServiceResponseMeta>>,
    #[serde(rename = "short_codes", skip_serializing_if = "Option::is_none")]
    pub short_codes: Option<Vec<crate::models::ProxyV1ServiceShortCode>>,
}

impl ListShortCodeResponse {
    pub fn new() -> ListShortCodeResponse {
        ListShortCodeResponse {
            meta: None,
            short_codes: None,
        }
    }
}

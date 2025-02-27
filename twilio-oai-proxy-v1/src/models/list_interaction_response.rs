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
pub struct ListInteractionResponse {
    #[serde(rename = "interactions", skip_serializing_if = "Option::is_none")]
    pub interactions: Option<Vec<crate::models::ProxyV1ServiceSessionInteraction>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListServiceResponseMeta>>,
}

impl ListInteractionResponse {
    pub fn new() -> ListInteractionResponse {
        ListInteractionResponse {
            interactions: None,
            meta: None,
        }
    }
}

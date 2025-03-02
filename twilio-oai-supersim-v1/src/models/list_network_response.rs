/*
 * Twilio - Supersim
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListNetworkResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListEsimProfileResponseMeta>>,
    #[serde(rename = "networks", skip_serializing_if = "Option::is_none")]
    pub networks: Option<Vec<crate::models::SupersimV1Network>>,
}

impl ListNetworkResponse {
    pub fn new() -> ListNetworkResponse {
        ListNetworkResponse {
            meta: None,
            networks: None,
        }
    }
}

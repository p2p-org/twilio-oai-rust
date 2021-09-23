/*
 * Twilio - Chat
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListUserChannelResponse {
    #[serde(rename = "channels", skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<crate::models::ChatV1ServiceUserUserChannel>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListCredentialResponseMeta>>,
}

impl ListUserChannelResponse {
    pub fn new() -> ListUserChannelResponse {
        ListUserChannelResponse {
            channels: None,
            meta: None,
        }
    }
}



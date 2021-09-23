/*
 * Twilio - Notify
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListBindingResponse {
    #[serde(rename = "bindings", skip_serializing_if = "Option::is_none")]
    pub bindings: Option<Vec<crate::models::NotifyV1ServiceBinding>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListCredentialResponseMeta>>,
}

impl ListBindingResponse {
    pub fn new() -> ListBindingResponse {
        ListBindingResponse {
            bindings: None,
            meta: None,
        }
    }
}



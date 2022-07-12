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
pub struct ListCredentialListResponse {
    #[serde(rename = "credential_lists", skip_serializing_if = "Option::is_none")]
    pub credential_lists: Option<Vec<crate::models::TrunkingV1TrunkCredentialList>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListTrunkResponseMeta>>,
}

impl ListCredentialListResponse {
    pub fn new() -> ListCredentialListResponse {
        ListCredentialListResponse {
            credential_lists: None,
            meta: None,
        }
    }
}

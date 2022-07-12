/*
 * Twilio - Voice
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListConnectionPolicyTargetResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListByocTrunkResponseMeta>>,
    #[serde(rename = "targets", skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<crate::models::VoiceV1ConnectionPolicyConnectionPolicyTarget>>,
}

impl ListConnectionPolicyTargetResponse {
    pub fn new() -> ListConnectionPolicyTargetResponse {
        ListConnectionPolicyTargetResponse {
            meta: None,
            targets: None,
        }
    }
}

/*
 * Twilio - Studio
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListFlowRevisionResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListFlowResponseMeta>>,
    #[serde(rename = "revisions", skip_serializing_if = "Option::is_none")]
    pub revisions: Option<Vec<crate::models::StudioV2FlowFlowRevision>>,
}

impl ListFlowRevisionResponse {
    pub fn new() -> ListFlowRevisionResponse {
        ListFlowRevisionResponse {
            meta: None,
            revisions: None,
        }
    }
}

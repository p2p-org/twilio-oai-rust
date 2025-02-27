/*
 * Twilio - Insights
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListConferenceResponse {
    #[serde(rename = "conferences", skip_serializing_if = "Option::is_none")]
    pub conferences: Option<Vec<crate::models::InsightsV1Conference>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListConferenceResponseMeta>>,
}

impl ListConferenceResponse {
    pub fn new() -> ListConferenceResponse {
        ListConferenceResponse {
            conferences: None,
            meta: None,
        }
    }
}

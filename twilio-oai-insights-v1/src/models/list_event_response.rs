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
pub struct ListEventResponse {
    #[serde(rename = "events", skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<crate::models::InsightsV1CallEvent>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListConferenceResponseMeta>>,
}

impl ListEventResponse {
    pub fn new() -> ListEventResponse {
        ListEventResponse {
            events: None,
            meta: None,
        }
    }
}

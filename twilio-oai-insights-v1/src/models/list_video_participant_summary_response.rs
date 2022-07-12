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
pub struct ListVideoParticipantSummaryResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListConferenceResponseMeta>>,
    #[serde(rename = "participants", skip_serializing_if = "Option::is_none")]
    pub participants: Option<Vec<crate::models::InsightsV1VideoRoomSummaryVideoParticipantSummary>>,
}

impl ListVideoParticipantSummaryResponse {
    pub fn new() -> ListVideoParticipantSummaryResponse {
        ListVideoParticipantSummaryResponse {
            meta: None,
            participants: None,
        }
    }
}

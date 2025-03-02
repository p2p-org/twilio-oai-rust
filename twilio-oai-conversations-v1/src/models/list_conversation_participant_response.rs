/*
 * Twilio - Conversations
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListConversationParticipantResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListConfigurationAddressResponseMeta>>,
    #[serde(rename = "participants", skip_serializing_if = "Option::is_none")]
    pub participants:
        Option<Vec<crate::models::ConversationsV1ConversationConversationParticipant>>,
}

impl ListConversationParticipantResponse {
    pub fn new() -> ListConversationParticipantResponse {
        ListConversationParticipantResponse {
            meta: None,
            participants: None,
        }
    }
}

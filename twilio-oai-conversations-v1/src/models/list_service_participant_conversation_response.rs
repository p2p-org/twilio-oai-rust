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
pub struct ListServiceParticipantConversationResponse {
    #[serde(rename = "conversations", skip_serializing_if = "Option::is_none")]
    pub conversations:
        Option<Vec<crate::models::ConversationsV1ServiceServiceParticipantConversation>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListConfigurationAddressResponseMeta>>,
}

impl ListServiceParticipantConversationResponse {
    pub fn new() -> ListServiceParticipantConversationResponse {
        ListServiceParticipantConversationResponse {
            conversations: None,
            meta: None,
        }
    }
}

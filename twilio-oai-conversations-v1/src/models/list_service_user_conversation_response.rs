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
pub struct ListServiceUserConversationResponse {
    #[serde(rename = "conversations", skip_serializing_if = "Option::is_none")]
    pub conversations:
        Option<Vec<crate::models::ConversationsV1ServiceServiceUserServiceUserConversation>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListConfigurationAddressResponseMeta>>,
}

impl ListServiceUserConversationResponse {
    pub fn new() -> ListServiceUserConversationResponse {
        ListServiceUserConversationResponse {
            conversations: None,
            meta: None,
        }
    }
}

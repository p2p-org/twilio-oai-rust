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
pub struct ListServiceConversationScopedWebhookResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListConfigurationAddressResponseMeta>>,
    #[serde(rename = "webhooks", skip_serializing_if = "Option::is_none")]
    pub webhooks: Option<Vec<crate::models::ConversationsV1ServiceServiceConversationServiceConversationScopedWebhook>>,
}

impl ListServiceConversationScopedWebhookResponse {
    pub fn new() -> ListServiceConversationScopedWebhookResponse {
        ListServiceConversationScopedWebhookResponse {
            meta: None,
            webhooks: None,
        }
    }
}

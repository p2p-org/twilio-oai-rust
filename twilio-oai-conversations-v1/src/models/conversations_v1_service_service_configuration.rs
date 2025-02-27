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
pub struct ConversationsV1ServiceServiceConfiguration {
    /// The unique string that identifies the resource
    #[serde(rename = "chat_service_sid", skip_serializing_if = "Option::is_none")]
    pub chat_service_sid: Option<String>,
    /// The service role assigned to users when they are added to the service
    #[serde(
        rename = "default_chat_service_role_sid",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_chat_service_role_sid: Option<String>,
    /// The role assigned to a conversation creator user when they join a new conversation
    #[serde(
        rename = "default_conversation_creator_role_sid",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_conversation_creator_role_sid: Option<String>,
    /// The role assigned to users when they are added to a conversation
    #[serde(
        rename = "default_conversation_role_sid",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_conversation_role_sid: Option<String>,
    /// Absolute URL to access the push notifications configuration of this service.
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// Whether the Reachability Indicator feature is enabled for this Conversations Service
    #[serde(
        rename = "reachability_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub reachability_enabled: Option<bool>,
    /// An absolute URL for this service configuration.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ConversationsV1ServiceServiceConfiguration {
    pub fn new() -> ConversationsV1ServiceServiceConfiguration {
        ConversationsV1ServiceServiceConfiguration {
            chat_service_sid: None,
            default_chat_service_role_sid: None,
            default_conversation_creator_role_sid: None,
            default_conversation_role_sid: None,
            links: None,
            reachability_enabled: None,
            url: None,
        }
    }
}

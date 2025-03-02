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
pub struct ConversationsV1ServiceServiceConversation {
    /// The unique ID of the Account responsible for this conversation.
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// An optional string metadata field you can use to store any data you wish.
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    #[serde(rename = "bindings", skip_serializing_if = "Option::is_none")]
    pub bindings: Option<serde_json::Value>,
    /// The unique ID of the Conversation Service this conversation belongs to.
    #[serde(rename = "chat_service_sid", skip_serializing_if = "Option::is_none")]
    pub chat_service_sid: Option<String>,
    /// The date that this resource was created.
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The date that this resource was last updated.
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The human-readable name of this conversation.
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// Absolute URLs to access the participants, messages and webhooks of this conversation.
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// The unique ID of the Messaging Service this conversation belongs to.
    #[serde(
        rename = "messaging_service_sid",
        skip_serializing_if = "Option::is_none"
    )]
    pub messaging_service_sid: Option<String>,
    /// A 34 character string that uniquely identifies this resource.
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// Current state of this conversation.
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// Timer date values for this conversation.
    #[serde(rename = "timers", skip_serializing_if = "Option::is_none")]
    pub timers: Option<serde_json::Value>,
    /// An application-defined string that uniquely identifies the resource
    #[serde(rename = "unique_name", skip_serializing_if = "Option::is_none")]
    pub unique_name: Option<String>,
    /// An absolute URL for this conversation.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ConversationsV1ServiceServiceConversation {
    pub fn new() -> ConversationsV1ServiceServiceConversation {
        ConversationsV1ServiceServiceConversation {
            account_sid: None,
            attributes: None,
            bindings: None,
            chat_service_sid: None,
            date_created: None,
            date_updated: None,
            friendly_name: None,
            links: None,
            messaging_service_sid: None,
            sid: None,
            state: None,
            timers: None,
            unique_name: None,
            url: None,
        }
    }
}

/// Current state of this conversation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "inactive")]
    Inactive,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "closed")]
    Closed,
}

impl Default for State {
    fn default() -> State {
        Self::Inactive
    }
}

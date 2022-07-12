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
pub struct ConversationsV1User {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The JSON Object string that stores application-specific data
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    /// The SID of the Conversation Service that the resource is associated with
    #[serde(rename = "chat_service_sid", skip_serializing_if = "Option::is_none")]
    pub chat_service_sid: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The string that you assigned to describe the resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The string that identifies the resource's User
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// Whether the User has a potentially valid Push Notification registration for this Conversations Service
    #[serde(rename = "is_notifiable", skip_serializing_if = "Option::is_none")]
    pub is_notifiable: Option<bool>,
    /// Whether the User is actively connected to this Conversations Service and online
    #[serde(rename = "is_online", skip_serializing_if = "Option::is_none")]
    pub is_online: Option<bool>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// The SID of a service-level Role assigned to the user
    #[serde(rename = "role_sid", skip_serializing_if = "Option::is_none")]
    pub role_sid: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// An absolute URL for this user.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ConversationsV1User {
    pub fn new() -> ConversationsV1User {
        ConversationsV1User {
            account_sid: None,
            attributes: None,
            chat_service_sid: None,
            date_created: None,
            date_updated: None,
            friendly_name: None,
            identity: None,
            is_notifiable: None,
            is_online: None,
            links: None,
            role_sid: None,
            sid: None,
            url: None,
        }
    }
}

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
pub struct ConversationsV1ServiceServiceBinding {
    /// The unique ID of the Account responsible for this binding.
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The push technology to use for the binding.
    #[serde(rename = "binding_type", skip_serializing_if = "Option::is_none")]
    pub binding_type: Option<BindingType>,
    /// The SID of the Conversation Service that the resource is associated with.
    #[serde(rename = "chat_service_sid", skip_serializing_if = "Option::is_none")]
    pub chat_service_sid: Option<String>,
    /// The SID of the Credential for the binding.
    #[serde(rename = "credential_sid", skip_serializing_if = "Option::is_none")]
    pub credential_sid: Option<String>,
    /// The date that this resource was created.
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The date that this resource was last updated.
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The unique endpoint identifier for the Binding.
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// The identity of Conversation User associated with this binding.
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// The Conversation message types the binding is subscribed to.
    #[serde(rename = "message_types", skip_serializing_if = "Option::is_none")]
    pub message_types: Option<Vec<String>>,
    /// A 34 character string that uniquely identifies this resource.
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// An absolute URL for this binding.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ConversationsV1ServiceServiceBinding {
    pub fn new() -> ConversationsV1ServiceServiceBinding {
        ConversationsV1ServiceServiceBinding {
            account_sid: None,
            binding_type: None,
            chat_service_sid: None,
            credential_sid: None,
            date_created: None,
            date_updated: None,
            endpoint: None,
            identity: None,
            message_types: None,
            sid: None,
            url: None,
        }
    }
}

/// The push technology to use for the binding.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BindingType {
    #[serde(rename = "apn")]
    Apn,
    #[serde(rename = "gcm")]
    Gcm,
    #[serde(rename = "fcm")]
    Fcm,
}

impl Default for BindingType {
    fn default() -> BindingType {
        Self::Apn
    }
}

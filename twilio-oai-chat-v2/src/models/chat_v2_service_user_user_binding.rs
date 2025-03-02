/*
 * Twilio - Chat
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ChatV2ServiceUserUserBinding {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The push technology to use for the binding
    #[serde(rename = "binding_type", skip_serializing_if = "Option::is_none")]
    pub binding_type: Option<BindingType>,
    /// The SID of the Credential for the binding
    #[serde(rename = "credential_sid", skip_serializing_if = "Option::is_none")]
    pub credential_sid: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The unique endpoint identifier for the User Binding
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// The string that identifies the resource's User
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// The Programmable Chat message types the binding is subscribed to
    #[serde(rename = "message_types", skip_serializing_if = "Option::is_none")]
    pub message_types: Option<Vec<String>>,
    /// The SID of the Service that the resource is associated with
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The absolute URL of the User Binding resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The SID of the User with the binding
    #[serde(rename = "user_sid", skip_serializing_if = "Option::is_none")]
    pub user_sid: Option<String>,
}

impl ChatV2ServiceUserUserBinding {
    pub fn new() -> ChatV2ServiceUserUserBinding {
        ChatV2ServiceUserUserBinding {
            account_sid: None,
            binding_type: None,
            credential_sid: None,
            date_created: None,
            date_updated: None,
            endpoint: None,
            identity: None,
            message_types: None,
            service_sid: None,
            sid: None,
            url: None,
            user_sid: None,
        }
    }
}

/// The push technology to use for the binding
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BindingType {
    #[serde(rename = "gcm")]
    Gcm,
    #[serde(rename = "apn")]
    Apn,
    #[serde(rename = "fcm")]
    Fcm,
}

impl Default for BindingType {
    fn default() -> BindingType {
        Self::Gcm
    }
}

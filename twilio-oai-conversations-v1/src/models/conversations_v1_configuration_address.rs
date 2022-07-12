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
pub struct ConversationsV1ConfigurationAddress {
    /// The unique ID of the Account the address belongs to.
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The unique address to be configured.
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Auto Creation configuration for the address.
    #[serde(rename = "auto_creation", skip_serializing_if = "Option::is_none")]
    pub auto_creation: Option<serde_json::Value>,
    /// The date that this resource was created.
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The date that this resource was last updated.
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The human-readable name of this configuration.
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// A 34 character string that uniquely identifies this resource.
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// Type of Address.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// An absolute URL for this address configuration.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ConversationsV1ConfigurationAddress {
    pub fn new() -> ConversationsV1ConfigurationAddress {
        ConversationsV1ConfigurationAddress {
            account_sid: None,
            address: None,
            auto_creation: None,
            date_created: None,
            date_updated: None,
            friendly_name: None,
            sid: None,
            _type: None,
            url: None,
        }
    }
}

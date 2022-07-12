/*
 * Twilio - Notify
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NotifyV1ServiceBinding {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The channel-specific address
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// The type of the Binding
    #[serde(rename = "binding_type", skip_serializing_if = "Option::is_none")]
    pub binding_type: Option<String>,
    /// The SID of the Credential resource to be used to send notifications to this Binding
    #[serde(rename = "credential_sid", skip_serializing_if = "Option::is_none")]
    pub credential_sid: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// Deprecated
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// The `identity` value that identifies the new resource's User
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// The URLs of related resources
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// The protocol version to use to send the notification
    #[serde(
        rename = "notification_protocol_version",
        skip_serializing_if = "Option::is_none"
    )]
    pub notification_protocol_version: Option<String>,
    /// The SID of the Service that the resource is associated with
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The list of tags associated with this Binding
    #[serde(rename = "tags", skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The absolute URL of the Binding resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl NotifyV1ServiceBinding {
    pub fn new() -> NotifyV1ServiceBinding {
        NotifyV1ServiceBinding {
            account_sid: None,
            address: None,
            binding_type: None,
            credential_sid: None,
            date_created: None,
            date_updated: None,
            endpoint: None,
            identity: None,
            links: None,
            notification_protocol_version: None,
            service_sid: None,
            sid: None,
            tags: None,
            url: None,
        }
    }
}

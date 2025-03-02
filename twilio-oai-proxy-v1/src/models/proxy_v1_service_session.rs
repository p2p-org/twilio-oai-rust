/*
 * Twilio - Proxy
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProxyV1ServiceSession {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The reason the Session ended
    #[serde(rename = "closed_reason", skip_serializing_if = "Option::is_none")]
    pub closed_reason: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 date when the Session ended
    #[serde(rename = "date_ended", skip_serializing_if = "Option::is_none")]
    pub date_ended: Option<String>,
    /// The ISO 8601 date when the Session should expire
    #[serde(rename = "date_expiry", skip_serializing_if = "Option::is_none")]
    pub date_expiry: Option<String>,
    /// The ISO 8601 date when the Session last had an interaction
    #[serde(
        rename = "date_last_interaction",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_last_interaction: Option<String>,
    /// The ISO 8601 date when the Session started
    #[serde(rename = "date_started", skip_serializing_if = "Option::is_none")]
    pub date_started: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The URLs of resources related to the Session
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// The Mode of the Session
    #[serde(rename = "mode", skip_serializing_if = "Option::is_none")]
    pub mode: Option<Mode>,
    /// The SID of the resource's parent Service
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The status of the Session
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// When the session will expire
    #[serde(rename = "ttl", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
    /// An application-defined string that uniquely identifies the resource
    #[serde(rename = "unique_name", skip_serializing_if = "Option::is_none")]
    pub unique_name: Option<String>,
    /// The absolute URL of the Session resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ProxyV1ServiceSession {
    pub fn new() -> ProxyV1ServiceSession {
        ProxyV1ServiceSession {
            account_sid: None,
            closed_reason: None,
            date_created: None,
            date_ended: None,
            date_expiry: None,
            date_last_interaction: None,
            date_started: None,
            date_updated: None,
            links: None,
            mode: None,
            service_sid: None,
            sid: None,
            status: None,
            ttl: None,
            unique_name: None,
            url: None,
        }
    }
}

/// The Mode of the Session
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Mode {
    #[serde(rename = "message-only")]
    MessageOnly,
    #[serde(rename = "voice-only")]
    VoiceOnly,
    #[serde(rename = "voice-and-message")]
    VoiceAndMessage,
}

impl Default for Mode {
    fn default() -> Mode {
        Self::MessageOnly
    }
}
/// The status of the Session
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "in-progress")]
    InProgress,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for Status {
    fn default() -> Status {
        Self::Open
    }
}

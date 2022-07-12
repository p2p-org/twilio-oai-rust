/*
 * Twilio - Wireless
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WirelessV1Sim {
    /// The SID of the Account to which the Sim resource belongs
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The HTTP method we use to call commands_callback_url
    #[serde(
        rename = "commands_callback_method",
        skip_serializing_if = "Option::is_none"
    )]
    pub commands_callback_method: Option<CommandsCallbackMethod>,
    /// The URL we call when the SIM originates a machine-to-machine Command
    #[serde(
        rename = "commands_callback_url",
        skip_serializing_if = "Option::is_none"
    )]
    pub commands_callback_url: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 date and time in GMT when the Sim resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// Deprecated
    #[serde(rename = "e_id", skip_serializing_if = "Option::is_none")]
    pub e_id: Option<String>,
    /// The string that you assigned to describe the Sim resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The ICCID associated with the SIM
    #[serde(rename = "iccid", skip_serializing_if = "Option::is_none")]
    pub iccid: Option<String>,
    /// Deprecated
    #[serde(rename = "ip_address", skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// The URLs of related subresources
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// The SID of the RatePlan resource to which the Sim resource is assigned.
    #[serde(rename = "rate_plan_sid", skip_serializing_if = "Option::is_none")]
    pub rate_plan_sid: Option<String>,
    /// The connectivity reset status of the SIM
    #[serde(rename = "reset_status", skip_serializing_if = "Option::is_none")]
    pub reset_status: Option<ResetStatus>,
    /// The unique string that identifies the Sim resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// Deprecated
    #[serde(
        rename = "sms_fallback_method",
        skip_serializing_if = "Option::is_none"
    )]
    pub sms_fallback_method: Option<SmsFallbackMethod>,
    /// Deprecated
    #[serde(rename = "sms_fallback_url", skip_serializing_if = "Option::is_none")]
    pub sms_fallback_url: Option<String>,
    /// Deprecated
    #[serde(rename = "sms_method", skip_serializing_if = "Option::is_none")]
    pub sms_method: Option<SmsMethod>,
    /// Deprecated
    #[serde(rename = "sms_url", skip_serializing_if = "Option::is_none")]
    pub sms_url: Option<String>,
    /// The status of the Sim resource
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// An application-defined string that uniquely identifies the resource
    #[serde(rename = "unique_name", skip_serializing_if = "Option::is_none")]
    pub unique_name: Option<String>,
    /// The absolute URL of the resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Deprecated. The HTTP method we use to call voice_fallback_url
    #[serde(
        rename = "voice_fallback_method",
        skip_serializing_if = "Option::is_none"
    )]
    pub voice_fallback_method: Option<VoiceFallbackMethod>,
    /// Deprecated. The URL we call when an error occurs while retrieving or executing the TwiML requested from voice_url
    #[serde(rename = "voice_fallback_url", skip_serializing_if = "Option::is_none")]
    pub voice_fallback_url: Option<String>,
    /// Deprecated. The HTTP method we use to call voice_url
    #[serde(rename = "voice_method", skip_serializing_if = "Option::is_none")]
    pub voice_method: Option<VoiceMethod>,
    /// Deprecated. The URL we call when the SIM-connected device makes a voice call
    #[serde(rename = "voice_url", skip_serializing_if = "Option::is_none")]
    pub voice_url: Option<String>,
}

impl WirelessV1Sim {
    pub fn new() -> WirelessV1Sim {
        WirelessV1Sim {
            account_sid: None,
            commands_callback_method: None,
            commands_callback_url: None,
            date_created: None,
            date_updated: None,
            e_id: None,
            friendly_name: None,
            iccid: None,
            ip_address: None,
            links: None,
            rate_plan_sid: None,
            reset_status: None,
            sid: None,
            sms_fallback_method: None,
            sms_fallback_url: None,
            sms_method: None,
            sms_url: None,
            status: None,
            unique_name: None,
            url: None,
            voice_fallback_method: None,
            voice_fallback_url: None,
            voice_method: None,
            voice_url: None,
        }
    }
}

/// The HTTP method we use to call commands_callback_url
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CommandsCallbackMethod {
    #[serde(rename = "HEAD")]
    HEAD,
    #[serde(rename = "GET")]
    GET,
    #[serde(rename = "POST")]
    POST,
    #[serde(rename = "PATCH")]
    PATCH,
    #[serde(rename = "PUT")]
    PUT,
    #[serde(rename = "DELETE")]
    DELETE,
}

impl Default for CommandsCallbackMethod {
    fn default() -> CommandsCallbackMethod {
        Self::HEAD
    }
}
/// The connectivity reset status of the SIM
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ResetStatus {
    #[serde(rename = "resetting")]
    Resetting,
}

impl Default for ResetStatus {
    fn default() -> ResetStatus {
        Self::Resetting
    }
}
/// Deprecated
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SmsFallbackMethod {
    #[serde(rename = "HEAD")]
    HEAD,
    #[serde(rename = "GET")]
    GET,
    #[serde(rename = "POST")]
    POST,
    #[serde(rename = "PATCH")]
    PATCH,
    #[serde(rename = "PUT")]
    PUT,
    #[serde(rename = "DELETE")]
    DELETE,
}

impl Default for SmsFallbackMethod {
    fn default() -> SmsFallbackMethod {
        Self::HEAD
    }
}
/// Deprecated
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SmsMethod {
    #[serde(rename = "HEAD")]
    HEAD,
    #[serde(rename = "GET")]
    GET,
    #[serde(rename = "POST")]
    POST,
    #[serde(rename = "PATCH")]
    PATCH,
    #[serde(rename = "PUT")]
    PUT,
    #[serde(rename = "DELETE")]
    DELETE,
}

impl Default for SmsMethod {
    fn default() -> SmsMethod {
        Self::HEAD
    }
}
/// The status of the Sim resource
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "deactivated")]
    Deactivated,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "updating")]
    Updating,
}

impl Default for Status {
    fn default() -> Status {
        Self::New
    }
}
/// Deprecated. The HTTP method we use to call voice_fallback_url
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VoiceFallbackMethod {
    #[serde(rename = "HEAD")]
    HEAD,
    #[serde(rename = "GET")]
    GET,
    #[serde(rename = "POST")]
    POST,
    #[serde(rename = "PATCH")]
    PATCH,
    #[serde(rename = "PUT")]
    PUT,
    #[serde(rename = "DELETE")]
    DELETE,
}

impl Default for VoiceFallbackMethod {
    fn default() -> VoiceFallbackMethod {
        Self::HEAD
    }
}
/// Deprecated. The HTTP method we use to call voice_url
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum VoiceMethod {
    #[serde(rename = "HEAD")]
    HEAD,
    #[serde(rename = "GET")]
    GET,
    #[serde(rename = "POST")]
    POST,
    #[serde(rename = "PATCH")]
    PATCH,
    #[serde(rename = "PUT")]
    PUT,
    #[serde(rename = "DELETE")]
    DELETE,
}

impl Default for VoiceMethod {
    fn default() -> VoiceMethod {
        Self::HEAD
    }
}

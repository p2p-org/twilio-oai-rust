/*
 * Twilio - Trunking
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TrunkingV1TrunkPhoneNumber {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// Whether the phone number requires an Address registered with Twilio
    #[serde(
        rename = "address_requirements",
        skip_serializing_if = "Option::is_none"
    )]
    pub address_requirements: Option<AddressRequirements>,
    /// The API version used to start a new TwiML session
    #[serde(rename = "api_version", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// Whether the phone number is new to the Twilio platform
    #[serde(rename = "beta", skip_serializing_if = "Option::is_none")]
    pub beta: Option<bool>,
    /// Indicate if a phone can receive calls or messages
    #[serde(rename = "capabilities", skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<serde_json::Value>,
    /// The RFC 2822 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The string that you assigned to describe the resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The URLs of related resources
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// The phone number in E.164 format
    #[serde(rename = "phone_number", skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The SID of the application that handles SMS messages sent to the phone number
    #[serde(
        rename = "sms_application_sid",
        skip_serializing_if = "Option::is_none"
    )]
    pub sms_application_sid: Option<String>,
    /// The HTTP method used with sms_fallback_url
    #[serde(
        rename = "sms_fallback_method",
        skip_serializing_if = "Option::is_none"
    )]
    pub sms_fallback_method: Option<SmsFallbackMethod>,
    /// The URL that we call when an error occurs while retrieving or executing the TwiML
    #[serde(rename = "sms_fallback_url", skip_serializing_if = "Option::is_none")]
    pub sms_fallback_url: Option<String>,
    /// The HTTP method to use with sms_url
    #[serde(rename = "sms_method", skip_serializing_if = "Option::is_none")]
    pub sms_method: Option<SmsMethod>,
    /// The URL we call when the phone number receives an incoming SMS message
    #[serde(rename = "sms_url", skip_serializing_if = "Option::is_none")]
    pub sms_url: Option<String>,
    /// The URL to send status information to your application
    #[serde(rename = "status_callback", skip_serializing_if = "Option::is_none")]
    pub status_callback: Option<String>,
    /// The HTTP method we use to call status_callback
    #[serde(
        rename = "status_callback_method",
        skip_serializing_if = "Option::is_none"
    )]
    pub status_callback_method: Option<StatusCallbackMethod>,
    /// The SID of the Trunk that handles calls to the phone number
    #[serde(rename = "trunk_sid", skip_serializing_if = "Option::is_none")]
    pub trunk_sid: Option<String>,
    /// The absolute URL of the resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The SID of the application that handles calls to the phone number
    #[serde(
        rename = "voice_application_sid",
        skip_serializing_if = "Option::is_none"
    )]
    pub voice_application_sid: Option<String>,
    /// Whether to lookup the caller's name
    #[serde(
        rename = "voice_caller_id_lookup",
        skip_serializing_if = "Option::is_none"
    )]
    pub voice_caller_id_lookup: Option<bool>,
    /// The HTTP method that we use to call voice_fallback_url
    #[serde(
        rename = "voice_fallback_method",
        skip_serializing_if = "Option::is_none"
    )]
    pub voice_fallback_method: Option<VoiceFallbackMethod>,
    /// The URL we call when an error occurs in TwiML
    #[serde(rename = "voice_fallback_url", skip_serializing_if = "Option::is_none")]
    pub voice_fallback_url: Option<String>,
    /// The HTTP method used with the voice_url
    #[serde(rename = "voice_method", skip_serializing_if = "Option::is_none")]
    pub voice_method: Option<VoiceMethod>,
    /// The URL we call when the phone number receives a call
    #[serde(rename = "voice_url", skip_serializing_if = "Option::is_none")]
    pub voice_url: Option<String>,
}

impl TrunkingV1TrunkPhoneNumber {
    pub fn new() -> TrunkingV1TrunkPhoneNumber {
        TrunkingV1TrunkPhoneNumber {
            account_sid: None,
            address_requirements: None,
            api_version: None,
            beta: None,
            capabilities: None,
            date_created: None,
            date_updated: None,
            friendly_name: None,
            links: None,
            phone_number: None,
            sid: None,
            sms_application_sid: None,
            sms_fallback_method: None,
            sms_fallback_url: None,
            sms_method: None,
            sms_url: None,
            status_callback: None,
            status_callback_method: None,
            trunk_sid: None,
            url: None,
            voice_application_sid: None,
            voice_caller_id_lookup: None,
            voice_fallback_method: None,
            voice_fallback_url: None,
            voice_method: None,
            voice_url: None,
        }
    }
}

/// Whether the phone number requires an Address registered with Twilio
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AddressRequirements {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "foreign")]
    Foreign,
}

impl Default for AddressRequirements {
    fn default() -> AddressRequirements {
        Self::None
    }
}
/// The HTTP method used with sms_fallback_url
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
/// The HTTP method to use with sms_url
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
/// The HTTP method we use to call status_callback
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusCallbackMethod {
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

impl Default for StatusCallbackMethod {
    fn default() -> StatusCallbackMethod {
        Self::HEAD
    }
}
/// The HTTP method that we use to call voice_fallback_url
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
/// The HTTP method used with the voice_url
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

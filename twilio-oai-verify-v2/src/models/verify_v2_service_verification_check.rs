/*
 * Twilio - Verify
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VerifyV2ServiceVerificationCheck {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The amount of the associated PSD2 compliant transaction.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// The verification method to use
    #[serde(rename = "channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
    /// The ISO 8601 date and time in GMT when the Verification Check resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 date and time in GMT when the Verification Check resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The payee of the associated PSD2 compliant transaction
    #[serde(rename = "payee", skip_serializing_if = "Option::is_none")]
    pub payee: Option<String>,
    /// The SID of the Service that the resource is associated with
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The status of the verification resource
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// The phone number or email being verified
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// Whether the verification was successful
    #[serde(rename = "valid", skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}

impl VerifyV2ServiceVerificationCheck {
    pub fn new() -> VerifyV2ServiceVerificationCheck {
        VerifyV2ServiceVerificationCheck {
            account_sid: None,
            amount: None,
            channel: None,
            date_created: None,
            date_updated: None,
            payee: None,
            service_sid: None,
            sid: None,
            status: None,
            to: None,
            valid: None,
        }
    }
}

/// The verification method to use
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Channel {
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "call")]
    Call,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "whatsapp")]
    Whatsapp,
    #[serde(rename = "sna")]
    Sna,
}

impl Default for Channel {
    fn default() -> Channel {
        Self::Sms
    }
}

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
pub struct VerifyV2ServiceEntityChallenge {
    /// Account Sid.
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The date this Challenge was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The date this Challenge was responded
    #[serde(rename = "date_responded", skip_serializing_if = "Option::is_none")]
    pub date_responded: Option<String>,
    /// The date this Challenge was updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// Details about the Challenge.
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
    /// Entity Sid.
    #[serde(rename = "entity_sid", skip_serializing_if = "Option::is_none")]
    pub entity_sid: Option<String>,
    /// The date-time when this Challenge expires
    #[serde(rename = "expiration_date", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// Factor Sid.
    #[serde(rename = "factor_sid", skip_serializing_if = "Option::is_none")]
    pub factor_sid: Option<String>,
    /// The Factor Type of this Challenge
    #[serde(rename = "factor_type", skip_serializing_if = "Option::is_none")]
    pub factor_type: Option<FactorType>,
    /// Hidden details about the Challenge
    #[serde(rename = "hidden_details", skip_serializing_if = "Option::is_none")]
    pub hidden_details: Option<serde_json::Value>,
    /// Unique external identifier of the Entity
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// Nested resource URLs.
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// Metadata of the challenge.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// The Reason of this Challenge `status`
    #[serde(rename = "responded_reason", skip_serializing_if = "Option::is_none")]
    pub responded_reason: Option<RespondedReason>,
    /// Service Sid.
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    /// A string that uniquely identifies this Challenge.
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The Status of this Challenge
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The URL of this resource.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl VerifyV2ServiceEntityChallenge {
    pub fn new() -> VerifyV2ServiceEntityChallenge {
        VerifyV2ServiceEntityChallenge {
            account_sid: None,
            date_created: None,
            date_responded: None,
            date_updated: None,
            details: None,
            entity_sid: None,
            expiration_date: None,
            factor_sid: None,
            factor_type: None,
            hidden_details: None,
            identity: None,
            links: None,
            metadata: None,
            responded_reason: None,
            service_sid: None,
            sid: None,
            status: None,
            url: None,
        }
    }
}

/// The Factor Type of this Challenge
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FactorType {
    #[serde(rename = "push")]
    Push,
    #[serde(rename = "totp")]
    Totp,
}

impl Default for FactorType {
    fn default() -> FactorType {
        Self::Push
    }
}
/// The Reason of this Challenge `status`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RespondedReason {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "not_needed")]
    NotNeeded,
    #[serde(rename = "not_requested")]
    NotRequested,
}

impl Default for RespondedReason {
    fn default() -> RespondedReason {
        Self::None
    }
}
/// The Status of this Challenge
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "denied")]
    Denied,
}

impl Default for Status {
    fn default() -> Status {
        Self::Pending
    }
}

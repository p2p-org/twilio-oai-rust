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
pub struct VerifyV2ServiceEntityNewFactor {
    /// Account Sid.
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// Binding of the factor
    #[serde(rename = "binding", skip_serializing_if = "Option::is_none")]
    pub binding: Option<serde_json::Value>,
    /// Configurations for a `factor_type`.
    #[serde(rename = "config", skip_serializing_if = "Option::is_none")]
    pub config: Option<serde_json::Value>,
    /// The date this Factor was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The date this Factor was updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// Entity Sid.
    #[serde(rename = "entity_sid", skip_serializing_if = "Option::is_none")]
    pub entity_sid: Option<String>,
    /// The Type of this Factor
    #[serde(rename = "factor_type", skip_serializing_if = "Option::is_none")]
    pub factor_type: Option<FactorType>,
    /// A human readable description of this resource.
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// Unique external identifier of the Entity
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// Metadata of the factor.
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// Service Sid.
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    /// A string that uniquely identifies this Factor.
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The Status of this Factor
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The URL of this resource.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl VerifyV2ServiceEntityNewFactor {
    pub fn new() -> VerifyV2ServiceEntityNewFactor {
        VerifyV2ServiceEntityNewFactor {
            account_sid: None,
            binding: None,
            config: None,
            date_created: None,
            date_updated: None,
            entity_sid: None,
            factor_type: None,
            friendly_name: None,
            identity: None,
            metadata: None,
            service_sid: None,
            sid: None,
            status: None,
            url: None,
        }
    }
}

/// The Type of this Factor
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
/// The Status of this Factor
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "unverified")]
    Unverified,
    #[serde(rename = "verified")]
    Verified,
}

impl Default for Status {
    fn default() -> Status {
        Self::Unverified
    }
}

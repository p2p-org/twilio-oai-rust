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
pub struct VerifyV2ServiceAccessToken {
    /// Account Sid.
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The date this access token was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// Unique external identifier of the Entity
    #[serde(rename = "entity_identity", skip_serializing_if = "Option::is_none")]
    pub entity_identity: Option<String>,
    /// A human readable description of this factor.
    #[serde(
        rename = "factor_friendly_name",
        skip_serializing_if = "Option::is_none"
    )]
    pub factor_friendly_name: Option<String>,
    /// The Type of the Factor
    #[serde(rename = "factor_type", skip_serializing_if = "Option::is_none")]
    pub factor_type: Option<FactorType>,
    /// Verify Service Sid.
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    /// A string that uniquely identifies this Access Token.
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// Generated access token.
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// How long, in seconds, the access token is valid.
    #[serde(rename = "ttl", skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i32>,
    /// The URL of this resource.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl VerifyV2ServiceAccessToken {
    pub fn new() -> VerifyV2ServiceAccessToken {
        VerifyV2ServiceAccessToken {
            account_sid: None,
            date_created: None,
            entity_identity: None,
            factor_friendly_name: None,
            factor_type: None,
            service_sid: None,
            sid: None,
            token: None,
            ttl: None,
            url: None,
        }
    }
}

/// The Type of the Factor
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FactorType {
    #[serde(rename = "push")]
    Push,
}

impl Default for FactorType {
    fn default() -> FactorType {
        Self::Push
    }
}

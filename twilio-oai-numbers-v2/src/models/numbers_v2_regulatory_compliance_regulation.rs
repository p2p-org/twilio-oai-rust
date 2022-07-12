/*
 * Twilio - Numbers
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NumbersV2RegulatoryComplianceRegulation {
    /// The type of End User of the Regulation resource
    #[serde(rename = "end_user_type", skip_serializing_if = "Option::is_none")]
    pub end_user_type: Option<EndUserType>,
    /// A human-readable description of the Regulation resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The ISO country code of the phone number's country
    #[serde(rename = "iso_country", skip_serializing_if = "Option::is_none")]
    pub iso_country: Option<String>,
    /// The type of phone number restricted by the regulatory requirement
    #[serde(rename = "number_type", skip_serializing_if = "Option::is_none")]
    pub number_type: Option<String>,
    /// The sid of a regulation object that dictates requirements
    #[serde(rename = "requirements", skip_serializing_if = "Option::is_none")]
    pub requirements: Option<serde_json::Value>,
    /// The unique string that identifies the Regulation resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The absolute URL of the Regulation resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl NumbersV2RegulatoryComplianceRegulation {
    pub fn new() -> NumbersV2RegulatoryComplianceRegulation {
        NumbersV2RegulatoryComplianceRegulation {
            end_user_type: None,
            friendly_name: None,
            iso_country: None,
            number_type: None,
            requirements: None,
            sid: None,
            url: None,
        }
    }
}

/// The type of End User of the Regulation resource
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EndUserType {
    #[serde(rename = "individual")]
    Individual,
    #[serde(rename = "business")]
    Business,
}

impl Default for EndUserType {
    fn default() -> EndUserType {
        Self::Individual
    }
}

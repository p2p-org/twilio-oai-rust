/*
 * Twilio - Voice
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VoiceV1DialingPermissionsDialingPermissionsCountryInstance {
    /// The name of the continent in which the country is located
    #[serde(rename = "continent", skip_serializing_if = "Option::is_none")]
    pub continent: Option<String>,
    /// The E.164 assigned country codes(s)
    #[serde(rename = "country_codes", skip_serializing_if = "Option::is_none")]
    pub country_codes: Option<Vec<String>>,
    /// Whether dialing to high-risk special services numbers is enabled
    #[serde(
        rename = "high_risk_special_numbers_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub high_risk_special_numbers_enabled: Option<bool>,
    /// Whether dialing to high-risk toll fraud numbers is enabled, else `false`
    #[serde(
        rename = "high_risk_tollfraud_numbers_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub high_risk_tollfraud_numbers_enabled: Option<bool>,
    /// The ISO country code
    #[serde(rename = "iso_code", skip_serializing_if = "Option::is_none")]
    pub iso_code: Option<String>,
    /// A list of URLs related to this resource
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// Whether dialing to low-risk numbers is enabled
    #[serde(
        rename = "low_risk_numbers_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub low_risk_numbers_enabled: Option<bool>,
    /// The name of the country
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The absolute URL of this resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl VoiceV1DialingPermissionsDialingPermissionsCountryInstance {
    pub fn new() -> VoiceV1DialingPermissionsDialingPermissionsCountryInstance {
        VoiceV1DialingPermissionsDialingPermissionsCountryInstance {
            continent: None,
            country_codes: None,
            high_risk_special_numbers_enabled: None,
            high_risk_tollfraud_numbers_enabled: None,
            iso_code: None,
            links: None,
            low_risk_numbers_enabled: None,
            name: None,
            url: None,
        }
    }
}

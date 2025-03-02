/*
 * Twilio - Insights
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InsightsV1AccountSettings {
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    #[serde(rename = "advanced_features", skip_serializing_if = "Option::is_none")]
    pub advanced_features: Option<bool>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "voice_trace", skip_serializing_if = "Option::is_none")]
    pub voice_trace: Option<bool>,
}

impl InsightsV1AccountSettings {
    pub fn new() -> InsightsV1AccountSettings {
        InsightsV1AccountSettings {
            account_sid: None,
            advanced_features: None,
            url: None,
            voice_trace: None,
        }
    }
}

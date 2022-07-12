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
pub struct VerifyV2VerificationAttemptsSummary {
    /// Percentage of the confirmed messages over the total.
    #[serde(
        rename = "conversion_rate_percentage",
        skip_serializing_if = "Option::is_none"
    )]
    pub conversion_rate_percentage: Option<f32>,
    /// Total of attempts made.
    #[serde(rename = "total_attempts", skip_serializing_if = "Option::is_none")]
    pub total_attempts: Option<i32>,
    /// Total of attempts confirmed by the end user.
    #[serde(rename = "total_converted", skip_serializing_if = "Option::is_none")]
    pub total_converted: Option<i32>,
    /// Total of attempts made that were not confirmed by the end user.
    #[serde(rename = "total_unconverted", skip_serializing_if = "Option::is_none")]
    pub total_unconverted: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl VerifyV2VerificationAttemptsSummary {
    pub fn new() -> VerifyV2VerificationAttemptsSummary {
        VerifyV2VerificationAttemptsSummary {
            conversion_rate_percentage: None,
            total_attempts: None,
            total_converted: None,
            total_unconverted: None,
            url: None,
        }
    }
}

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
pub struct ListFactorResponse {
    #[serde(rename = "factors", skip_serializing_if = "Option::is_none")]
    pub factors: Option<Vec<crate::models::VerifyV2ServiceEntityFactor>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListVerificationAttemptResponseMeta>>,
}

impl ListFactorResponse {
    pub fn new() -> ListFactorResponse {
        ListFactorResponse {
            factors: None,
            meta: None,
        }
    }
}

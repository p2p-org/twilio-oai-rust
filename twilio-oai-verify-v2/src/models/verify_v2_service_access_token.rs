/*
 * Twilio - Verify
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct VerifyV2ServiceAccessToken {
    /// Generated access token.
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl VerifyV2ServiceAccessToken {
    pub fn new() -> VerifyV2ServiceAccessToken {
        VerifyV2ServiceAccessToken {
            token: None,
        }
    }
}



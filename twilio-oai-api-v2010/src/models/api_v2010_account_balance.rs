/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiV2010AccountBalance {
    /// Account Sid.
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// Account balance
    #[serde(rename = "balance", skip_serializing_if = "Option::is_none")]
    pub balance: Option<String>,
    /// Currency units
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl ApiV2010AccountBalance {
    pub fn new() -> ApiV2010AccountBalance {
        ApiV2010AccountBalance {
            account_sid: None,
            balance: None,
            currency: None,
        }
    }
}

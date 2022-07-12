/*
 * Twilio - Wireless
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WirelessV1AccountUsageRecord {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// An object that describes the aggregated Commands usage for all SIMs during the specified period
    #[serde(rename = "commands", skip_serializing_if = "Option::is_none")]
    pub commands: Option<serde_json::Value>,
    /// An object that describes the aggregated Data usage for all SIMs over the period
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// The time period for which usage is reported
    #[serde(rename = "period", skip_serializing_if = "Option::is_none")]
    pub period: Option<serde_json::Value>,
}

impl WirelessV1AccountUsageRecord {
    pub fn new() -> WirelessV1AccountUsageRecord {
        WirelessV1AccountUsageRecord {
            account_sid: None,
            commands: None,
            data: None,
            period: None,
        }
    }
}

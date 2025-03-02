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
pub struct InsightsV1CallEvent {
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    #[serde(rename = "call_sid", skip_serializing_if = "Option::is_none")]
    pub call_sid: Option<String>,
    #[serde(rename = "carrier_edge", skip_serializing_if = "Option::is_none")]
    pub carrier_edge: Option<serde_json::Value>,
    #[serde(rename = "client_edge", skip_serializing_if = "Option::is_none")]
    pub client_edge: Option<serde_json::Value>,
    #[serde(rename = "edge", skip_serializing_if = "Option::is_none")]
    pub edge: Option<Edge>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "level", skip_serializing_if = "Option::is_none")]
    pub level: Option<Level>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "sdk_edge", skip_serializing_if = "Option::is_none")]
    pub sdk_edge: Option<serde_json::Value>,
    #[serde(rename = "sip_edge", skip_serializing_if = "Option::is_none")]
    pub sip_edge: Option<serde_json::Value>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl InsightsV1CallEvent {
    pub fn new() -> InsightsV1CallEvent {
        InsightsV1CallEvent {
            account_sid: None,
            call_sid: None,
            carrier_edge: None,
            client_edge: None,
            edge: None,
            group: None,
            level: None,
            name: None,
            sdk_edge: None,
            sip_edge: None,
            timestamp: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Edge {
    #[serde(rename = "unknown_edge")]
    UnknownEdge,
    #[serde(rename = "carrier_edge")]
    CarrierEdge,
    #[serde(rename = "sip_edge")]
    SipEdge,
    #[serde(rename = "sdk_edge")]
    SdkEdge,
    #[serde(rename = "client_edge")]
    ClientEdge,
}

impl Default for Edge {
    fn default() -> Edge {
        Self::UnknownEdge
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Level {
    #[serde(rename = "UNKNOWN")]
    UNKNOWN,
    #[serde(rename = "DEBUG")]
    DEBUG,
    #[serde(rename = "INFO")]
    INFO,
    #[serde(rename = "WARNING")]
    WARNING,
    #[serde(rename = "ERROR")]
    ERROR,
}

impl Default for Level {
    fn default() -> Level {
        Self::UNKNOWN
    }
}

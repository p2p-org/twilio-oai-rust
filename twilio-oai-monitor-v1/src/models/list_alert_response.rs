/*
 * Twilio - Monitor
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListAlertResponse {
    #[serde(rename = "alerts", skip_serializing_if = "Option::is_none")]
    pub alerts: Option<Vec<crate::models::MonitorV1Alert>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListAlertResponseMeta>>,
}

impl ListAlertResponse {
    pub fn new() -> ListAlertResponse {
        ListAlertResponse {
            alerts: None,
            meta: None,
        }
    }
}

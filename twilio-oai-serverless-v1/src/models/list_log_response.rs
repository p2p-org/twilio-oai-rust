/*
 * Twilio - Serverless
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListLogResponse {
    #[serde(rename = "logs", skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<crate::models::ServerlessV1ServiceEnvironmentLog>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListServiceResponseMeta>>,
}

impl ListLogResponse {
    pub fn new() -> ListLogResponse {
        ListLogResponse {
            logs: None,
            meta: None,
        }
    }
}

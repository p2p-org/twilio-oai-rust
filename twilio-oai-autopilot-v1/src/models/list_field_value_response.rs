/*
 * Twilio - Autopilot
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListFieldValueResponse {
    #[serde(rename = "field_values", skip_serializing_if = "Option::is_none")]
    pub field_values: Option<Vec<crate::models::AutopilotV1AssistantFieldTypeFieldValue>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListAssistantResponseMeta>>,
}

impl ListFieldValueResponse {
    pub fn new() -> ListFieldValueResponse {
        ListFieldValueResponse {
            field_values: None,
            meta: None,
        }
    }
}

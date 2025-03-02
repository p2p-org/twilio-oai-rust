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
pub struct ListModelBuildResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListAssistantResponseMeta>>,
    #[serde(rename = "model_builds", skip_serializing_if = "Option::is_none")]
    pub model_builds: Option<Vec<crate::models::AutopilotV1AssistantModelBuild>>,
}

impl ListModelBuildResponse {
    pub fn new() -> ListModelBuildResponse {
        ListModelBuildResponse {
            meta: None,
            model_builds: None,
        }
    }
}

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
pub struct AutopilotV1AssistantTaskTaskActions {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The SID of the Assistant that is the parent of the Task associated with the resource
    #[serde(rename = "assistant_sid", skip_serializing_if = "Option::is_none")]
    pub assistant_sid: Option<String>,
    /// The JSON string that specifies the actions that instruct the Assistant on how to perform the task
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
    /// The SID of the Task associated with the resource
    #[serde(rename = "task_sid", skip_serializing_if = "Option::is_none")]
    pub task_sid: Option<String>,
    /// The absolute URL of the TaskActions resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl AutopilotV1AssistantTaskTaskActions {
    pub fn new() -> AutopilotV1AssistantTaskTaskActions {
        AutopilotV1AssistantTaskTaskActions {
            account_sid: None,
            assistant_sid: None,
            data: None,
            task_sid: None,
            url: None,
        }
    }
}

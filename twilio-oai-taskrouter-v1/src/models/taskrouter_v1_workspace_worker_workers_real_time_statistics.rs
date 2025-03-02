/*
 * Twilio - Taskrouter
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TaskrouterV1WorkspaceWorkerWorkersRealTimeStatistics {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The number of current Workers by Activity
    #[serde(
        rename = "activity_statistics",
        skip_serializing_if = "Option::is_none"
    )]
    pub activity_statistics: Option<Vec<serde_json::Value>>,
    /// The total number of Workers
    #[serde(rename = "total_workers", skip_serializing_if = "Option::is_none")]
    pub total_workers: Option<i32>,
    /// The absolute URL of the Workers statistics resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The SID of the Workspace that contains the Workers
    #[serde(rename = "workspace_sid", skip_serializing_if = "Option::is_none")]
    pub workspace_sid: Option<String>,
}

impl TaskrouterV1WorkspaceWorkerWorkersRealTimeStatistics {
    pub fn new() -> TaskrouterV1WorkspaceWorkerWorkersRealTimeStatistics {
        TaskrouterV1WorkspaceWorkerWorkersRealTimeStatistics {
            account_sid: None,
            activity_statistics: None,
            total_workers: None,
            url: None,
            workspace_sid: None,
        }
    }
}

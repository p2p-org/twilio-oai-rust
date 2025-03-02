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
pub struct TaskrouterV1WorkspaceWorkspaceRealTimeStatistics {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The number of current Workers by Activity
    #[serde(
        rename = "activity_statistics",
        skip_serializing_if = "Option::is_none"
    )]
    pub activity_statistics: Option<Vec<serde_json::Value>>,
    /// The age of the longest waiting Task
    #[serde(
        rename = "longest_task_waiting_age",
        skip_serializing_if = "Option::is_none"
    )]
    pub longest_task_waiting_age: Option<i32>,
    /// The SID of the longest waiting Task
    #[serde(
        rename = "longest_task_waiting_sid",
        skip_serializing_if = "Option::is_none"
    )]
    pub longest_task_waiting_sid: Option<String>,
    /// The number of Tasks by priority
    #[serde(rename = "tasks_by_priority", skip_serializing_if = "Option::is_none")]
    pub tasks_by_priority: Option<serde_json::Value>,
    /// The number of Tasks by their current status
    #[serde(rename = "tasks_by_status", skip_serializing_if = "Option::is_none")]
    pub tasks_by_status: Option<serde_json::Value>,
    /// The total number of Tasks
    #[serde(rename = "total_tasks", skip_serializing_if = "Option::is_none")]
    pub total_tasks: Option<i32>,
    /// The total number of Workers in the Workspace
    #[serde(rename = "total_workers", skip_serializing_if = "Option::is_none")]
    pub total_workers: Option<i32>,
    /// The absolute URL of the Workspace statistics resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The SID of the Workspace
    #[serde(rename = "workspace_sid", skip_serializing_if = "Option::is_none")]
    pub workspace_sid: Option<String>,
}

impl TaskrouterV1WorkspaceWorkspaceRealTimeStatistics {
    pub fn new() -> TaskrouterV1WorkspaceWorkspaceRealTimeStatistics {
        TaskrouterV1WorkspaceWorkspaceRealTimeStatistics {
            account_sid: None,
            activity_statistics: None,
            longest_task_waiting_age: None,
            longest_task_waiting_sid: None,
            tasks_by_priority: None,
            tasks_by_status: None,
            total_tasks: None,
            total_workers: None,
            url: None,
            workspace_sid: None,
        }
    }
}

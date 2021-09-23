/*
 * Twilio - Taskrouter
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct TaskrouterV1WorkspaceWorkerWorkerChannel {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The total number of Tasks assigned to Worker for the TaskChannel type
    #[serde(rename = "assigned_tasks", skip_serializing_if = "Option::is_none")]
    pub assigned_tasks: Option<i32>,
    /// Whether the Worker should receive Tasks of the TaskChannel type
    #[serde(rename = "available", skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    /// The current available capacity between 0 to 100 for the TaskChannel
    #[serde(rename = "available_capacity_percentage", skip_serializing_if = "Option::is_none")]
    pub available_capacity_percentage: Option<i32>,
    /// The current configured capacity for the WorkerChannel
    #[serde(rename = "configured_capacity", skip_serializing_if = "Option::is_none")]
    pub configured_capacity: Option<i32>,
    /// The RFC 2822 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The SID of the TaskChannel
    #[serde(rename = "task_channel_sid", skip_serializing_if = "Option::is_none")]
    pub task_channel_sid: Option<String>,
    /// The unique name of the TaskChannel, such as 'voice' or 'sms'
    #[serde(rename = "task_channel_unique_name", skip_serializing_if = "Option::is_none")]
    pub task_channel_unique_name: Option<String>,
    /// The absolute URL of the WorkerChannel resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The SID of the Worker that contains the WorkerChannel
    #[serde(rename = "worker_sid", skip_serializing_if = "Option::is_none")]
    pub worker_sid: Option<String>,
    /// The SID of the Workspace that contains the WorkerChannel
    #[serde(rename = "workspace_sid", skip_serializing_if = "Option::is_none")]
    pub workspace_sid: Option<String>,
}

impl TaskrouterV1WorkspaceWorkerWorkerChannel {
    pub fn new() -> TaskrouterV1WorkspaceWorkerWorkerChannel {
        TaskrouterV1WorkspaceWorkerWorkerChannel {
            account_sid: None,
            assigned_tasks: None,
            available: None,
            available_capacity_percentage: None,
            configured_capacity: None,
            date_created: None,
            date_updated: None,
            sid: None,
            task_channel_sid: None,
            task_channel_unique_name: None,
            url: None,
            worker_sid: None,
            workspace_sid: None,
        }
    }
}



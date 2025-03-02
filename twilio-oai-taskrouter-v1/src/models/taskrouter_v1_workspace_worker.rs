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
pub struct TaskrouterV1WorkspaceWorker {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The friendly_name of the Worker's current Activity
    #[serde(rename = "activity_name", skip_serializing_if = "Option::is_none")]
    pub activity_name: Option<String>,
    /// The SID of the Worker's current Activity
    #[serde(rename = "activity_sid", skip_serializing_if = "Option::is_none")]
    pub activity_sid: Option<String>,
    /// The JSON string that describes the Worker
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    /// Whether the Worker is available to perform tasks
    #[serde(rename = "available", skip_serializing_if = "Option::is_none")]
    pub available: Option<bool>,
    /// The ISO 8601 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The date and time in GMT of the last change to the Worker's activity
    #[serde(
        rename = "date_status_changed",
        skip_serializing_if = "Option::is_none"
    )]
    pub date_status_changed: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The string that you assigned to describe the resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The URLs of related resources
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The absolute URL of the Worker resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The SID of the Workspace that contains the Worker
    #[serde(rename = "workspace_sid", skip_serializing_if = "Option::is_none")]
    pub workspace_sid: Option<String>,
}

impl TaskrouterV1WorkspaceWorker {
    pub fn new() -> TaskrouterV1WorkspaceWorker {
        TaskrouterV1WorkspaceWorker {
            account_sid: None,
            activity_name: None,
            activity_sid: None,
            attributes: None,
            available: None,
            date_created: None,
            date_status_changed: None,
            date_updated: None,
            friendly_name: None,
            links: None,
            sid: None,
            url: None,
            workspace_sid: None,
        }
    }
}

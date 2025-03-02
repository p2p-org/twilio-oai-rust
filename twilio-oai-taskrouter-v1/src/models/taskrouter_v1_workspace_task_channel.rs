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
pub struct TaskrouterV1WorkspaceTaskChannel {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// Whether the Task Channel will prioritize Workers that have been idle
    #[serde(
        rename = "channel_optimized_routing",
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_optimized_routing: Option<bool>,
    /// The ISO 8601 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
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
    /// An application-defined string that uniquely identifies the Task Channel
    #[serde(rename = "unique_name", skip_serializing_if = "Option::is_none")]
    pub unique_name: Option<String>,
    /// The absolute URL of the Task Channel resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The SID of the Workspace that contains the Task Channel
    #[serde(rename = "workspace_sid", skip_serializing_if = "Option::is_none")]
    pub workspace_sid: Option<String>,
}

impl TaskrouterV1WorkspaceTaskChannel {
    pub fn new() -> TaskrouterV1WorkspaceTaskChannel {
        TaskrouterV1WorkspaceTaskChannel {
            account_sid: None,
            channel_optimized_routing: None,
            date_created: None,
            date_updated: None,
            friendly_name: None,
            links: None,
            sid: None,
            unique_name: None,
            url: None,
            workspace_sid: None,
        }
    }
}

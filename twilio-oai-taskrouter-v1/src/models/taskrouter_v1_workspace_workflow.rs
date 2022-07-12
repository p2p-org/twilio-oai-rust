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
pub struct TaskrouterV1WorkspaceWorkflow {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The URL that we call when a task managed by the Workflow is assigned to a Worker
    #[serde(
        rename = "assignment_callback_url",
        skip_serializing_if = "Option::is_none"
    )]
    pub assignment_callback_url: Option<String>,
    /// A JSON string that contains the Workflow's configuration
    #[serde(rename = "configuration", skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The MIME type of the document
    #[serde(
        rename = "document_content_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub document_content_type: Option<String>,
    /// The URL that we call when a call to the `assignment_callback_url` fails
    #[serde(
        rename = "fallback_assignment_callback_url",
        skip_serializing_if = "Option::is_none"
    )]
    pub fallback_assignment_callback_url: Option<String>,
    /// The string that you assigned to describe the Workflow resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The URLs of related resources
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// How long TaskRouter will wait for a confirmation response from your application after it assigns a Task to a Worker
    #[serde(
        rename = "task_reservation_timeout",
        skip_serializing_if = "Option::is_none"
    )]
    pub task_reservation_timeout: Option<i32>,
    /// The absolute URL of the Workflow resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// The SID of the Workspace that contains the Workflow
    #[serde(rename = "workspace_sid", skip_serializing_if = "Option::is_none")]
    pub workspace_sid: Option<String>,
}

impl TaskrouterV1WorkspaceWorkflow {
    pub fn new() -> TaskrouterV1WorkspaceWorkflow {
        TaskrouterV1WorkspaceWorkflow {
            account_sid: None,
            assignment_callback_url: None,
            configuration: None,
            date_created: None,
            date_updated: None,
            document_content_type: None,
            fallback_assignment_callback_url: None,
            friendly_name: None,
            links: None,
            sid: None,
            task_reservation_timeout: None,
            url: None,
            workspace_sid: None,
        }
    }
}

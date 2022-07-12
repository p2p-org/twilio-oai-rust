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
pub struct AutopilotV1Assistant {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// Reserved
    #[serde(rename = "callback_events", skip_serializing_if = "Option::is_none")]
    pub callback_events: Option<String>,
    /// Reserved
    #[serde(rename = "callback_url", skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// A string describing the state of the assistant.
    #[serde(rename = "development_stage", skip_serializing_if = "Option::is_none")]
    pub development_stage: Option<String>,
    /// The string that you assigned to describe the resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// Reserved
    #[serde(
        rename = "latest_model_build_sid",
        skip_serializing_if = "Option::is_none"
    )]
    pub latest_model_build_sid: Option<String>,
    /// A list of the URLs of the Assistant's related resources
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// Whether queries should be logged and kept after training
    #[serde(rename = "log_queries", skip_serializing_if = "Option::is_none")]
    pub log_queries: Option<bool>,
    /// Whether model needs to be rebuilt
    #[serde(rename = "needs_model_build", skip_serializing_if = "Option::is_none")]
    pub needs_model_build: Option<bool>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// An application-defined string that uniquely identifies the resource
    #[serde(rename = "unique_name", skip_serializing_if = "Option::is_none")]
    pub unique_name: Option<String>,
    /// The absolute URL of the Assistant resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl AutopilotV1Assistant {
    pub fn new() -> AutopilotV1Assistant {
        AutopilotV1Assistant {
            account_sid: None,
            callback_events: None,
            callback_url: None,
            date_created: None,
            date_updated: None,
            development_stage: None,
            friendly_name: None,
            latest_model_build_sid: None,
            links: None,
            log_queries: None,
            needs_model_build: None,
            sid: None,
            unique_name: None,
            url: None,
        }
    }
}

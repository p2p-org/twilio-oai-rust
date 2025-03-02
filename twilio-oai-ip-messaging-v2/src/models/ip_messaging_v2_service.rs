/*
 * Twilio - Ip_messaging
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct IpMessagingV2Service {
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    #[serde(
        rename = "consumption_report_interval",
        skip_serializing_if = "Option::is_none"
    )]
    pub consumption_report_interval: Option<i32>,
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    #[serde(
        rename = "default_channel_creator_role_sid",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_channel_creator_role_sid: Option<String>,
    #[serde(
        rename = "default_channel_role_sid",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_channel_role_sid: Option<String>,
    #[serde(
        rename = "default_service_role_sid",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_service_role_sid: Option<String>,
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "limits", skip_serializing_if = "Option::is_none")]
    pub limits: Option<serde_json::Value>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(rename = "media", skip_serializing_if = "Option::is_none")]
    pub media: Option<serde_json::Value>,
    #[serde(rename = "notifications", skip_serializing_if = "Option::is_none")]
    pub notifications: Option<serde_json::Value>,
    #[serde(
        rename = "post_webhook_retry_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub post_webhook_retry_count: Option<i32>,
    #[serde(rename = "post_webhook_url", skip_serializing_if = "Option::is_none")]
    pub post_webhook_url: Option<String>,
    #[serde(
        rename = "pre_webhook_retry_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub pre_webhook_retry_count: Option<i32>,
    #[serde(rename = "pre_webhook_url", skip_serializing_if = "Option::is_none")]
    pub pre_webhook_url: Option<String>,
    #[serde(
        rename = "reachability_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub reachability_enabled: Option<bool>,
    #[serde(
        rename = "read_status_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub read_status_enabled: Option<bool>,
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[serde(
        rename = "typing_indicator_timeout",
        skip_serializing_if = "Option::is_none"
    )]
    pub typing_indicator_timeout: Option<i32>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "webhook_filters", skip_serializing_if = "Option::is_none")]
    pub webhook_filters: Option<Vec<String>>,
    #[serde(rename = "webhook_method", skip_serializing_if = "Option::is_none")]
    pub webhook_method: Option<String>,
}

impl IpMessagingV2Service {
    pub fn new() -> IpMessagingV2Service {
        IpMessagingV2Service {
            account_sid: None,
            consumption_report_interval: None,
            date_created: None,
            date_updated: None,
            default_channel_creator_role_sid: None,
            default_channel_role_sid: None,
            default_service_role_sid: None,
            friendly_name: None,
            limits: None,
            links: None,
            media: None,
            notifications: None,
            post_webhook_retry_count: None,
            post_webhook_url: None,
            pre_webhook_retry_count: None,
            pre_webhook_url: None,
            reachability_enabled: None,
            read_status_enabled: None,
            sid: None,
            typing_indicator_timeout: None,
            url: None,
            webhook_filters: None,
            webhook_method: None,
        }
    }
}

/*
 * Twilio - Bulkexports
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct BulkexportsV1ExportConfiguration {
    /// Whether files are automatically generated
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// The type of communication – Messages, Calls, Conferences, and Participants
    #[serde(rename = "resource_type", skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// The URL of this resource.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Whether to GET or POST to the webhook url
    #[serde(rename = "webhook_method", skip_serializing_if = "Option::is_none")]
    pub webhook_method: Option<String>,
    /// URL targeted at export
    #[serde(rename = "webhook_url", skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
}

impl BulkexportsV1ExportConfiguration {
    pub fn new() -> BulkexportsV1ExportConfiguration {
        BulkexportsV1ExportConfiguration {
            enabled: None,
            resource_type: None,
            url: None,
            webhook_method: None,
            webhook_url: None,
        }
    }
}

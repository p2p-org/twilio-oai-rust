/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiV2010AccountQueue {
    /// The SID of the Account that created this resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// Average wait time of members in the queue
    #[serde(rename = "average_wait_time", skip_serializing_if = "Option::is_none")]
    pub average_wait_time: Option<i32>,
    /// The number of calls currently in the queue.
    #[serde(rename = "current_size", skip_serializing_if = "Option::is_none")]
    pub current_size: Option<i32>,
    /// The RFC 2822 date and time in GMT that this resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT that this resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// A string that you assigned to describe this resource
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The max number of calls allowed in the queue
    #[serde(rename = "max_size", skip_serializing_if = "Option::is_none")]
    pub max_size: Option<i32>,
    /// The unique string that identifies this resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The URI of this resource, relative to `https://api.twilio.com`
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl ApiV2010AccountQueue {
    pub fn new() -> ApiV2010AccountQueue {
        ApiV2010AccountQueue {
            account_sid: None,
            average_wait_time: None,
            current_size: None,
            date_created: None,
            date_updated: None,
            friendly_name: None,
            max_size: None,
            sid: None,
            uri: None,
        }
    }
}

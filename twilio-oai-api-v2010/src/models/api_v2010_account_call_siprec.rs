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
pub struct ApiV2010AccountCallSiprec {
    /// The SID of the Account that created this resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The SID of the Call the resource is associated with
    #[serde(rename = "call_sid", skip_serializing_if = "Option::is_none")]
    pub call_sid: Option<String>,
    /// The RFC 2822 date and time in GMT that this resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The name of this resource
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The SID of the Siprec resource.
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The status - one of `stopped`, `in-progress`
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The URI of the resource, relative to `https://api.twilio.com`
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl ApiV2010AccountCallSiprec {
    pub fn new() -> ApiV2010AccountCallSiprec {
        ApiV2010AccountCallSiprec {
            account_sid: None,
            call_sid: None,
            date_updated: None,
            name: None,
            sid: None,
            status: None,
            uri: None,
        }
    }
}

/// The status - one of `stopped`, `in-progress`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "in-progress")]
    InProgress,
    #[serde(rename = "stopped")]
    Stopped,
}

impl Default for Status {
    fn default() -> Status {
        Self::InProgress
    }
}

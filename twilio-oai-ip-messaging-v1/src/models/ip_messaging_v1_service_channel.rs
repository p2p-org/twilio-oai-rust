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
pub struct IpMessagingV1ServiceChannel {
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    #[serde(rename = "attributes", skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(rename = "members_count", skip_serializing_if = "Option::is_none")]
    pub members_count: Option<i32>,
    #[serde(rename = "messages_count", skip_serializing_if = "Option::is_none")]
    pub messages_count: Option<i32>,
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    #[serde(rename = "unique_name", skip_serializing_if = "Option::is_none")]
    pub unique_name: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl IpMessagingV1ServiceChannel {
    pub fn new() -> IpMessagingV1ServiceChannel {
        IpMessagingV1ServiceChannel {
            account_sid: None,
            attributes: None,
            created_by: None,
            date_created: None,
            date_updated: None,
            friendly_name: None,
            links: None,
            members_count: None,
            messages_count: None,
            service_sid: None,
            sid: None,
            _type: None,
            unique_name: None,
            url: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "public")]
    Public,
    #[serde(rename = "private")]
    Private,
}

impl Default for Type {
    fn default() -> Type {
        Self::Public
    }
}

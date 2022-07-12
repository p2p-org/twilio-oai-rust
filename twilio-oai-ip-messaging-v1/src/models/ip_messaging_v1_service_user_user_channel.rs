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
pub struct IpMessagingV1ServiceUserUserChannel {
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    #[serde(rename = "channel_sid", skip_serializing_if = "Option::is_none")]
    pub channel_sid: Option<String>,
    #[serde(
        rename = "last_consumed_message_index",
        skip_serializing_if = "Option::is_none"
    )]
    pub last_consumed_message_index: Option<i32>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    #[serde(rename = "member_sid", skip_serializing_if = "Option::is_none")]
    pub member_sid: Option<String>,
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    #[serde(
        rename = "unread_messages_count",
        skip_serializing_if = "Option::is_none"
    )]
    pub unread_messages_count: Option<i32>,
}

impl IpMessagingV1ServiceUserUserChannel {
    pub fn new() -> IpMessagingV1ServiceUserUserChannel {
        IpMessagingV1ServiceUserUserChannel {
            account_sid: None,
            channel_sid: None,
            last_consumed_message_index: None,
            links: None,
            member_sid: None,
            service_sid: None,
            status: None,
            unread_messages_count: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "joined")]
    Joined,
    #[serde(rename = "invited")]
    Invited,
    #[serde(rename = "not_participating")]
    NotParticipating,
}

impl Default for Status {
    fn default() -> Status {
        Self::Joined
    }
}

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
pub struct IpMessagingV2ServiceChannelInvite {
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    #[serde(rename = "channel_sid", skip_serializing_if = "Option::is_none")]
    pub channel_sid: Option<String>,
    #[serde(rename = "created_by", skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    #[serde(rename = "role_sid", skip_serializing_if = "Option::is_none")]
    pub role_sid: Option<String>,
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl IpMessagingV2ServiceChannelInvite {
    pub fn new() -> IpMessagingV2ServiceChannelInvite {
        IpMessagingV2ServiceChannelInvite {
            account_sid: None,
            channel_sid: None,
            created_by: None,
            date_created: None,
            date_updated: None,
            identity: None,
            role_sid: None,
            service_sid: None,
            sid: None,
            url: None,
        }
    }
}

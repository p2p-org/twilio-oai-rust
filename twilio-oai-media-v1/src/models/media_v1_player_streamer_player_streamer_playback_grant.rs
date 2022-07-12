/*
 * Twilio - Media
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MediaV1PlayerStreamerPlayerStreamerPlaybackGrant {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The grant that authorizes the player sdk to connect to the livestream
    #[serde(rename = "grant", skip_serializing_if = "Option::is_none")]
    pub grant: Option<serde_json::Value>,
    /// The unique string that identifies the PlayerStreamer associated with this PlaybackGrant.
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The absolute URL of the resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl MediaV1PlayerStreamerPlayerStreamerPlaybackGrant {
    pub fn new() -> MediaV1PlayerStreamerPlayerStreamerPlaybackGrant {
        MediaV1PlayerStreamerPlayerStreamerPlaybackGrant {
            account_sid: None,
            date_created: None,
            grant: None,
            sid: None,
            url: None,
        }
    }
}

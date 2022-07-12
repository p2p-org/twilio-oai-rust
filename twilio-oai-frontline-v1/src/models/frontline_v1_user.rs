/*
 * Twilio - Frontline
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FrontlineV1User {
    /// The avatar URL which will be shown in Frontline application
    #[serde(rename = "avatar", skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// The string that you assigned to describe the User
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// The string that identifies the resource's User
    #[serde(rename = "identity", skip_serializing_if = "Option::is_none")]
    pub identity: Option<String>,
    /// Whether the User is available for new conversations
    #[serde(rename = "is_available", skip_serializing_if = "Option::is_none")]
    pub is_available: Option<bool>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// Current state of this user
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<State>,
    /// An absolute URL for this user.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl FrontlineV1User {
    pub fn new() -> FrontlineV1User {
        FrontlineV1User {
            avatar: None,
            friendly_name: None,
            identity: None,
            is_available: None,
            sid: None,
            state: None,
            url: None,
        }
    }
}

/// Current state of this user
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum State {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "deactivated")]
    Deactivated,
}

impl Default for State {
    fn default() -> State {
        Self::Active
    }
}

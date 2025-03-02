/*
 * Twilio - Flex
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FlexV1InteractionInteractionChannel {
    /// The unique string that identifies the resource
    #[serde(rename = "interaction_sid", skip_serializing_if = "Option::is_none")]
    pub interaction_sid: Option<String>,
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The Interaction Channel's type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl FlexV1InteractionInteractionChannel {
    pub fn new() -> FlexV1InteractionInteractionChannel {
        FlexV1InteractionInteractionChannel {
            interaction_sid: None,
            links: None,
            sid: None,
            _type: None,
            url: None,
        }
    }
}

/// The Interaction Channel's type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "voice")]
    Voice,
    #[serde(rename = "sms")]
    Sms,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "web")]
    Web,
    #[serde(rename = "whatsapp")]
    Whatsapp,
    #[serde(rename = "chat")]
    Chat,
}

impl Default for Type {
    fn default() -> Type {
        Self::Voice
    }
}

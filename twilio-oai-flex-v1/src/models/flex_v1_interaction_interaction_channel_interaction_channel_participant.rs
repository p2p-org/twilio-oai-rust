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
pub struct FlexV1InteractionInteractionChannelInteractionChannelParticipant {
    /// The Channel Sid for this Participant.
    #[serde(rename = "channel_sid", skip_serializing_if = "Option::is_none")]
    pub channel_sid: Option<String>,
    /// The Interaction Sid for this channel.
    #[serde(rename = "interaction_sid", skip_serializing_if = "Option::is_none")]
    pub interaction_sid: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// Participant type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl FlexV1InteractionInteractionChannelInteractionChannelParticipant {
    pub fn new() -> FlexV1InteractionInteractionChannelInteractionChannelParticipant {
        FlexV1InteractionInteractionChannelInteractionChannelParticipant {
            channel_sid: None,
            interaction_sid: None,
            sid: None,
            _type: None,
            url: None,
        }
    }
}

/// Participant type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "supervisor")]
    Supervisor,
    #[serde(rename = "customer")]
    Customer,
    #[serde(rename = "external")]
    External,
    #[serde(rename = "agent")]
    Agent,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for Type {
    fn default() -> Type {
        Self::Supervisor
    }
}

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
pub struct ListInteractionChannelParticipantResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListChannelResponseMeta>>,
    #[serde(rename = "participants", skip_serializing_if = "Option::is_none")]
    pub participants: Option<
        Vec<crate::models::FlexV1InteractionInteractionChannelInteractionChannelParticipant>,
    >,
}

impl ListInteractionChannelParticipantResponse {
    pub fn new() -> ListInteractionChannelParticipantResponse {
        ListInteractionChannelParticipantResponse {
            meta: None,
            participants: None,
        }
    }
}

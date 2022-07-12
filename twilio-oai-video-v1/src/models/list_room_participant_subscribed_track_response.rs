/*
 * Twilio - Video
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListRoomParticipantSubscribedTrackResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListCompositionHookResponseMeta>>,
    #[serde(rename = "subscribed_tracks", skip_serializing_if = "Option::is_none")]
    pub subscribed_tracks:
        Option<Vec<crate::models::VideoV1RoomRoomParticipantRoomParticipantSubscribedTrack>>,
}

impl ListRoomParticipantSubscribedTrackResponse {
    pub fn new() -> ListRoomParticipantSubscribedTrackResponse {
        ListRoomParticipantSubscribedTrackResponse {
            meta: None,
            subscribed_tracks: None,
        }
    }
}

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
pub struct ListMediaRecordingResponse {
    #[serde(rename = "media_recordings", skip_serializing_if = "Option::is_none")]
    pub media_recordings: Option<Vec<crate::models::MediaV1MediaRecording>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListMediaProcessorResponseMeta>>,
}

impl ListMediaRecordingResponse {
    pub fn new() -> ListMediaRecordingResponse {
        ListMediaRecordingResponse {
            media_recordings: None,
            meta: None,
        }
    }
}

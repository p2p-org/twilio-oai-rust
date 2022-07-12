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
pub struct VideoV1RoomRoomRecording {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The codec used for the recording
    #[serde(rename = "codec", skip_serializing_if = "Option::is_none")]
    pub codec: Option<Codec>,
    /// The file format for the recording
    #[serde(rename = "container_format", skip_serializing_if = "Option::is_none")]
    pub container_format: Option<ContainerFormat>,
    /// The ISO 8601 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The duration of the recording in seconds
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    /// A list of SIDs related to the Recording
    #[serde(rename = "grouping_sids", skip_serializing_if = "Option::is_none")]
    pub grouping_sids: Option<serde_json::Value>,
    /// The URLs of related resources
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// The URL of the media file associated with the recording when stored externally
    #[serde(
        rename = "media_external_location",
        skip_serializing_if = "Option::is_none"
    )]
    pub media_external_location: Option<String>,
    /// The number of milliseconds between a point in time that is common to all rooms in a group and when the source room of the recording started
    #[serde(rename = "offset", skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// The SID of the Room resource the recording is associated with
    #[serde(rename = "room_sid", skip_serializing_if = "Option::is_none")]
    pub room_sid: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The size of the recorded track in bytes
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// The SID of the recording source
    #[serde(rename = "source_sid", skip_serializing_if = "Option::is_none")]
    pub source_sid: Option<String>,
    /// The status of the recording
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The name that was given to the source track of the recording
    #[serde(rename = "track_name", skip_serializing_if = "Option::is_none")]
    pub track_name: Option<String>,
    /// The recording's media type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// The absolute URL of the resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl VideoV1RoomRoomRecording {
    pub fn new() -> VideoV1RoomRoomRecording {
        VideoV1RoomRoomRecording {
            account_sid: None,
            codec: None,
            container_format: None,
            date_created: None,
            duration: None,
            grouping_sids: None,
            links: None,
            media_external_location: None,
            offset: None,
            room_sid: None,
            sid: None,
            size: None,
            source_sid: None,
            status: None,
            track_name: None,
            _type: None,
            url: None,
        }
    }
}

/// The codec used for the recording
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Codec {
    #[serde(rename = "VP8")]
    VP8,
    #[serde(rename = "H264")]
    H264,
    #[serde(rename = "OPUS")]
    OPUS,
    #[serde(rename = "PCMU")]
    PCMU,
}

impl Default for Codec {
    fn default() -> Codec {
        Self::VP8
    }
}
/// The file format for the recording
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContainerFormat {
    #[serde(rename = "mka")]
    Mka,
    #[serde(rename = "mkv")]
    Mkv,
}

impl Default for ContainerFormat {
    fn default() -> ContainerFormat {
        Self::Mka
    }
}
/// The status of the recording
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "processing")]
    Processing,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "failed")]
    Failed,
}

impl Default for Status {
    fn default() -> Status {
        Self::Processing
    }
}
/// The recording's media type
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "audio")]
    Audio,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "data")]
    Data,
}

impl Default for Type {
    fn default() -> Type {
        Self::Audio
    }
}

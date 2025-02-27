/*
 * Twilio - Insights
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct InsightsV1VideoRoomSummary {
    /// Account SID associated with this room.
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// Codecs used by participants in the room.
    #[serde(rename = "codecs", skip_serializing_if = "Option::is_none")]
    pub codecs: Option<Vec<Codecs>>,
    /// Actual number of concurrent participants.
    #[serde(
        rename = "concurrent_participants",
        skip_serializing_if = "Option::is_none"
    )]
    pub concurrent_participants: Option<i32>,
    /// Creation time of the room.
    #[serde(rename = "create_time", skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// How the room was created.
    #[serde(rename = "created_method", skip_serializing_if = "Option::is_none")]
    pub created_method: Option<CreatedMethod>,
    /// Total room duration from create time to end time.
    #[serde(rename = "duration_sec", skip_serializing_if = "Option::is_none")]
    pub duration_sec: Option<i32>,
    /// Edge location of Twilio media servers for the room.
    #[serde(rename = "edge_location", skip_serializing_if = "Option::is_none")]
    pub edge_location: Option<EdgeLocation>,
    /// Reason the room ended.
    #[serde(rename = "end_reason", skip_serializing_if = "Option::is_none")]
    pub end_reason: Option<EndReason>,
    /// End time for the room.
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// Room subresources.
    #[serde(rename = "links", skip_serializing_if = "Option::is_none")]
    pub links: Option<serde_json::Value>,
    /// Maximum number of participants allowed in the room at the same time allowed by the application settings.
    #[serde(
        rename = "max_concurrent_participants",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_concurrent_participants: Option<i32>,
    /// Max number of total participants allowed by the application settings.
    #[serde(rename = "max_participants", skip_serializing_if = "Option::is_none")]
    pub max_participants: Option<i32>,
    /// Region of Twilio media servers for the room.
    #[serde(rename = "media_region", skip_serializing_if = "Option::is_none")]
    pub media_region: Option<MediaRegion>,
    /// Video Log Analyzer resource state. Will be either `in-progress` or `complete`.
    #[serde(rename = "processing_state", skip_serializing_if = "Option::is_none")]
    pub processing_state: Option<ProcessingState>,
    /// Boolean indicating if recording is enabled for the room.
    #[serde(rename = "recording_enabled", skip_serializing_if = "Option::is_none")]
    pub recording_enabled: Option<bool>,
    /// room friendly name.
    #[serde(rename = "room_name", skip_serializing_if = "Option::is_none")]
    pub room_name: Option<String>,
    /// Unique identifier for the room.
    #[serde(rename = "room_sid", skip_serializing_if = "Option::is_none")]
    pub room_sid: Option<String>,
    /// Status of the room.
    #[serde(rename = "room_status", skip_serializing_if = "Option::is_none")]
    pub room_status: Option<RoomStatus>,
    /// Type of room.
    #[serde(rename = "room_type", skip_serializing_if = "Option::is_none")]
    pub room_type: Option<RoomType>,
    /// Webhook provided for status callbacks.
    #[serde(rename = "status_callback", skip_serializing_if = "Option::is_none")]
    pub status_callback: Option<String>,
    /// HTTP method provided for status callback URL.
    #[serde(
        rename = "status_callback_method",
        skip_serializing_if = "Option::is_none"
    )]
    pub status_callback_method: Option<StatusCallbackMethod>,
    /// Combined amount of participant time in the room.
    #[serde(
        rename = "total_participant_duration_sec",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_participant_duration_sec: Option<i32>,
    /// Combined amount of recorded seconds for participants in the room.
    #[serde(
        rename = "total_recording_duration_sec",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_recording_duration_sec: Option<i32>,
    /// Unique number of participant identities.
    #[serde(
        rename = "unique_participant_identities",
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_participant_identities: Option<i32>,
    /// Number of participants. May include duplicate identities for participants who left and rejoined.
    #[serde(
        rename = "unique_participants",
        skip_serializing_if = "Option::is_none"
    )]
    pub unique_participants: Option<i32>,
    /// URL for the room resource.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl InsightsV1VideoRoomSummary {
    pub fn new() -> InsightsV1VideoRoomSummary {
        InsightsV1VideoRoomSummary {
            account_sid: None,
            codecs: None,
            concurrent_participants: None,
            create_time: None,
            created_method: None,
            duration_sec: None,
            edge_location: None,
            end_reason: None,
            end_time: None,
            links: None,
            max_concurrent_participants: None,
            max_participants: None,
            media_region: None,
            processing_state: None,
            recording_enabled: None,
            room_name: None,
            room_sid: None,
            room_status: None,
            room_type: None,
            status_callback: None,
            status_callback_method: None,
            total_participant_duration_sec: None,
            total_recording_duration_sec: None,
            unique_participant_identities: None,
            unique_participants: None,
            url: None,
        }
    }
}

/// Codecs used by participants in the room.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Codecs {
    #[serde(rename = "VP8")]
    VP8,
    #[serde(rename = "H264")]
    H264,
    #[serde(rename = "VP9")]
    VP9,
}

impl Default for Codecs {
    fn default() -> Codecs {
        Self::VP8
    }
}
/// How the room was created.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CreatedMethod {
    #[serde(rename = "sdk")]
    Sdk,
    #[serde(rename = "ad_hoc")]
    AdHoc,
    #[serde(rename = "api")]
    Api,
}

impl Default for CreatedMethod {
    fn default() -> CreatedMethod {
        Self::Sdk
    }
}
/// Edge location of Twilio media servers for the room.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EdgeLocation {
    #[serde(rename = "ashburn")]
    Ashburn,
    #[serde(rename = "dublin")]
    Dublin,
    #[serde(rename = "frankfurt")]
    Frankfurt,
    #[serde(rename = "singapore")]
    Singapore,
    #[serde(rename = "sydney")]
    Sydney,
    #[serde(rename = "sao_paulo")]
    SaoPaulo,
    #[serde(rename = "roaming")]
    Roaming,
    #[serde(rename = "umatilla")]
    Umatilla,
    #[serde(rename = "tokyo")]
    Tokyo,
}

impl Default for EdgeLocation {
    fn default() -> EdgeLocation {
        Self::Ashburn
    }
}
/// Reason the room ended.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EndReason {
    #[serde(rename = "room_ended_via_api")]
    RoomEndedViaApi,
    #[serde(rename = "timeout")]
    Timeout,
}

impl Default for EndReason {
    fn default() -> EndReason {
        Self::RoomEndedViaApi
    }
}
/// Region of Twilio media servers for the room.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MediaRegion {
    #[serde(rename = "us1")]
    Us1,
    #[serde(rename = "us2")]
    Us2,
    #[serde(rename = "au1")]
    Au1,
    #[serde(rename = "br1")]
    Br1,
    #[serde(rename = "ie1")]
    Ie1,
    #[serde(rename = "jp1")]
    Jp1,
    #[serde(rename = "sg1")]
    Sg1,
    #[serde(rename = "in1")]
    In1,
    #[serde(rename = "de1")]
    De1,
    #[serde(rename = "gll")]
    Gll,
}

impl Default for MediaRegion {
    fn default() -> MediaRegion {
        Self::Us1
    }
}
/// Video Log Analyzer resource state. Will be either `in-progress` or `complete`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProcessingState {
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "in_progress")]
    InProgress,
}

impl Default for ProcessingState {
    fn default() -> ProcessingState {
        Self::Complete
    }
}
/// Status of the room.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RoomStatus {
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
}

impl Default for RoomStatus {
    fn default() -> RoomStatus {
        Self::InProgress
    }
}
/// Type of room.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RoomType {
    #[serde(rename = "go")]
    Go,
    #[serde(rename = "peer_to_peer")]
    PeerToPeer,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "group_small")]
    GroupSmall,
}

impl Default for RoomType {
    fn default() -> RoomType {
        Self::Go
    }
}
/// HTTP method provided for status callback URL.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StatusCallbackMethod {
    #[serde(rename = "HEAD")]
    HEAD,
    #[serde(rename = "GET")]
    GET,
    #[serde(rename = "POST")]
    POST,
    #[serde(rename = "PATCH")]
    PATCH,
    #[serde(rename = "PUT")]
    PUT,
    #[serde(rename = "DELETE")]
    DELETE,
}

impl Default for StatusCallbackMethod {
    fn default() -> StatusCallbackMethod {
        Self::HEAD
    }
}

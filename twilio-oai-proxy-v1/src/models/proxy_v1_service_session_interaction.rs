/*
 * Twilio - Proxy
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProxyV1ServiceSessionInteraction {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// A JSON string that includes the message body of message interactions
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// The ISO 8601 date and time in GMT when the Interaction was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// The SID of the inbound Participant resource
    #[serde(
        rename = "inbound_participant_sid",
        skip_serializing_if = "Option::is_none"
    )]
    pub inbound_participant_sid: Option<String>,
    /// The SID of the inbound resource
    #[serde(
        rename = "inbound_resource_sid",
        skip_serializing_if = "Option::is_none"
    )]
    pub inbound_resource_sid: Option<String>,
    /// The inbound resource status of the Interaction
    #[serde(
        rename = "inbound_resource_status",
        skip_serializing_if = "Option::is_none"
    )]
    pub inbound_resource_status: Option<InboundResourceStatus>,
    /// The inbound resource type
    #[serde(
        rename = "inbound_resource_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub inbound_resource_type: Option<String>,
    /// The URL of the Twilio inbound resource
    #[serde(
        rename = "inbound_resource_url",
        skip_serializing_if = "Option::is_none"
    )]
    pub inbound_resource_url: Option<String>,
    /// The SID of the outbound Participant
    #[serde(
        rename = "outbound_participant_sid",
        skip_serializing_if = "Option::is_none"
    )]
    pub outbound_participant_sid: Option<String>,
    /// The SID of the outbound resource
    #[serde(
        rename = "outbound_resource_sid",
        skip_serializing_if = "Option::is_none"
    )]
    pub outbound_resource_sid: Option<String>,
    /// The outbound resource status of the Interaction
    #[serde(
        rename = "outbound_resource_status",
        skip_serializing_if = "Option::is_none"
    )]
    pub outbound_resource_status: Option<OutboundResourceStatus>,
    /// The outbound resource type
    #[serde(
        rename = "outbound_resource_type",
        skip_serializing_if = "Option::is_none"
    )]
    pub outbound_resource_type: Option<String>,
    /// The URL of the Twilio outbound resource
    #[serde(
        rename = "outbound_resource_url",
        skip_serializing_if = "Option::is_none"
    )]
    pub outbound_resource_url: Option<String>,
    /// The SID of the resource's parent Service
    #[serde(rename = "service_sid", skip_serializing_if = "Option::is_none")]
    pub service_sid: Option<String>,
    /// The SID of the resource's parent Session
    #[serde(rename = "session_sid", skip_serializing_if = "Option::is_none")]
    pub session_sid: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The Type of the Interaction
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<Type>,
    /// The absolute URL of the Interaction resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ProxyV1ServiceSessionInteraction {
    pub fn new() -> ProxyV1ServiceSessionInteraction {
        ProxyV1ServiceSessionInteraction {
            account_sid: None,
            data: None,
            date_created: None,
            date_updated: None,
            inbound_participant_sid: None,
            inbound_resource_sid: None,
            inbound_resource_status: None,
            inbound_resource_type: None,
            inbound_resource_url: None,
            outbound_participant_sid: None,
            outbound_resource_sid: None,
            outbound_resource_status: None,
            outbound_resource_type: None,
            outbound_resource_url: None,
            service_sid: None,
            session_sid: None,
            sid: None,
            _type: None,
            url: None,
        }
    }
}

/// The inbound resource status of the Interaction
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InboundResourceStatus {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "answered")]
    Answered,
    #[serde(rename = "busy")]
    Busy,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "delivery-unknown")]
    DeliveryUnknown,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "in-progress")]
    InProgress,
    #[serde(rename = "initiated")]
    Initiated,
    #[serde(rename = "no-answer")]
    NoAnswer,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "received")]
    Received,
    #[serde(rename = "receiving")]
    Receiving,
    #[serde(rename = "ringing")]
    Ringing,
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "sending")]
    Sending,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "undelivered")]
    Undelivered,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for InboundResourceStatus {
    fn default() -> InboundResourceStatus {
        Self::Accepted
    }
}
/// The outbound resource status of the Interaction
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OutboundResourceStatus {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "answered")]
    Answered,
    #[serde(rename = "busy")]
    Busy,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "delivery-unknown")]
    DeliveryUnknown,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "in-progress")]
    InProgress,
    #[serde(rename = "initiated")]
    Initiated,
    #[serde(rename = "no-answer")]
    NoAnswer,
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "received")]
    Received,
    #[serde(rename = "receiving")]
    Receiving,
    #[serde(rename = "ringing")]
    Ringing,
    #[serde(rename = "scheduled")]
    Scheduled,
    #[serde(rename = "sending")]
    Sending,
    #[serde(rename = "sent")]
    Sent,
    #[serde(rename = "undelivered")]
    Undelivered,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for OutboundResourceStatus {
    fn default() -> OutboundResourceStatus {
        Self::Accepted
    }
}
/// The Type of the Interaction
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "message")]
    Message,
    #[serde(rename = "voice")]
    Voice,
    #[serde(rename = "unknown")]
    Unknown,
}

impl Default for Type {
    fn default() -> Type {
        Self::Message
    }
}

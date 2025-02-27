/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiV2010AccountCall {
    /// The SID of the Account that created this resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// Either `human` or `machine` if this call was initiated with answering machine detection. Empty otherwise.
    #[serde(rename = "answered_by", skip_serializing_if = "Option::is_none")]
    pub answered_by: Option<String>,
    /// The API Version used to create the call
    #[serde(rename = "api_version", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    /// The caller's name if this call was an incoming call to a phone number with caller ID Lookup enabled. Otherwise, empty.
    #[serde(rename = "caller_name", skip_serializing_if = "Option::is_none")]
    pub caller_name: Option<String>,
    /// The RFC 2822 date and time in GMT that this resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The RFC 2822 date and time in GMT that this resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// A string describing the direction of the call. `inbound` for inbound calls, `outbound-api` for calls initiated via the REST API or `outbound-dial` for calls initiated by a `Dial` verb.
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
    /// The length of the call in seconds.
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// The end time of the call. Null if the call did not complete successfully.
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// The forwarding phone number if this call was an incoming call forwarded from another number (depends on carrier supporting forwarding). Otherwise, empty.
    #[serde(rename = "forwarded_from", skip_serializing_if = "Option::is_none")]
    pub forwarded_from: Option<String>,
    /// The phone number, SIP address or Client identifier that made this call. Phone numbers are in E.164 format (e.g., +16175551212). SIP addresses are formatted as `name@company.com`. Client identifiers are formatted `client:name`.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// The calling phone number, SIP address, or Client identifier formatted for display.
    #[serde(rename = "from_formatted", skip_serializing_if = "Option::is_none")]
    pub from_formatted: Option<String>,
    /// The Group SID associated with this call. If no Group is associated with the call, the field is empty.
    #[serde(rename = "group_sid", skip_serializing_if = "Option::is_none")]
    pub group_sid: Option<String>,
    /// The SID that identifies the call that created this leg.
    #[serde(rename = "parent_call_sid", skip_serializing_if = "Option::is_none")]
    pub parent_call_sid: Option<String>,
    /// If the call was inbound, this is the SID of the IncomingPhoneNumber resource that received the call. If the call was outbound, it is the SID of the OutgoingCallerId resource from which the call was placed.
    #[serde(rename = "phone_number_sid", skip_serializing_if = "Option::is_none")]
    pub phone_number_sid: Option<String>,
    /// The charge for this call, in the currency associated with the account. Populated after the call is completed. May not be immediately available.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// The currency in which `Price` is measured.
    #[serde(rename = "price_unit", skip_serializing_if = "Option::is_none")]
    pub price_unit: Option<String>,
    /// The wait time in milliseconds before the call is placed.
    #[serde(rename = "queue_time", skip_serializing_if = "Option::is_none")]
    pub queue_time: Option<String>,
    /// The unique string that identifies this resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The start time of the call. Null if the call has not yet been dialed.
    #[serde(rename = "start_time", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// The status of this call.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// A list of related subresources identified by their relative URIs
    #[serde(rename = "subresource_uris", skip_serializing_if = "Option::is_none")]
    pub subresource_uris: Option<serde_json::Value>,
    /// The phone number, SIP address or Client identifier that received this call. Phone numbers are in E.164 format (e.g., +16175551212). SIP addresses are formatted as `name@company.com`. Client identifiers are formatted `client:name`.
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    /// The phone number, SIP address or Client identifier that received this call. Formatted for display.
    #[serde(rename = "to_formatted", skip_serializing_if = "Option::is_none")]
    pub to_formatted: Option<String>,
    /// The (optional) unique identifier of the trunk resource that was used for this call.
    #[serde(rename = "trunk_sid", skip_serializing_if = "Option::is_none")]
    pub trunk_sid: Option<String>,
    /// The URI of this resource, relative to `https://api.twilio.com`
    #[serde(rename = "uri", skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

impl ApiV2010AccountCall {
    pub fn new() -> ApiV2010AccountCall {
        ApiV2010AccountCall {
            account_sid: None,
            answered_by: None,
            api_version: None,
            caller_name: None,
            date_created: None,
            date_updated: None,
            direction: None,
            duration: None,
            end_time: None,
            forwarded_from: None,
            from: None,
            from_formatted: None,
            group_sid: None,
            parent_call_sid: None,
            phone_number_sid: None,
            price: None,
            price_unit: None,
            queue_time: None,
            sid: None,
            start_time: None,
            status: None,
            subresource_uris: None,
            to: None,
            to_formatted: None,
            trunk_sid: None,
            uri: None,
        }
    }
}

/// The status of this call.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "queued")]
    Queued,
    #[serde(rename = "ringing")]
    Ringing,
    #[serde(rename = "in-progress")]
    InProgress,
    #[serde(rename = "completed")]
    Completed,
    #[serde(rename = "busy")]
    Busy,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "no-answer")]
    NoAnswer,
    #[serde(rename = "canceled")]
    Canceled,
}

impl Default for Status {
    fn default() -> Status {
        Self::Queued
    }
}

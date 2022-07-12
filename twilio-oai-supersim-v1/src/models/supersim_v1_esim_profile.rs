/*
 * Twilio - Supersim
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SupersimV1EsimProfile {
    /// The SID of the Account to which the eSIM Profile resource belongs
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// Identifier of the eUICC that can claim the eSIM Profile
    #[serde(rename = "eid", skip_serializing_if = "Option::is_none")]
    pub eid: Option<String>,
    /// Code indicating the failure if the download of the SIM Profile failed and the eSIM Profile is in `failed` state
    #[serde(rename = "error_code", skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// Error message describing the failure if the download of the SIM Profile failed and the eSIM Profile is in `failed` state
    #[serde(rename = "error_message", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// The ICCID associated with the Sim resource
    #[serde(rename = "iccid", skip_serializing_if = "Option::is_none")]
    pub iccid: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The SID of the Sim resource that this eSIM Profile controls
    #[serde(rename = "sim_sid", skip_serializing_if = "Option::is_none")]
    pub sim_sid: Option<String>,
    /// Address of the SM-DP+ server from which the Profile will be downloaded
    #[serde(rename = "smdp_plus_address", skip_serializing_if = "Option::is_none")]
    pub smdp_plus_address: Option<String>,
    /// The status of the eSIM Profile
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// The absolute URL of the eSIM Profile resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl SupersimV1EsimProfile {
    pub fn new() -> SupersimV1EsimProfile {
        SupersimV1EsimProfile {
            account_sid: None,
            date_created: None,
            date_updated: None,
            eid: None,
            error_code: None,
            error_message: None,
            iccid: None,
            sid: None,
            sim_sid: None,
            smdp_plus_address: None,
            status: None,
            url: None,
        }
    }
}

/// The status of the eSIM Profile
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "reserving")]
    Reserving,
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "downloaded")]
    Downloaded,
    #[serde(rename = "installed")]
    Installed,
    #[serde(rename = "failed")]
    Failed,
}

impl Default for Status {
    fn default() -> Status {
        Self::New
    }
}

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
pub struct SupersimV1Fleet {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// Defines whether SIMs in the Fleet are capable of using data connectivity
    #[serde(rename = "data_enabled", skip_serializing_if = "Option::is_none")]
    pub data_enabled: Option<bool>,
    /// The total data usage (download and upload combined) in Megabytes that each Sim resource assigned to the Fleet resource can consume
    #[serde(rename = "data_limit", skip_serializing_if = "Option::is_none")]
    pub data_limit: Option<i32>,
    /// The model by which a SIM is metered and billed
    #[serde(rename = "data_metering", skip_serializing_if = "Option::is_none")]
    pub data_metering: Option<DataMetering>,
    /// The ISO 8601 date and time in GMT when the resource was created
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The ISO 8601 date and time in GMT when the resource was last updated
    #[serde(rename = "date_updated", skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    /// A string representing the HTTP method to use when making a request to `ip_commands_url`
    #[serde(rename = "ip_commands_method", skip_serializing_if = "Option::is_none")]
    pub ip_commands_method: Option<IpCommandsMethod>,
    /// The URL that will receive a webhook when a Super SIM in the Fleet is used to send an IP Command from your device
    #[serde(rename = "ip_commands_url", skip_serializing_if = "Option::is_none")]
    pub ip_commands_url: Option<String>,
    /// The SID of the Network Access Profile of the Fleet
    #[serde(
        rename = "network_access_profile_sid",
        skip_serializing_if = "Option::is_none"
    )]
    pub network_access_profile_sid: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// Defines whether SIMs in the Fleet are capable of sending and receiving machine-to-machine SMS via Commands
    #[serde(
        rename = "sms_commands_enabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub sms_commands_enabled: Option<bool>,
    /// A string representing the HTTP method to use when making a request to `sms_commands_url`
    #[serde(
        rename = "sms_commands_method",
        skip_serializing_if = "Option::is_none"
    )]
    pub sms_commands_method: Option<SmsCommandsMethod>,
    /// The URL that will receive a webhook when a Super SIM in the Fleet is used to send an SMS from your device to the SMS Commands number
    #[serde(rename = "sms_commands_url", skip_serializing_if = "Option::is_none")]
    pub sms_commands_url: Option<String>,
    /// An application-defined string that uniquely identifies the resource
    #[serde(rename = "unique_name", skip_serializing_if = "Option::is_none")]
    pub unique_name: Option<String>,
    /// The absolute URL of the Fleet resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl SupersimV1Fleet {
    pub fn new() -> SupersimV1Fleet {
        SupersimV1Fleet {
            account_sid: None,
            data_enabled: None,
            data_limit: None,
            data_metering: None,
            date_created: None,
            date_updated: None,
            ip_commands_method: None,
            ip_commands_url: None,
            network_access_profile_sid: None,
            sid: None,
            sms_commands_enabled: None,
            sms_commands_method: None,
            sms_commands_url: None,
            unique_name: None,
            url: None,
        }
    }
}

/// The model by which a SIM is metered and billed
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DataMetering {
    #[serde(rename = "payg")]
    Payg,
}

impl Default for DataMetering {
    fn default() -> DataMetering {
        Self::Payg
    }
}
/// A string representing the HTTP method to use when making a request to `ip_commands_url`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IpCommandsMethod {
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

impl Default for IpCommandsMethod {
    fn default() -> IpCommandsMethod {
        Self::HEAD
    }
}
/// A string representing the HTTP method to use when making a request to `sms_commands_url`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SmsCommandsMethod {
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

impl Default for SmsCommandsMethod {
    fn default() -> SmsCommandsMethod {
        Self::HEAD
    }
}

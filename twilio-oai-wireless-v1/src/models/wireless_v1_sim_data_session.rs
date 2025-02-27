/*
 * Twilio - Wireless
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct WirelessV1SimDataSession {
    /// The SID of the Account that created the resource
    #[serde(rename = "account_sid", skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    /// The unique ID of the cellular tower that the device was attached to at the moment when the Data Session was last updated
    #[serde(rename = "cell_id", skip_serializing_if = "Option::is_none")]
    pub cell_id: Option<String>,
    /// An object with the estimated location where the device's Data Session took place
    #[serde(
        rename = "cell_location_estimate",
        skip_serializing_if = "Option::is_none"
    )]
    pub cell_location_estimate: Option<serde_json::Value>,
    /// The date that the record ended, given as GMT in ISO 8601 format
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// The unique ID of the device using the SIM to connect
    #[serde(rename = "imei", skip_serializing_if = "Option::is_none")]
    pub imei: Option<String>,
    /// The date that the resource was last updated, given as GMT in ISO 8601 format
    #[serde(rename = "last_updated", skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    /// The three letter country code representing where the device's Data Session took place
    #[serde(rename = "operator_country", skip_serializing_if = "Option::is_none")]
    pub operator_country: Option<String>,
    /// The 'mobile country code' is the unique ID of the home country where the Data Session took place
    #[serde(rename = "operator_mcc", skip_serializing_if = "Option::is_none")]
    pub operator_mcc: Option<String>,
    /// The 'mobile network code' is the unique ID specific to the mobile operator network where the Data Session took place
    #[serde(rename = "operator_mnc", skip_serializing_if = "Option::is_none")]
    pub operator_mnc: Option<String>,
    /// The friendly name of the mobile operator network that the SIM-connected device is attached to
    #[serde(rename = "operator_name", skip_serializing_if = "Option::is_none")]
    pub operator_name: Option<String>,
    /// The number of packets downloaded by the device between the start time and when the Data Session was last updated
    #[serde(rename = "packets_downloaded", skip_serializing_if = "Option::is_none")]
    pub packets_downloaded: Option<i32>,
    /// The number of packets uploaded by the device between the start time and when the Data Session was last updated
    #[serde(rename = "packets_uploaded", skip_serializing_if = "Option::is_none")]
    pub packets_uploaded: Option<i32>,
    /// The generation of wireless technology that the device was using
    #[serde(rename = "radio_link", skip_serializing_if = "Option::is_none")]
    pub radio_link: Option<String>,
    /// The unique string that identifies the resource
    #[serde(rename = "sid", skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    /// The SID of the Sim resource that the Data Session is for
    #[serde(rename = "sim_sid", skip_serializing_if = "Option::is_none")]
    pub sim_sid: Option<String>,
    /// The date that the Data Session started, given as GMT in ISO 8601 format
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
}

impl WirelessV1SimDataSession {
    pub fn new() -> WirelessV1SimDataSession {
        WirelessV1SimDataSession {
            account_sid: None,
            cell_id: None,
            cell_location_estimate: None,
            end: None,
            imei: None,
            last_updated: None,
            operator_country: None,
            operator_mcc: None,
            operator_mnc: None,
            operator_name: None,
            packets_downloaded: None,
            packets_uploaded: None,
            radio_link: None,
            sid: None,
            sim_sid: None,
            start: None,
        }
    }
}

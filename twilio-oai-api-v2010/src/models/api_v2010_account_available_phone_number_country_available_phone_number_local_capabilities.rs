/*
 * Twilio - Api
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

/// ApiV2010AccountAvailablePhoneNumberCountryAvailablePhoneNumberLocalCapabilities : Whether a phone number can receive calls or messages

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ApiV2010AccountAvailablePhoneNumberCountryAvailablePhoneNumberLocalCapabilities {
    #[serde(rename = "fax", skip_serializing_if = "Option::is_none")]
    pub fax: Option<bool>,
    #[serde(rename = "mms", skip_serializing_if = "Option::is_none")]
    pub mms: Option<bool>,
    #[serde(rename = "sms", skip_serializing_if = "Option::is_none")]
    pub sms: Option<bool>,
    #[serde(rename = "voice", skip_serializing_if = "Option::is_none")]
    pub voice: Option<bool>,
}

impl ApiV2010AccountAvailablePhoneNumberCountryAvailablePhoneNumberLocalCapabilities {
    /// Whether a phone number can receive calls or messages
    pub fn new() -> ApiV2010AccountAvailablePhoneNumberCountryAvailablePhoneNumberLocalCapabilities
    {
        ApiV2010AccountAvailablePhoneNumberCountryAvailablePhoneNumberLocalCapabilities {
            fax: None,
            mms: None,
            sms: None,
            voice: None,
        }
    }
}

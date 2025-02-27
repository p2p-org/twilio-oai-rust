/*
 * Twilio - Trunking
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListPhoneNumberResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListTrunkResponseMeta>>,
    #[serde(rename = "phone_numbers", skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<crate::models::TrunkingV1TrunkPhoneNumber>>,
}

impl ListPhoneNumberResponse {
    pub fn new() -> ListPhoneNumberResponse {
        ListPhoneNumberResponse {
            meta: None,
            phone_numbers: None,
        }
    }
}

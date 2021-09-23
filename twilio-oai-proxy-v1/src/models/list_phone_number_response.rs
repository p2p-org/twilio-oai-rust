/*
 * Twilio - Proxy
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListPhoneNumberResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListServiceResponseMeta>>,
    #[serde(rename = "phone_numbers", skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<crate::models::ProxyV1ServicePhoneNumber>>,
}

impl ListPhoneNumberResponse {
    pub fn new() -> ListPhoneNumberResponse {
        ListPhoneNumberResponse {
            meta: None,
            phone_numbers: None,
        }
    }
}



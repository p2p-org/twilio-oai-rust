/*
 * Twilio - Messaging
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListBrandRegistrationsResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::models::MessagingV1BrandRegistrations>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListServiceResponseMeta>>,
}

impl ListBrandRegistrationsResponse {
    pub fn new() -> ListBrandRegistrationsResponse {
        ListBrandRegistrationsResponse {
            data: None,
            meta: None,
        }
    }
}

/*
 * Twilio - Conversations
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListConfigurationAddressResponse {
    #[serde(
        rename = "address_configurations",
        skip_serializing_if = "Option::is_none"
    )]
    pub address_configurations: Option<Vec<crate::models::ConversationsV1ConfigurationAddress>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListConfigurationAddressResponseMeta>>,
}

impl ListConfigurationAddressResponse {
    pub fn new() -> ListConfigurationAddressResponse {
        ListConfigurationAddressResponse {
            address_configurations: None,
            meta: None,
        }
    }
}

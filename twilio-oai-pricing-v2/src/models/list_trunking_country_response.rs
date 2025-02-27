/*
 * Twilio - Pricing
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListTrunkingCountryResponse {
    #[serde(rename = "countries", skip_serializing_if = "Option::is_none")]
    pub countries: Option<Vec<crate::models::PricingV2TrunkingCountry>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListTrunkingCountryResponseMeta>>,
}

impl ListTrunkingCountryResponse {
    pub fn new() -> ListTrunkingCountryResponse {
        ListTrunkingCountryResponse {
            countries: None,
            meta: None,
        }
    }
}

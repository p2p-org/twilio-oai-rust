/*
 * Twilio - Pricing
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListVoiceCountryResponse {
    #[serde(rename = "countries", skip_serializing_if = "Option::is_none")]
    pub countries: Option<Vec<crate::models::PricingV2VoiceVoiceCountry>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListTrunkingCountryResponseMeta>>,
}

impl ListVoiceCountryResponse {
    pub fn new() -> ListVoiceCountryResponse {
        ListVoiceCountryResponse {
            countries: None,
            meta: None,
        }
    }
}



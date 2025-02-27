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
pub struct PricingV1VoiceVoiceCountryInstanceOutboundPrefixPricesInner {
    #[serde(rename = "base_price", skip_serializing_if = "Option::is_none")]
    pub base_price: Option<f32>,
    #[serde(rename = "current_price", skip_serializing_if = "Option::is_none")]
    pub current_price: Option<f32>,
    #[serde(rename = "friendly_name", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "prefixes", skip_serializing_if = "Option::is_none")]
    pub prefixes: Option<Vec<String>>,
}

impl PricingV1VoiceVoiceCountryInstanceOutboundPrefixPricesInner {
    pub fn new() -> PricingV1VoiceVoiceCountryInstanceOutboundPrefixPricesInner {
        PricingV1VoiceVoiceCountryInstanceOutboundPrefixPricesInner {
            base_price: None,
            current_price: None,
            friendly_name: None,
            prefixes: None,
        }
    }
}

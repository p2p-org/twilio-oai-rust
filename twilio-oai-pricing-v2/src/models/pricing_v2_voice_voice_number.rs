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
pub struct PricingV2VoiceVoiceNumber {
    /// The name of the country
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The destination phone number, in E.164 format
    #[serde(rename = "destination_number", skip_serializing_if = "Option::is_none")]
    pub destination_number: Option<String>,
    #[serde(rename = "inbound_call_price", skip_serializing_if = "Option::is_none")]
    pub inbound_call_price: Option<Box<crate::models::PricingV2VoiceVoiceNumberInboundCallPrice>>,
    /// The ISO country code
    #[serde(rename = "iso_country", skip_serializing_if = "Option::is_none")]
    pub iso_country: Option<String>,
    /// The origination phone number, in E.164 format
    #[serde(rename = "origination_number", skip_serializing_if = "Option::is_none")]
    pub origination_number: Option<String>,
    /// The list of OutboundCallPriceWithOrigin records
    #[serde(rename = "outbound_call_prices", skip_serializing_if = "Option::is_none")]
    pub outbound_call_prices: Option<Vec<crate::models::PricingV2VoiceVoiceNumberOutboundCallPrices>>,
    /// The currency in which prices are measured, in ISO 4127 format (e.g. usd, eur, jpy)
    #[serde(rename = "price_unit", skip_serializing_if = "Option::is_none")]
    pub price_unit: Option<String>,
    /// The absolute URL of the resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl PricingV2VoiceVoiceNumber {
    pub fn new() -> PricingV2VoiceVoiceNumber {
        PricingV2VoiceVoiceNumber {
            country: None,
            destination_number: None,
            inbound_call_price: None,
            iso_country: None,
            origination_number: None,
            outbound_call_prices: None,
            price_unit: None,
            url: None,
        }
    }
}



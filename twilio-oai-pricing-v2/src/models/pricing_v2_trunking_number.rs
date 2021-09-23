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
pub struct PricingV2TrunkingNumber {
    /// The name of the country
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The destination phone number, in E.164 format
    #[serde(rename = "destination_number", skip_serializing_if = "Option::is_none")]
    pub destination_number: Option<String>,
    /// The ISO country code
    #[serde(rename = "iso_country", skip_serializing_if = "Option::is_none")]
    pub iso_country: Option<String>,
    #[serde(rename = "originating_call_price", skip_serializing_if = "Option::is_none")]
    pub originating_call_price: Option<Box<crate::models::PricingV2TrunkingNumberOriginatingCallPrice>>,
    /// The origination phone number, in E.164 format
    #[serde(rename = "origination_number", skip_serializing_if = "Option::is_none")]
    pub origination_number: Option<String>,
    /// The currency in which prices are measured, in ISO 4127 format (e.g. usd, eur, jpy)
    #[serde(rename = "price_unit", skip_serializing_if = "Option::is_none")]
    pub price_unit: Option<String>,
    #[serde(rename = "terminating_prefix_prices", skip_serializing_if = "Option::is_none")]
    pub terminating_prefix_prices: Option<Vec<crate::models::PricingV2TrunkingCountryInstanceTerminatingPrefixPrices>>,
    /// The absolute URL of the resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl PricingV2TrunkingNumber {
    pub fn new() -> PricingV2TrunkingNumber {
        PricingV2TrunkingNumber {
            country: None,
            destination_number: None,
            iso_country: None,
            originating_call_price: None,
            origination_number: None,
            price_unit: None,
            terminating_prefix_prices: None,
            url: None,
        }
    }
}



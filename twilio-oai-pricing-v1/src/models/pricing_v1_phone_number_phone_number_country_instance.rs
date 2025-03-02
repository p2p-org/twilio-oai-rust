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
pub struct PricingV1PhoneNumberPhoneNumberCountryInstance {
    /// The name of the country
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The ISO country code
    #[serde(rename = "iso_country", skip_serializing_if = "Option::is_none")]
    pub iso_country: Option<String>,
    /// The list of PhoneNumberPrices records
    #[serde(
        rename = "phone_number_prices",
        skip_serializing_if = "Option::is_none"
    )]
    pub phone_number_prices: Option<
        Vec<crate::models::PricingV1PhoneNumberPhoneNumberCountryInstancePhoneNumberPricesInner>,
    >,
    /// The currency in which prices are measured, in ISO 4127 format (e.g. usd, eur, jpy)
    #[serde(rename = "price_unit", skip_serializing_if = "Option::is_none")]
    pub price_unit: Option<String>,
    /// The absolute URL of the resource
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl PricingV1PhoneNumberPhoneNumberCountryInstance {
    pub fn new() -> PricingV1PhoneNumberPhoneNumberCountryInstance {
        PricingV1PhoneNumberPhoneNumberCountryInstance {
            country: None,
            iso_country: None,
            phone_number_prices: None,
            price_unit: None,
            url: None,
        }
    }
}

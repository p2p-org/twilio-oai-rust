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
pub struct PricingV1PhoneNumberPhoneNumberCountryInstancePhoneNumberPricesInner {
    #[serde(rename = "base_price", skip_serializing_if = "Option::is_none")]
    pub base_price: Option<f32>,
    #[serde(rename = "current_price", skip_serializing_if = "Option::is_none")]
    pub current_price: Option<f32>,
    #[serde(rename = "number_type", skip_serializing_if = "Option::is_none")]
    pub number_type: Option<String>,
}

impl PricingV1PhoneNumberPhoneNumberCountryInstancePhoneNumberPricesInner {
    pub fn new() -> PricingV1PhoneNumberPhoneNumberCountryInstancePhoneNumberPricesInner {
        PricingV1PhoneNumberPhoneNumberCountryInstancePhoneNumberPricesInner {
            base_price: None,
            current_price: None,
            number_type: None,
        }
    }
}

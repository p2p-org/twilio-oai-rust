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
pub struct MessagingV1Deactivation {
    /// Redirect url to the list of deactivated numbers.
    #[serde(rename = "redirect_to", skip_serializing_if = "Option::is_none")]
    pub redirect_to: Option<String>,
}

impl MessagingV1Deactivation {
    pub fn new() -> MessagingV1Deactivation {
        MessagingV1Deactivation { redirect_to: None }
    }
}

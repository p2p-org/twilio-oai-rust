/*
 * Twilio - Events
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListSubscriptionResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListSchemaVersionResponseMeta>>,
    #[serde(rename = "subscriptions", skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<crate::models::EventsV1Subscription>>,
}

impl ListSubscriptionResponse {
    pub fn new() -> ListSubscriptionResponse {
        ListSubscriptionResponse {
            meta: None,
            subscriptions: None,
        }
    }
}

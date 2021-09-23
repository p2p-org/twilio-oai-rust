/*
 * Twilio - Supersim
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListSmsCommandResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListCommandResponseMeta>>,
    #[serde(rename = "sms_commands", skip_serializing_if = "Option::is_none")]
    pub sms_commands: Option<Vec<crate::models::SupersimV1SmsCommand>>,
}

impl ListSmsCommandResponse {
    pub fn new() -> ListSmsCommandResponse {
        ListSmsCommandResponse {
            meta: None,
            sms_commands: None,
        }
    }
}



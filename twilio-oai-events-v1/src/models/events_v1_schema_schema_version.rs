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
pub struct EventsV1SchemaSchemaVersion {
    /// The date the schema version was created.
    #[serde(rename = "date_created", skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    /// The unique identifier of the schema.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "raw", skip_serializing_if = "Option::is_none")]
    pub raw: Option<String>,
    /// The version of this schema.
    #[serde(rename = "schema_version", skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<i32>,
    /// The URL of this resource.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl EventsV1SchemaSchemaVersion {
    pub fn new() -> EventsV1SchemaSchemaVersion {
        EventsV1SchemaSchemaVersion {
            date_created: None,
            id: None,
            raw: None,
            schema_version: None,
            url: None,
        }
    }
}

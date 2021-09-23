/*
 * Twilio - Sync
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListSyncMapResponse {
    #[serde(rename = "maps", skip_serializing_if = "Option::is_none")]
    pub maps: Option<Vec<crate::models::SyncV1ServiceSyncMap>>,
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListServiceResponseMeta>>,
}

impl ListSyncMapResponse {
    pub fn new() -> ListSyncMapResponse {
        ListSyncMapResponse {
            maps: None,
            meta: None,
        }
    }
}



/*
 * Twilio - Sync
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ListSyncListPermissionResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::ListServiceResponseMeta>>,
    #[serde(rename = "permissions", skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<crate::models::SyncV1ServiceSyncListSyncListPermission>>,
}

impl ListSyncListPermissionResponse {
    pub fn new() -> ListSyncListPermissionResponse {
        ListSyncListPermissionResponse {
            meta: None,
            permissions: None,
        }
    }
}

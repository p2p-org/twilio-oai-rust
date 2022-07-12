/*
 * Twilio - Chat
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.30.0
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::{configuration, Error};
use crate::apis::ResponseContent;

/// struct for passing parameters to the method [`update_channel`]
#[derive(Clone, Debug, Default)]
pub struct UpdateChannelParams {
    /// The unique SID identifier of the Service.
    pub service_sid: String,
    /// A 34 character string that uniquely identifies this Channel.
    pub sid: String,
    /// The X-Twilio-Webhook-Enabled HTTP request header
    pub x_twilio_webhook_enabled: Option<String>,
    /// The unique ID of the [Messaging Service](https://www.twilio.com/docs/sms/services/api) this channel belongs to.
    pub messaging_service_sid: Option<String>,
    /// TThe Type for this Channel to migrate to. Can only be `private`. Migration to 'public' is not allowed.
    pub _type: Option<String>,
}

/// struct for typed successes of method [`update_channel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateChannelSuccess {
    Status200(crate::models::ChatV3Channel),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_channel`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateChannelError {
    UnknownValue(serde_json::Value),
}

/// Update a specific Channel.
pub async fn update_channel(
    configuration: &configuration::Configuration,
    params: UpdateChannelParams,
) -> Result<ResponseContent<UpdateChannelSuccess>, Error<UpdateChannelError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_sid = params.service_sid;
    let sid = params.sid;
    let x_twilio_webhook_enabled = params.x_twilio_webhook_enabled;
    let messaging_service_sid = params.messaging_service_sid;
    let _type = params._type;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v3/Services/{ServiceSid}/Channels/{Sid}",
        local_var_configuration.base_path,
        ServiceSid = crate::apis::urlencode(service_sid),
        Sid = crate::apis::urlencode(sid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(local_var_param_value) = x_twilio_webhook_enabled {
        local_var_req_builder = local_var_req_builder.header(
            "X-Twilio-Webhook-Enabled",
            local_var_param_value.to_string(),
        );
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = messaging_service_sid {
        local_var_form_params.insert("MessagingServiceSid", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = _type {
        local_var_form_params.insert("Type", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<UpdateChannelSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<UpdateChannelError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

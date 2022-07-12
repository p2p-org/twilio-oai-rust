/*
 * Twilio - Monitor
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

/// struct for passing parameters to the method [`fetch_alert`]
#[derive(Clone, Debug, Default)]
pub struct FetchAlertParams {
    /// The SID of the Alert resource to fetch.
    pub sid: String,
}

/// struct for passing parameters to the method [`fetch_event`]
#[derive(Clone, Debug, Default)]
pub struct FetchEventParams {
    /// The SID of the Event resource to fetch.
    pub sid: String,
}

/// struct for passing parameters to the method [`list_alert`]
#[derive(Clone, Debug, Default)]
pub struct ListAlertParams {
    /// Only show alerts for this log-level.  Can be: `error`, `warning`, `notice`, or `debug`.
    pub log_level: Option<String>,
    /// Only include alerts that occurred on or after this date and time. Specify the date and time in GMT and format as `YYYY-MM-DD` or `YYYY-MM-DDThh:mm:ssZ`. Queries for alerts older than 30 days are not supported.
    pub start_date: Option<String>,
    /// Only include alerts that occurred on or before this date and time. Specify the date and time in GMT and format as `YYYY-MM-DD` or `YYYY-MM-DDThh:mm:ssZ`. Queries for alerts older than 30 days are not supported.
    pub end_date: Option<String>,
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i32>,
}

/// struct for passing parameters to the method [`list_event`]
#[derive(Clone, Debug, Default)]
pub struct ListEventParams {
    /// Only include events initiated by this Actor. Useful for auditing actions taken by specific users or API credentials.
    pub actor_sid: Option<String>,
    /// Only include events of this [Event Type](https://www.twilio.com/docs/usage/monitor-events#event-types).
    pub event_type: Option<String>,
    /// Only include events that refer to this resource. Useful for discovering the history of a specific resource.
    pub resource_sid: Option<String>,
    /// Only include events that originated from this IP address. Useful for tracking suspicious activity originating from the API or the Twilio Console.
    pub source_ip_address: Option<String>,
    /// Only include events that occurred on or after this date. Specify the date in GMT and [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format.
    pub start_date: Option<String>,
    /// Only include events that occurred on or before this date. Specify the date in GMT and [ISO 8601](https://en.wikipedia.org/wiki/ISO_8601) format.
    pub end_date: Option<String>,
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i32>,
}

/// struct for typed successes of method [`fetch_alert`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchAlertSuccess {
    Status200(crate::models::MonitorV1AlertInstance),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`fetch_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchEventSuccess {
    Status200(crate::models::MonitorV1Event),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_alert`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAlertSuccess {
    Status200(crate::models::ListAlertResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListEventSuccess {
    Status200(crate::models::ListEventResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_alert`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchAlertError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchEventError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_alert`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListAlertError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_event`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListEventError {
    UnknownValue(serde_json::Value),
}

///
pub async fn fetch_alert(
    configuration: &configuration::Configuration,
    params: FetchAlertParams,
) -> Result<ResponseContent<FetchAlertSuccess>, Error<FetchAlertError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let sid = params.sid;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/Alerts/{Sid}",
        local_var_configuration.base_path,
        Sid = crate::apis::urlencode(sid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<FetchAlertSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<FetchAlertError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

///
pub async fn fetch_event(
    configuration: &configuration::Configuration,
    params: FetchEventParams,
) -> Result<ResponseContent<FetchEventSuccess>, Error<FetchEventError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let sid = params.sid;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!(
        "{}/v1/Events/{Sid}",
        local_var_configuration.base_path,
        Sid = crate::apis::urlencode(sid)
    );
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<FetchEventSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<FetchEventError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

///
pub async fn list_alert(
    configuration: &configuration::Configuration,
    params: ListAlertParams,
) -> Result<ResponseContent<ListAlertSuccess>, Error<ListAlertError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let log_level = params.log_level;
    let start_date = params.start_date;
    let end_date = params.end_date;
    let page_size = params.page_size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/Alerts", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = log_level {
        local_var_req_builder =
            local_var_req_builder.query(&[("LogLevel", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_date {
        local_var_req_builder =
            local_var_req_builder.query(&[("StartDate", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_date {
        local_var_req_builder =
            local_var_req_builder.query(&[("EndDate", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("PageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<ListAlertSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<ListAlertError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a list of events in the account, sorted by event-date.
pub async fn list_event(
    configuration: &configuration::Configuration,
    params: ListEventParams,
) -> Result<ResponseContent<ListEventSuccess>, Error<ListEventError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let actor_sid = params.actor_sid;
    let event_type = params.event_type;
    let resource_sid = params.resource_sid;
    let source_ip_address = params.source_ip_address;
    let start_date = params.start_date;
    let end_date = params.end_date;
    let page_size = params.page_size;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/Events", local_var_configuration.base_path);
    let mut local_var_req_builder =
        local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = actor_sid {
        local_var_req_builder =
            local_var_req_builder.query(&[("ActorSid", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = event_type {
        local_var_req_builder =
            local_var_req_builder.query(&[("EventType", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = resource_sid {
        local_var_req_builder =
            local_var_req_builder.query(&[("ResourceSid", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = source_ip_address {
        local_var_req_builder =
            local_var_req_builder.query(&[("SourceIpAddress", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = start_date {
        local_var_req_builder =
            local_var_req_builder.query(&[("StartDate", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = end_date {
        local_var_req_builder =
            local_var_req_builder.query(&[("EndDate", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page_size {
        local_var_req_builder =
            local_var_req_builder.query(&[("PageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder =
            local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(
            local_var_auth_conf.0.to_owned(),
            local_var_auth_conf.1.to_owned(),
        );
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<ListEventSuccess> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<ListEventError> =
            serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent {
            status: local_var_status,
            content: local_var_content,
            entity: local_var_entity,
        };
        Err(Error::ResponseError(local_var_error))
    }
}

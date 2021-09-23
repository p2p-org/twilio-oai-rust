/*
 * Twilio - Pricing
 *
 * This is the public Twilio REST API.
 *
 * The version of the OpenAPI document: 1.20.2
 * Contact: support@twilio.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`fetch_trunking_country`]
#[derive(Clone, Debug, Default)]
pub struct FetchTrunkingCountryParams {
    /// The [ISO country code](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) of the origin-based voice pricing information to fetch.
    pub iso_country: String
}

/// struct for passing parameters to the method [`fetch_trunking_number`]
#[derive(Clone, Debug, Default)]
pub struct FetchTrunkingNumberParams {
    /// The destination phone number, in [E.164](https://www.twilio.com/docs/glossary/what-e164) format, for which to fetch the origin-based voice pricing information. E.164 format consists of a + followed by the country code and subscriber number.
    pub destination_number: String,
    /// The origination phone number, in [E.164](https://www.twilio.com/docs/glossary/what-e164) format, for which to fetch the origin-based voice pricing information. E.164 format consists of a + followed by the country code and subscriber number.
    pub origination_number: Option<String>
}

/// struct for passing parameters to the method [`fetch_voice_country`]
#[derive(Clone, Debug, Default)]
pub struct FetchVoiceCountryParams {
    /// The [ISO country code](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) of the origin-based voice pricing information to fetch.
    pub iso_country: String
}

/// struct for passing parameters to the method [`fetch_voice_number`]
#[derive(Clone, Debug, Default)]
pub struct FetchVoiceNumberParams {
    /// The destination phone number, in [E.164](https://www.twilio.com/docs/glossary/what-e164) format, for which to fetch the origin-based voice pricing information. E.164 format consists of a + followed by the country code and subscriber number.
    pub destination_number: String,
    /// The origination phone number, in [E.164](https://www.twilio.com/docs/glossary/what-e164) format, for which to fetch the origin-based voice pricing information. E.164 format consists of a + followed by the country code and subscriber number.
    pub origination_number: Option<String>
}

/// struct for passing parameters to the method [`list_trunking_country`]
#[derive(Clone, Debug, Default)]
pub struct ListTrunkingCountryParams {
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i32>
}

/// struct for passing parameters to the method [`list_voice_country`]
#[derive(Clone, Debug, Default)]
pub struct ListVoiceCountryParams {
    /// How many resources to return in each list page. The default is 50, and the maximum is 1000.
    pub page_size: Option<i32>
}


/// struct for typed successes of method [`fetch_trunking_country`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchTrunkingCountrySuccess {
    Status200(crate::models::PricingV2TrunkingCountryInstance),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`fetch_trunking_number`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchTrunkingNumberSuccess {
    Status200(crate::models::PricingV2TrunkingNumber),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`fetch_voice_country`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchVoiceCountrySuccess {
    Status200(crate::models::PricingV2VoiceVoiceCountryInstance),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`fetch_voice_number`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchVoiceNumberSuccess {
    Status200(crate::models::PricingV2VoiceVoiceNumber),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_trunking_country`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTrunkingCountrySuccess {
    Status200(crate::models::ListTrunkingCountryResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed successes of method [`list_voice_country`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListVoiceCountrySuccess {
    Status200(crate::models::ListVoiceCountryResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_trunking_country`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchTrunkingCountryError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_trunking_number`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchTrunkingNumberError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_voice_country`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchVoiceCountryError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_voice_number`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchVoiceNumberError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_trunking_country`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListTrunkingCountryError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_voice_country`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListVoiceCountryError {
    UnknownValue(serde_json::Value),
}


/// Fetch a specific Country.
pub async fn fetch_trunking_country(configuration: &configuration::Configuration, params: FetchTrunkingCountryParams) -> Result<ResponseContent<FetchTrunkingCountrySuccess>, Error<FetchTrunkingCountryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let iso_country = params.iso_country;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/Trunking/Countries/{IsoCountry}", local_var_configuration.base_path, IsoCountry=crate::apis::urlencode(iso_country));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<FetchTrunkingCountrySuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<FetchTrunkingCountryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch pricing information for a specific destination and, optionally, origination phone number.
pub async fn fetch_trunking_number(configuration: &configuration::Configuration, params: FetchTrunkingNumberParams) -> Result<ResponseContent<FetchTrunkingNumberSuccess>, Error<FetchTrunkingNumberError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let destination_number = params.destination_number;
    let origination_number = params.origination_number;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/Trunking/Numbers/{DestinationNumber}", local_var_configuration.base_path, DestinationNumber=crate::apis::urlencode(destination_number));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = origination_number {
        local_var_req_builder = local_var_req_builder.query(&[("OriginationNumber", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<FetchTrunkingNumberSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<FetchTrunkingNumberError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch a specific Country.
pub async fn fetch_voice_country(configuration: &configuration::Configuration, params: FetchVoiceCountryParams) -> Result<ResponseContent<FetchVoiceCountrySuccess>, Error<FetchVoiceCountryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let iso_country = params.iso_country;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/Voice/Countries/{IsoCountry}", local_var_configuration.base_path, IsoCountry=crate::apis::urlencode(iso_country));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<FetchVoiceCountrySuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<FetchVoiceCountryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Fetch pricing information for a specific destination and, optionally, origination phone number.
pub async fn fetch_voice_number(configuration: &configuration::Configuration, params: FetchVoiceNumberParams) -> Result<ResponseContent<FetchVoiceNumberSuccess>, Error<FetchVoiceNumberError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let destination_number = params.destination_number;
    let origination_number = params.origination_number;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/Voice/Numbers/{DestinationNumber}", local_var_configuration.base_path, DestinationNumber=crate::apis::urlencode(destination_number));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = origination_number {
        local_var_req_builder = local_var_req_builder.query(&[("OriginationNumber", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<FetchVoiceNumberSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<FetchVoiceNumberError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list_trunking_country(configuration: &configuration::Configuration, params: ListTrunkingCountryParams) -> Result<ResponseContent<ListTrunkingCountrySuccess>, Error<ListTrunkingCountryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page_size = params.page_size;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/Trunking/Countries", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("PageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<ListTrunkingCountrySuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<ListTrunkingCountryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn list_voice_country(configuration: &configuration::Configuration, params: ListVoiceCountryParams) -> Result<ResponseContent<ListVoiceCountrySuccess>, Error<ListVoiceCountryError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let page_size = params.page_size;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v2/Voice/Countries", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = page_size {
        local_var_req_builder = local_var_req_builder.query(&[("PageSize", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        let local_var_entity: Option<ListVoiceCountrySuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<ListVoiceCountryError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}


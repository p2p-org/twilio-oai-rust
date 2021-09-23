/*
 * Twilio - Lookups
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

/// struct for passing parameters to the method [`fetch_phone_number`]
#[derive(Clone, Debug, Default)]
pub struct FetchPhoneNumberParams {
    /// The phone number to lookup in [E.164](https://www.twilio.com/docs/glossary/what-e164) format, which consists of a + followed by the country code and subscriber number.
    pub phone_number: String,
    /// The [ISO country code](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) of the phone number to fetch. This is used to specify the country when the phone number is provided in a national format.
    pub country_code: Option<String>,
    /// The type of information to return. Can be: `carrier` or `caller-name`. The default is null.  Carrier information costs $0.005 per phone number looked up.  Caller Name information is currently available only in the US and costs $0.01 per phone number looked up.  To retrieve both types on information, specify this parameter twice; once with `carrier` and once with `caller-name` as the value.
    pub _type: Option<Vec<String>>,
    /// The `unique_name` of an Add-on you would like to invoke. Can be the `unique_name` of an Add-on that is installed on your account. You can specify multiple instances of this parameter to invoke multiple Add-ons. For more information about  Add-ons, see the [Add-ons documentation](https://www.twilio.com/docs/add-ons).
    pub add_ons: Option<Vec<String>>,
    /// Data specific to the add-on you would like to invoke. The content and format of this value depends on the add-on.
    pub add_ons_data: Option<serde_json::Value>
}


/// struct for typed successes of method [`fetch_phone_number`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchPhoneNumberSuccess {
    Status200(crate::models::LookupsV1PhoneNumber),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`fetch_phone_number`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FetchPhoneNumberError {
    UnknownValue(serde_json::Value),
}


pub async fn fetch_phone_number(configuration: &configuration::Configuration, params: FetchPhoneNumberParams) -> Result<ResponseContent<FetchPhoneNumberSuccess>, Error<FetchPhoneNumberError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let phone_number = params.phone_number;
    let country_code = params.country_code;
    let _type = params._type;
    let add_ons = params.add_ons;
    let add_ons_data = params.add_ons_data;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/PhoneNumbers/{PhoneNumber}", local_var_configuration.base_path, PhoneNumber=crate::apis::urlencode(phone_number));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = country_code {
        local_var_req_builder = local_var_req_builder.query(&[("CountryCode", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = _type {
        local_var_req_builder = local_var_req_builder.query(&[("Type", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
    }
    if let Some(ref local_var_str) = add_ons {
        local_var_req_builder = local_var_req_builder.query(&[("AddOns", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
    }
    if let Some(ref local_var_str) = add_ons_data {
        local_var_req_builder = local_var_req_builder.query(&[("AddOnsData", &local_var_str.to_string())]);
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
        let local_var_entity: Option<FetchPhoneNumberSuccess> = serde_json::from_str(&local_var_content).ok();
        let local_var_result = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Ok(local_var_result)
    } else {
        let local_var_entity: Option<FetchPhoneNumberError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}


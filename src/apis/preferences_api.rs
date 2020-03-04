/*
 * Firefly III API
 *
 * This is the official documentation of the Firefly III API. You can find accompanying documentation on the website of Firefly III itself (see below). Please report any bugs or issues. This version of the API is live from version v4.7.9 and onwards. You may use the \"Authorize\" button to try the API below. 
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: thegrumpydictator@gmail.com
 * Generated by: https://openapi-generator.tech
 */

use async_trait::async_trait;
use std::sync::Arc;
use std::borrow::Borrow;
#[allow(unused_imports)]
use std::option::Option;

use reqwest;

use super::{Error, configuration};

pub struct PreferencesApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl PreferencesApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> PreferencesApiClient {
        PreferencesApiClient {
            configuration,
        }
    }
}

#[async_trait]
pub trait PreferencesApi {
    async fn get_preference(&self, name: &str) -> Result<crate::models::PreferenceSingle, Error>;
    async fn list_preference(&self, page: Option<i32>) -> Result<crate::models::PreferenceArray, Error>;
    async fn update_preference(&self, name: &str, preference: crate::models::Preference) -> Result<crate::models::PreferenceSingle, Error>;
}

#[async_trait]
impl PreferencesApi for PreferencesApiClient {
    async fn get_preference(&self, name: &str) -> Result<crate::models::PreferenceSingle, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/preferences/{name}", configuration.base_path, name=crate::apis::urlencode(name));
        let mut req_builder = client.request(::reqwest::Method::GET, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req).await?.error_for_status()?.json().await?)
    }

    async fn list_preference(&self, page: Option<i32>) -> Result<crate::models::PreferenceArray, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/preferences", configuration.base_path);
        let mut req_builder = client.request(::reqwest::Method::GET, uri_str.as_str());

        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req).await?.error_for_status()?.json().await?)
    }

    async fn update_preference(&self, name: &str, preference: crate::models::Preference) -> Result<crate::models::PreferenceSingle, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/preferences/{name}", configuration.base_path, name=crate::apis::urlencode(name));
        let mut req_builder = client.request(::reqwest::Method::PUT, uri_str.as_str());

        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(ref token) = configuration.oauth_access_token {
            req_builder = req_builder.bearer_auth(token.to_owned());
        };
        req_builder = req_builder.json(&preference);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req).await?.error_for_status()?.json().await?)
    }

}

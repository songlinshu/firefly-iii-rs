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

pub struct SummaryApiClient {
    configuration: Arc<configuration::Configuration>,
}

impl SummaryApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> SummaryApiClient {
        SummaryApiClient {
            configuration,
        }
    }
}

#[async_trait]
pub trait SummaryApi {
    async fn get_basic_summary(&self, start: String, end: String, currency_code: Option<&str>) -> Result<Vec<crate::models::BasicSummaryEntry>, Error>;
}

#[async_trait]
impl SummaryApi for SummaryApiClient {
    async fn get_basic_summary(&self, start: String, end: String, currency_code: Option<&str>) -> Result<Vec<crate::models::BasicSummaryEntry>, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/api/v1/summary/basic", configuration.base_path);
        let mut req_builder = client.request(::reqwest::Method::GET, uri_str.as_str());

        req_builder = req_builder.query(&[("start", &start.to_string())]);
        req_builder = req_builder.query(&[("end", &end.to_string())]);
        if let Some(ref s) = currency_code {
            req_builder = req_builder.query(&[("currency_code", &s.to_string())]);
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

}

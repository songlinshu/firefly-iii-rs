/*
 * Firefly III API
 *
 * This is the official documentation of the Firefly III API. You can find accompanying documentation on the website of Firefly III itself (see below). Please report any bugs or issues. This version of the API is live from version v4.7.9 and onwards. You may use the \"Authorize\" button to try the API below. 
 *
 * The version of the OpenAPI document: 1.1.0
 * Contact: james@firefly-iii.org
 * Generated by: https://openapi-generator.tech
 */


use serde::{Deserialize, Serialize};



#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountArray {
    #[serde(rename = "data")]
    pub data: Vec<crate::models::AccountRead>,
    #[serde(rename = "meta")]
    pub meta: crate::models::Meta,
}

impl AccountArray {
    pub fn new(data: Vec<crate::models::AccountRead>, meta: crate::models::Meta) -> AccountArray {
        AccountArray {
            data,
            meta,
        }
    }
}



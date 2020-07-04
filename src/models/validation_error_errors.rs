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
pub struct ValidationErrorErrors {
    #[serde(rename = "field1", skip_serializing_if = "Option::is_none")]
    pub field1: Option<Vec<String>>,
    #[serde(rename = "field2", skip_serializing_if = "Option::is_none")]
    pub field2: Option<Vec<String>>,
}

impl ValidationErrorErrors {
    pub fn new() -> ValidationErrorErrors {
        ValidationErrorErrors {
            field1: None,
            field2: None,
        }
    }
}



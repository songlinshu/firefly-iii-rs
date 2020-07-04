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
pub struct TagRead {
    /// Immutable value
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "attributes")]
    pub attributes: crate::models::TagModel,
    #[serde(rename = "links")]
    pub links: crate::models::ObjectLink,
}

impl TagRead {
    pub fn new(_type: String, id: String, attributes: crate::models::TagModel, links: crate::models::ObjectLink) -> TagRead {
        TagRead {
            _type,
            id,
            attributes,
            links,
        }
    }
}



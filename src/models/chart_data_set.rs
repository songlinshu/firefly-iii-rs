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
pub struct ChartDataSet {
    /// This is the title of the current set. It can refer to an account, a budget or another object (by name).
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The currency ID of the currency associated to the data in the entries.
    #[serde(rename = "currency_id", skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<i32>,
    #[serde(rename = "currency_code", skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "currency_symbol", skip_serializing_if = "Option::is_none")]
    pub currency_symbol: Option<String>,
    /// Number of decimals for the currency associated to the data in the entries.
    #[serde(rename = "currency_decimal_places", skip_serializing_if = "Option::is_none")]
    pub currency_decimal_places: Option<i32>,
    /// Indicated the type of chart that is expected to be rendered. You can safely ignore this if you want.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// Used to indicate the Y axis for this data set. Is usually between 0 and 1 (left and right side of the chart).
    #[serde(rename = "yAxisID", skip_serializing_if = "Option::is_none")]
    pub y_axis_id: Option<i32>,
    /// The actual entries for this data set. They 'key' value is the label for the data point. The value is the actual (numerical) value.
    #[serde(rename = "entries", skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<crate::models::ChartDataPoint>>,
}

impl ChartDataSet {
    pub fn new() -> ChartDataSet {
        ChartDataSet {
            label: None,
            currency_id: None,
            currency_code: None,
            currency_symbol: None,
            currency_decimal_places: None,
            _type: None,
            y_axis_id: None,
            entries: None,
        }
    }
}



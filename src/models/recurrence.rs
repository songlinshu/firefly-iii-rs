/*
 * Firefly III API
 *
 * This is the official documentation of the Firefly III API. You can find accompanying documentation on the website of Firefly III itself (see below). Please report any bugs or issues. This version of the API is live from version v4.7.9 and onwards. You may use the \"Authorize\" button to try the API below. 
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: thegrumpydictator@gmail.com
 * Generated by: https://openapi-generator.tech
 */


use serde::{Deserialize, Serialize};



#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Recurrence {
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(rename = "type")]
    pub _type: Type,
    #[serde(rename = "title")]
    pub title: String,
    /// Not to be confused with the description of the actual transaction(s) being created.
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// First time the recurring transaction will fire. Must be after today.
    #[serde(rename = "first_date")]
    pub first_date: String,
    /// First time the recurring transaction will fire. Must be after today.
    #[serde(rename = "latest_date", skip_serializing_if = "Option::is_none")]
    pub latest_date: Option<String>,
    /// Date until the recurring transaction can fire. Use either this field or repetitions.
    #[serde(rename = "repeat_until", skip_serializing_if = "Option::is_none")]
    pub repeat_until: Option<String>,
    /// Max number of created transactions. Use either this field or repeat_until.
    #[serde(rename = "nr_of_repetitions", skip_serializing_if = "Option::is_none")]
    pub nr_of_repetitions: Option<i32>,
    /// Whether or not to fire the rules after the creation of a transaction.
    #[serde(rename = "apply_rules", skip_serializing_if = "Option::is_none")]
    pub apply_rules: Option<bool>,
    /// If the recurrence is even active.
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "notes", skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "repetitions", skip_serializing_if = "Option::is_none")]
    pub repetitions: Option<Vec<crate::models::RecurrenceRepetition>>,
    #[serde(rename = "transactions", skip_serializing_if = "Option::is_none")]
    pub transactions: Option<Vec<crate::models::RecurrenceTransaction>>,
}

impl Recurrence {
    pub fn new(_type: Type, title: String, first_date: String) -> Recurrence {
        Recurrence {
            created_at: None,
            updated_at: None,
            _type,
            title,
            description: None,
            first_date,
            latest_date: None,
            repeat_until: None,
            nr_of_repetitions: None,
            apply_rules: None,
            active: None,
            notes: None,
            repetitions: None,
            transactions: None,
        }
    }
}

/// 
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "withdrawal")]
    Withdrawal,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "deposit")]
    Deposit,
    #[serde(rename = "opening-balance")]
    OpeningBalance,
    #[serde(rename = "reconciliation")]
    Reconciliation,
}


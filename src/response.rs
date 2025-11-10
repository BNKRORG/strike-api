//! Strike responses

use serde::{Deserialize, Deserializer, de};

/// Strike balance
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Balance {
    /// The currency of the balance
    pub currency: String,
    /// The balance that is currently available for trading (currency conversion). Includes settled and any non spent pending amount
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    pub current: f64,
    /// The balance of all deposits, in all currencies, that are pending settlement denominated in this currency. This number impacts the available balance and can cause the available balance to be lower than the current balance
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    pub pending: f64,
    /// The balance currently being withdrawn
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    pub outgoing: f64,
    /// The balance that is reserved to be spent. E.g. for pending withdrawals, target orders, etc.
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    pub reserved: f64,
    /// The balance that is available to be spent and sent out without any restrictions. Available balance might be lower than the current because of the pending deposits
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    pub available: f64,
    /// The sum of the available and outgoing balance
    #[serde(deserialize_with = "deserialize_string_to_f64")]
    pub total: f64,
}

impl Balance {
    pub(crate) fn new<T>(currency: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            currency: currency.into(),
            current: 0.0,
            pending: 0.0,
            outgoing: 0.0,
            reserved: 0.0,
            available: 0.0,
            total: 0.0,
        }
    }
}

fn deserialize_string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    s.parse().map_err(de::Error::custom)
}

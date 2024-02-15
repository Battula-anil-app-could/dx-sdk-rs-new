use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Account holds an Account's information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DctBalance {
    pub token_identifier: String,
    pub balance: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DctBalanceData {
    pub dcts: HashMap<String, DctBalance>,
}

// DctBalanceResponse holds the dct balance endpoint response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DctBalanceResponse {
    pub data: Option<DctBalanceData>,
    pub error: String,
    pub code: String,
}

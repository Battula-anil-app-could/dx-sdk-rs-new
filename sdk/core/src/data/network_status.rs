use serde::{Deserialize, Serialize};

// NetworkStatus holds the network status details of a specified shard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatus {
    #[serde(rename = "moa_current_round")]
    pub current_round: u64,
    #[serde(rename = "moa_epoch_number")]
    pub epoch_number: u64,
    #[serde(rename = "moa_nonce")]
    pub nonce: u64,
    #[serde(rename = "moa_nonce_at_epoch_start")]
    pub nonce_at_epoch_start: u64,
    #[serde(rename = "moa_nonces_passed_in_current_epoch")]
    pub nonces_passed_in_current_epoch: u64,
    #[serde(rename = "moa_round_at_epoch_start")]
    pub round_at_epoch_start: u64,
    #[serde(rename = "moa_rounds_passed_in_current_epoch")]
    pub rounds_passed_in_current_epoch: u64,
    #[serde(rename = "moa_rounds_per_epoch")]
    pub rounds_per_epoch: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatusData {
    pub status: NetworkStatus,
}

// NetworkStatusResponse holds the network status response (for a specified shard)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStatusResponse {
    pub error: String,
    pub code: String,
    pub data: Option<NetworkStatusData>,
}

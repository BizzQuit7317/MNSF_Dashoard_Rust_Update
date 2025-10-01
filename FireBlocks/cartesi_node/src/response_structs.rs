use serde::{Serialize, Deserialize};

//network_identifier_struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaretestNetworkResponse {
    pub balance:f64,
    pub lockedd:f64,
    pub delegated:f64,
    pub total_balance:f64,
}
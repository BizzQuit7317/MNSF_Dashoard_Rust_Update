use serde::{Serialize, Deserialize};
use serde_json::Value;

//available _balance_respone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableResponse {
    pub jsonrpc: String,
    pub id: String,
    pub result: u64
}

impl Default for AvailableResponse {
    fn default() -> Self {
        AvailableResponse {
            jsonrpc: String::new(),
            id: String::new(),
            result: 0
        }
    }
}

//delegated _balance_respone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegatedResponse {
    pub jsonrpc: String,
    pub id: String,
    pub result: Vec<Delagted_ResultResponse>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delagted_ResultResponse {
    pub Undelegations: Vec<Value>,
    pub amount: u64,
    pub delegator_address: String,
    pub reward: f64,
    pub  validator_address: String,
}

impl Default for DelegatedResponse {
    fn default() -> Self {
        Self {
            jsonrpc: String::new(),
            id: String::new(),
            result: Vec::new(), // Vec has Default built-in
        }
    }
}
use serde::{Serialize, Deserialize};
use serde_json::Value;

//Common structs with multi use
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationResponse {
    pub next_key: Option<String>,
    pub total: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance_Response {
    pub denom: String,
    pub amount: String,
}


// Delegation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationResponse {
    pub delegation_responses: Option<Vec<Delegation_BalanceResponse>>,
    pub pagination: PaginationResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delegation_BalanceResponse {
    pub delegation: Delegation_DelegationResponse,
    pub balance: Balance_Response
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delegation_DelegationResponse {
    pub delegator_address: String,
    pub validator_address: String,
    pub shares: String
}

impl Default for DelegationResponse {
    fn default() -> Self {
        DelegationResponse {
            delegation_responses: None,
            pagination: PaginationResponse {
                next_key: None,
                total: String::new(),
            },
        }
    }
}

//Available respones
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableResponse {
    pub balances: Option<Vec<Balance_Response>>,
    pub pagination: PaginationResponse,
}

impl Default for AvailableResponse {
    fn default() -> Self {
        AvailableResponse {
            balances: None,
            pagination: PaginationResponse {
                next_key: None,
                total: String::new(),
            },
        }
    }
}

// No implement response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoImplResponse {
    pub code: String,
    pub message: String,
    pub details: Option<Vec<Value>>
}

impl Default for NoImplResponse {
    fn default() -> Self {
        NoImplResponse {
            code: String::new(),
            message: String::new(),
            details: None
        }
    }
}
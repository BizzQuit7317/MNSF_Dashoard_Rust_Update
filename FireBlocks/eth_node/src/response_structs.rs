use serde::{Serialize, Deserialize};
use serde_json::Value;

//Validator response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorResponse {
    pub status: String,
    pub data: Option<Vec<Validator_DataResponse>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validator_DataResponse {
    pub publickey: String,
    pub valid_signature: bool,
    pub validatorindex: u64  
}

impl Default for ValidatorResponse {
    fn default() -> Self {
        ValidatorResponse {
            status: String::new(),
            data: None
        }
    }
}

//Deligation Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationResponse {
    pub status: String,
    pub data: Option<Vec<Delegation_DataResponse>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Delegation_DataResponse {
    pub amount: u64, 
    pub block_number: u64, 
    pub block_ts: u64, 
    pub from_address: String, 
    pub merkletree_index: String, 
    pub publickey: String,
    pub removed: bool, 
    pub signature: String, 
    pub tx_hash: String, 
    pub tx_index: u64, 
    pub tx_input: String, 
    pub valid_signature: bool, 
    pub withdrawal_credentials: String,
}

impl Default for DelegationResponse {
    fn default() -> Self {
        DelegationResponse {
            status: String::new(),
            data: None
        }
    }
}
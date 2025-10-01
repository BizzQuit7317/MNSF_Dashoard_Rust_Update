use serde::{Serialize, Deserialize};
use serde_json::Value;

//network_identifier_struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkListResponse {
    pub network_identifiers: Vec<NetworkIdentifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkIdentifier{
    pub blockchain: String,
    pub network: String,
}

impl Default for NetworkListResponse {
    fn default() -> Self {
        NetworkListResponse {
            network_identifiers: vec![],
        }
    }
}

//Available response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableResponse {
    pub block_identifier: Block_Identifier,
    pub balances: Vec<Balance>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block_Identifier{
    pub index: u64,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Balance {
    pub value: String,
    pub currency: Currency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    pub symbol: String,
    pub decimals: u64,
}

impl Default for AvailableResponse {
    fn default() -> Self {
        AvailableResponse {
            block_identifier: Block_Identifier {
                index:0,
                hash:String::new(),
            },
            balances: vec![]
        }
    }
}

//Staked response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakedResponse {
    pub jsonrpc: String,
    pub result: StakedResult,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StakedResult{
    pub block_hash: String,
    pub block_height: u64,
    pub logs: Vec<String>,
    pub result: Vec<u8>
}

impl Default for StakedResponse {
    fn default() -> Self {
        StakedResponse {
            jsonrpc: String::new(),
            result: StakedResult {
                block_hash: String::new(),
                block_height: 0,
                logs: vec![],
                result: vec![],
            },
            id: String::new()
        }
    }
}
use serde::{Serialize, Deserialize};
use serde_json::Value;

//BTC_response_struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BTC_Response{
    pub address: String,
    pub chain_stats: BTC_Stats,
    pub mempool_stats: BTC_Stats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BTC_Stats {
    pub funded_txo_count:u32,
    pub funded_txo_sum:u32,
    pub spent_txo_count:u32,
    pub spent_txo_sum:u32,
    pub tx_count:u32,
}

impl Default for BTC_Response {
    fn default() -> Self {
        BTC_Response {
            address: String::new(),
            chain_stats: BTC_Stats {
                funded_txo_count:0,
                funded_txo_sum:0,
                spent_txo_count:0,
                spent_txo_sum:0,
                tx_count:0,
            },
            mempool_stats: BTC_Stats {
                funded_txo_count:0,
                funded_txo_sum:0,
                spent_txo_count:0,
                spent_txo_sum:0,
                tx_count:0,
            },

        }
    }
}

//Flow_response_struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flow_Response {
    pub address: String,
    pub balance: String,
    pub _expandable: Flow_ExpandableResponse,
    pub _links: Flow_LinksResponse
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flow_ExpandableResponse {
    pub keys: String,
    pub contracts: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flow_LinksResponse {
    pub _self: String
}


impl Default for Flow_Response {
    fn default() -> Self {
        Flow_Response{
            address: String::new(),
            balance: String::new(),
            _expandable: Flow_ExpandableResponse {
                keys: String::new(),
                contracts: String::new(),
            },
            _links: Flow_LinksResponse {
                _self: String::new(),
            }
        }
    }
}

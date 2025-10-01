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
    pub sub_network_identifier: Option<SubNetworkIdentifier>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubNetworkIdentifier{
    pub network:String,
}

impl Default for NetworkListResponse {
    fn default() -> Self {
        NetworkListResponse {
            network_identifiers: vec![],
        }
    }
}


//c_chain_struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C_ChainResponse {
    pub block_identifier: C_BlockIdentifier,
    pub balances: Vec<C_Balance>,
    pub metadata: C_Metadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C_BlockIdentifier {
    pub index: u64,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C_Balance {
    pub value: String,
    pub currency: C_Balance_Currency
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C_Balance_Currency {
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct C_Metadata {
    pub nonce: String    
}

impl Default for C_ChainResponse {
    fn default() -> Self {
        C_ChainResponse {
            block_identifier: C_BlockIdentifier {
                index: 0,
                hash: String::new(),
            },
            balances: Vec::new(),
            metadata: C_Metadata {
                nonce: String::new(),
            },
        }
    }
}

//utox_chain_struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UTXO_ChainResponse {
    pub jsonrpc: String,
    pub result: UTXO_Result,
    pub id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UTXO_Result {
    pub numFetched: String,
    pub utxos: Option<Vec<String>>,
    pub endIndex: UTXO_EndIndex,
    pub encoding: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UTXO_EndIndex {
    pub address: String,
    pub utxo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UTXO {
    pub id:u32,
    pub tx_id:String,
    pub output_index:u32,
    pub asset_id:String,
    pub output_id:u32,
    pub amount:u32,
    pub locktime:u32,
    pub threshold:u32,
    pub address_length:u32,
    pub address:String,
}

impl Default for UTXO_ChainResponse {
    fn default() -> Self {
        UTXO_ChainResponse {
            jsonrpc: String::new(),
            result: UTXO_Result {
                numFetched: String::new(),
                utxos: None,
                endIndex: UTXO_EndIndex {
                    address: String::new(),
                    utxo: String::new(),
                },
                encoding: String::new(),
            },
            id: 0,
        }
    }
}

//staked_chain_struct
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Staked_ChainResponse {
    pub jsonrpc: String,
    pub result: Staked_Result,
    pub id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Staked_Result {
    pub staked: String,
    pub stakeds: Option<Value>,
    pub stakedOutputs:Option<Vec<String>>,
    pub encoding: String, 
}

impl Default for Staked_ChainResponse {
    fn default() -> Self {
        Staked_ChainResponse {
            jsonrpc: String::new(),
            result: Staked_Result {
                staked: String::new(),
                stakeds: None,
                stakedOutputs: None,
                encoding: String::new(),
            },
            id: 0,
        }
    }
}
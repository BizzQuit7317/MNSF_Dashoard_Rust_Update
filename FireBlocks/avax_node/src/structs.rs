use reqwest::{Client, Error, Method};
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Sha256, Digest};
use bs58;

use crate::response_structs;

pub struct AvaxClient {
    pub ADDRESS_P:String,
    pub ADDRESS_HEX:String,
    pub BASE_URL:String,
    pub AVAX_URL:String,
    pub NETWORKS: Option<response_structs::NetworkListResponse>,
}

impl AvaxClient {
    pub fn new(address_p: String, address_hex: String) -> AvaxClient {
        
        AvaxClient {
            ADDRESS_P: address_p,
            ADDRESS_HEX: address_hex,
            BASE_URL: String::from("http://141.95.100.75:8081"),
            AVAX_URL:String::from("https://api.avax.network"), 
            NETWORKS: None,
        }
    }

    pub async fn get_networks(&mut self) {
        let networks = self.send_request::<response_structs::NetworkListResponse>("/network/list", Method::POST,Some(json!({"metadata": {}})), true).await;
        match networks {
            Ok(network_list) => {
                let _ = self.NETWORKS = Some(network_list);
            }
            Err(e) => {
                println!("[ERR] Getting networks: {}", e);
            }
        }; 
    }

    pub async fn send_request<T: for<'de> Deserialize<'de> + Default>(&self, endpoint: &str, method: Method, body_data: Option<Value>, address_at_end: bool) -> Result<T, Error> {
        //The base variable being set to true will use BASE_URL set to false will use AVAX_URL
        let mut full_url = String::new();
        match address_at_end {
            true => {
                full_url = format!("{}{}", self.BASE_URL, endpoint);
            },
            false => {
                full_url = format!("{}{}", self.AVAX_URL, endpoint);
            }
        }

        //let full_url = format!("{}{}", self.BASE_URL, endpoint);
        let client = Client::new();

        let request_builder = match method {
            Method::GET => client.get(&full_url),
            Method::POST => client.post(&full_url).json(&body_data.unwrap_or_else(|| json!({}))),
            Method::PUT => client.put(&full_url).json(&body_data.unwrap_or_else(|| json!({}))),
            Method::DELETE => client.delete(&full_url),
            _ => {
                return Ok(T::default())
            }
        };

        let response = request_builder.send().await?.json::<T>().await?;
        Ok(response)
    }

    pub fn add_checksum(&self, mut _buff: Vec<u8>) -> Vec<u8> {
        let mut hasher = Sha256::new();
        let _ = hasher.update(&_buff);
        let result = hasher.finalize();
        let checksum = &result[result.len() - 4..];
        let _ = _buff.extend_from_slice(checksum);
        _buff
    }

    pub fn _decode_utxo_hex(&self, hex_string: String) -> response_structs::UTXO {
        //All ranges comes directly from utxo avax documentation for readin the hex string
        let id: u32 = u32::from_str_radix(&hex_string[2..6], 16).unwrap(); //16 is the default to read from hex 2..6 is the id bytes range as told by documentation
        
        let bytes_tx_id = hex::decode(&hex_string[6..70]).expect("[ERR] Error decoding hex string to bytes!");
        let checksum_tx_id = self.add_checksum(bytes_tx_id.clone());
        let tx_id = bs58::encode(checksum_tx_id.clone()).into_string();

        let output_index: u32 = u32::from_str_radix(&hex_string[70..78], 16).unwrap(); 

        let bytes_asset_id = hex::decode(&hex_string[78..142]).expect("[ERR] Error decoding hex string to bytes!");
        let checksum_asset_id = self.add_checksum(bytes_asset_id.clone());
        let asset_id = bs58::encode(checksum_asset_id.clone()).into_string();

        let output_string = &hex_string[142..];

        let output_id: u32 = u32::from_str_radix(&output_string[..8], 16).unwrap();
        let amount: u32 = u32::from_str_radix(&output_string[8..24], 16).unwrap();
        let locktime: u32 = u32::from_str_radix(&output_string[24..40], 16).unwrap();
        let threshold: u32 = u32::from_str_radix(&output_string[40..48], 16).unwrap();
        let address_length: u32 = u32::from_str_radix(&output_string[48..56], 16).unwrap();
        let address = &output_string[56..96];     

        let res =  response_structs::UTXO {
            id:id,
            tx_id:tx_id,
            output_index:output_index,
            asset_id:asset_id,
            output_id:output_id,
            amount:amount,
            locktime:locktime,
            threshold:threshold,
            address_length:address_length,
            address:String::from(address),
        };
        res
        
    }

    pub async fn get_total_balance(&self) -> f64 {
        let c_chain = self.send_request::<response_structs::C_ChainResponse>("/account/balance", Method::POST, Some(json!({"network_identifier": self.NETWORKS.as_ref().unwrap().network_identifiers[0], "account_identifier": {"address":self.ADDRESS_HEX}, "include_mempool": false})), true).await;
        let c2p_chain = self.send_request::<response_structs::UTXO_ChainResponse>("/ext/bc/P", Method::POST, Some(json!({"jsonrpc": "2.0", "id": 1, "method": "platform.getUTXOs","params": {"addresses": [self.ADDRESS_P], "sourceChain": "C", "encoding": "hex"}})), false).await;
        let p_chain = self.send_request::<response_structs::UTXO_ChainResponse>("/ext/bc/P", Method::POST, Some(json!({"jsonrpc": "2.0", "id": 1, "method": "platform.getUTXOs","params": {"addresses": [self.ADDRESS_P], "sourceChain": "P", "encoding": "hex"}})), false).await;
        let p2c_chain = self.send_request::<response_structs::UTXO_ChainResponse>("/ext/bc/C/avax", Method::POST, Some(json!({"jsonrpc": "2.0", "id": 1, "method": "avax.getUTXOs","params": {"addresses": [self.ADDRESS_P.replace("P", "C")], "sourceChain": "C", "encoding": "hex"}})), false).await;
        let staked_chain = self.send_request::<response_structs::Staked_ChainResponse>("/ext/bc/P", Method::POST, Some(json!({"jsonrpc": "2.0", "id": 1, "method": "platform.getStake","params": {"addresses": [self.ADDRESS_P], "sourceChain": "C", "encoding": "hex"}})), false).await;

        let mut staked_chain_value: f64 = 0.0; 
        match staked_chain {
            Ok(staked_chain_response) => {
                match staked_chain_response.result.stakedOutputs {
                    Some(stakedOutputs) if !stakedOutputs.is_empty() => {
                        let stakedOutputs = &stakedOutputs[0]; // safe now
                        let stakedOutputs_obj = self._decode_utxo_hex(stakedOutputs.to_string());
                        staked_chain_value = stakedOutputs_obj.amount as f64;
                    }
                    _ => println!("[WRN] No staked output found on staked"),
                }
            }
            Err(e) => println!("[ERR] Getting staked chain: {:?}", e)
        }

        let mut p2c_chain_value: f64 = 0.0; 
        match p2c_chain {
            Ok(p2c_chain_response) => {
                match p2c_chain_response.result.utxos {
                    Some(utox) if !utox.is_empty() => {
                        let utxo = &utox[0]; // safe now
                        let utxo_obj = self._decode_utxo_hex(utxo.to_string());
                        p2c_chain_value = utxo_obj.amount as f64;
                    }
                    _ => println!("[WRN] No utxo found on c2p"),
                }
            }
            Err(e) => println!("[ERR] Getting p2c chain: {:?}", e)
        }

        let mut p_chain_value: f64 = 0.0; 
        match p_chain {
            Ok(p_chain_response) => {
                match p_chain_response.result.utxos {
                    Some(utox) if !utox.is_empty() => {
                        let utxo = &utox[0]; // safe now
                        let utxo_obj = self._decode_utxo_hex(utxo.to_string());
                        p_chain_value = utxo_obj.amount as f64;
                    }
                    _ => println!("[WRN] No utxo found on c2p"),
                }
            }
            Err(e) => println!("[ERR] Getting p chain: {:?}", e)
        }

        let mut c2p_chain_value: f64 = 0.0; 
        match c2p_chain {
            Ok(c2p_chain_response) => {
                match c2p_chain_response.result.utxos {
                    Some(utox) if !utox.is_empty() => {
                        let utxo = &utox[0]; // safe now
                        let utxo_obj = self._decode_utxo_hex(utxo.to_string());
                        c2p_chain_value = utxo_obj.amount as f64;
                    }
                    _ => println!("[WRN] No utxo found on c2p"),
                }
            }
            Err(e) => println!("[ERR] Getting c2p chain: {:?}", e)
        }

        let mut c_chain_value: f64 = 0.0;
        match c_chain {
            Ok(c_chain_response) => {
                match c_chain_response.balances.get(0) {
                    Some(balance) => {
                        c_chain_value = balance.value.parse().unwrap_or(0.0);
                    }
                    None => println!("[WRN] No balances found")
                }
            }
            Err(e) => println!("[ERR] Getting C chain: {:?}", e)
        }

        let total_balance = c_chain_value + c2p_chain_value + p_chain_value + p2c_chain_value + staked_chain_value;

        println!("c value -> {:?}\nc2p value -> {:?}\np value -> {:?}\np2c value -> {:?}\nstaked value -> {:?}", c_chain_value, c2p_chain_value, p_chain_value, p2c_chain_value, staked_chain_value);
        total_balance
    }


}


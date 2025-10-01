use reqwest::{Client, Error, Method};
use serde::Deserialize;
use serde_json::{json, Value};
use base64;

use crate::response_structs;

pub struct NearClient {
    pub ADDRESS: String,
    pub VALIDATOR_ADDDRESS: String,
    pub BASE_URL: String,
    pub NETWORKS: Option<response_structs::NetworkListResponse>,
}

impl NearClient {
    pub fn new (address: String) -> NearClient {
        NearClient {
            ADDRESS: address,
            VALIDATOR_ADDDRESS: String::from("colossus.poolv1.near"),
            BASE_URL: String::from("http://rosetta-near-mainnet.colossus.life:3040"),
            NETWORKS: None,
        }
    }

    pub async fn send_request<T: for<'de> Deserialize<'de> + Default>(&self, endpoint: &str, method: Method, body_data: Option<Value>, address_at_end: bool) -> Result<T, Error> {
        //The base variable being set to true will use BASE_URL set to false will use AVAX_URL
        let mut full_url = String::new();
        match address_at_end {
            true => {
                full_url = format!("{}{}{}", self.BASE_URL, endpoint, self.ADDRESS);
            },
            false => {
                full_url = format!("{}{}", self.BASE_URL, endpoint);
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

    pub async fn get_networks(&mut self) {
        let networks = self.send_request::<response_structs::NetworkListResponse>("/network/list", Method::POST,Some(json!({"metadata": {}})), false).await;
        match networks {
            Ok(network_list) => {
                let _ = self.NETWORKS = Some(network_list);
            }
            Err(e) => {
                println!("[ERR] Getting networks: {}", e);
            }
        }; 
    }

    pub async fn get_total_balance(&mut self) -> f64 {
        let mut total_balance: f64 = 0.0;

        let available = self.send_request::<response_structs::AvailableResponse>("/account/balance", Method::POST,Some(json!({"network_identifier": self.NETWORKS.as_ref().unwrap().network_identifiers[0], "account_identifier": {"address":self.ADDRESS}})), false).await;
        match available {
            Ok(available_response) => {
                for balance in available_response.balances {
                    total_balance += balance.value.parse::<u128>().unwrap() as f64
                }
            },
            Err(e) => println!("[ERR] Getting available: {:?}", e)
        }

        self.BASE_URL = String::from("https://rpc.mainnet.near.org");
        
        let s = format!("{{\"account_id\": \"{}\"}}", self.ADDRESS);
        let encoded_args = base64::encode(s.to_string().into_bytes());

        let staked = self.send_request::<response_structs::StakedResponse>("", Method::POST,Some(json!({"jsonrpc": "2.0", "id": "", "method": "query", "params": {"request_type": "call_function", "finality": "final", "account_id": self.VALIDATOR_ADDDRESS, "method_name": "get_account_staked_balance", "args_base64": encoded_args}})), false).await;
        match staked {
            Ok(staked_response) => {
                let decoded_bytes = staked_response.result.result;
                let decoded_str = String::from_utf8(decoded_bytes).expect("[WRN] invalid UTF-8");

                let trimmed = decoded_str.trim_matches('"');

                let staked_amount: u128 = trimmed.parse().expect("invalid number");

                total_balance += staked_amount as f64;
            },
            Err(e) => println!("[ERR] Getting staked: {:?}", e)
        }

        let unstaked = self.send_request::<response_structs::StakedResponse>("", Method::POST,Some(json!({"jsonrpc": "2.0", "id": "", "method": "query", "params": {"request_type": "call_function", "finality": "final", "account_id": self.VALIDATOR_ADDDRESS, "method_name": "get_account_unstaked_balance", "args_base64": encoded_args}})), false).await;
        match unstaked {
            Ok(unstaked_response) => {
                let decoded_bytes = unstaked_response.result.result;
                let decoded_str = String::from_utf8(decoded_bytes).expect("[WRN] invalid UTF-8");

                let trimmed = decoded_str.trim_matches('"');

                let unstaked_amount: u128 = trimmed.parse().expect("invalid number");

                total_balance += unstaked_amount as f64;
            },
            Err(e) => println!("[ERR] Getting unstaked: {:?}", e)
        }

        total_balance
    }
}

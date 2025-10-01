use serde::Deserialize;
use reqwest::{Client, Error, Method};
use serde_json::{json, Value};
use crate::response_structs;

pub struct EthClient {
    pub ADDRESS: String,
    pub BASE_URL: String,
}

impl EthClient {
    pub fn new(address: String) -> EthClient {
        EthClient {
            ADDRESS: address,
            BASE_URL: String::from("https://beaconcha.in/api/v1/"),
        }
    }

    pub async fn send_request<T: for<'de> Deserialize<'de> + Default>(&mut self, endpoint: &str, method: Method, body_data: Option<Value>, address_at_end: bool) -> Result<T, Error> {
        let mut url = String::new();
        if address_at_end {
            url = format!("{}{}{}", self.BASE_URL, endpoint, self.ADDRESS);
        } else {
            url = format!("{}{}", self.BASE_URL, endpoint);
        }
        let client = Client::new();

        let request_builder = match method {
            Method::GET => client.get(&url),
            Method::POST => client.post(&url).json(&body_data.unwrap_or_else(|| json!({}))),
            Method::PUT => client.put(&url).json(&body_data.unwrap_or_else(|| json!({}))),
            Method::DELETE => client.delete(&url),
            _ => {
                return Ok(T::default())
            }
        };

        let response = request_builder.send().await?.json::<T>().await?;

        Ok(response) 
    }

    pub async fn get_total_balance(&mut self) -> f64 {
        let mut total_balance: f64 = 0.0;
        let validators = &self.send_request::<response_structs::ValidatorResponse>("validator/eth1/", Method::GET, None, true).await;
        match validators {
            Ok(validator_data) => {
                match &validator_data.data {
                    Some(data_outputs) if !data_outputs.is_empty() => {
                        for output in data_outputs {
                            let key_delegation_endpoint = format!("validator/{}/deposits", output.publickey);
                            let key_delegation = &self.send_request::<response_structs::DelegationResponse>(&key_delegation_endpoint, Method::GET, None, false).await;
                            match key_delegation {
                                Ok(delegation) => {
                                    match &delegation.data {
                                        Some(result_output) if !data_outputs.is_empty() => {
                                            for result in result_output {
                                                total_balance += result.amount as f64;
                                            }
                                        }
                                        _ => println!("[WRN] No delegator outputs"),
                                    }
                                },
                                Err(e) => println!("[ERR] No delegations {}", e),
                            }
                        }
                    }
                    _ => println!("[WRN] No validator outputs"),
                }
            },
            Err(e) => println!("[ERR] No validators {}", e),
        }
        total_balance
    }
}
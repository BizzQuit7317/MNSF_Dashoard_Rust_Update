use reqwest::{Client, Error, Method, header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT}};
use std::time::{SystemTime, UNIX_EPOCH};
use hmac::{Hmac, Mac};
use sha2::{Sha256, Digest};
use hex::encode;
use serde_json::{json, Value};
use url::form_urlencoded;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono;
use mongodb::{bson::{doc, Document, DateTime}, Database};

#[derive(Debug, Serialize)]
pub struct Snapshot {
    pub fetched_at: DateTime,
    pub account: String,
    pub wallet: String,
    pub data: Value,
}

pub struct BinanceClient {
    pub API_KEY:String,
    pub API_SECRET:String,
    pub BASE_URL:String
}

impl BinanceClient {
    pub fn new(api_key: String, api_secret: String) -> BinanceClient {
        BinanceClient {
            API_KEY:api_key,
            API_SECRET:api_secret,
            BASE_URL:String::from("https://api.binance.com")
        }
    }

    pub fn get_signature(&self, query_string: &str) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(self.API_SECRET.as_bytes()).expect("Error creating hamc!");
        let _ = mac.update(query_string.as_bytes());
        let result = mac.finalize();
        encode(result.into_bytes())
    }

    pub async fn send_request<T: for<'de> Deserialize<'de> + Default>(&mut self, base_endpoint: &str, endpoint: &str, method: Method, body_data: Option<Value>) -> Result<T, Error> {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let timestamp = since_epoch.as_millis();
        let TimeStamp = timestamp as u64;

        self.BASE_URL = format!("https://{}.binance.com", base_endpoint);

        let params: HashMap<String, String> = match &body_data {
            Some(data) => {
                serde_json::from_value(data.clone()).unwrap_or_default()
            }
            None => HashMap::new(),
        };

        let mut query_string = form_urlencoded::Serializer::new(String::new()).extend_pairs(params.iter()).finish();
        query_string = format!("{}&timestamp={}", query_string, TimeStamp);
        let signature = self.get_signature(&query_string);
        let url = format!("{}{}?{}&signature={}", self.BASE_URL, endpoint, query_string, signature);

        let mut headers = HeaderMap::new();
        let _ = headers.insert("X-MBX-APIKEY", HeaderValue::from_str(&self.API_KEY).unwrap());

        let client = Client::new();

        let request_builder = match method {
            Method::GET => client.get(&url).headers(headers),
            Method::POST => client.post(&url).headers(headers).json(&body_data.unwrap_or_else(|| json!({}))),
            Method::PUT => client.put(&url).headers(headers).json(&body_data.unwrap_or_else(|| json!({}))),
            Method::DELETE => client.delete(&url).headers(headers),
            _ => {
                return Ok(T::default())
            }
        };

        let response = request_builder.send().await?.json::<T>().await?;

        Ok(response)

    }

    pub async fn push_to_db(&self, account: String, wallet: String, data: Value, database: &str, collection: &str) {
        let snapshot = Snapshot {
            fetched_at: DateTime::now(),
            account: account,
            wallet: wallet,
            data: data,
        };
        //println!("{:?}", snapshot);
        let client = mongodb::Client::with_uri_str("mongodb://localhost:27017".to_string()).await.unwrap();
        let db = client.database(database);
        let test_collection: mongodb::Collection<Document> = db.collection(collection);

        //let test_doc = doc! { "test": "Success" };
        let doc = mongodb::bson::to_document(&snapshot).unwrap();
        let insert_result = test_collection.insert_one(doc).await.unwrap();
        //println!("Complete!")
    }

    pub async fn get_data(&mut self, given_account: String) {
        //Collect data
        let future_wallet = self.send_request::<Value>("fapi", "/fapi/v2/balance", Method::GET, None).await;
        let m_wallet = self.send_request::<Value>("dapi", "/dapi/v1/balance", Method::GET, None).await;
        let spot_wallet = self.send_request::<Value>("api", "/sapi/v1/capital/config/getall", Method::GET, Some(json!({"type": "SPOT"}))).await;
        let margin_wallet = self.send_request::<Value>("api", "/sapi/v1/margin/account", Method::GET, None).await;
        let isolated_margin_wallet = self.send_request::<Value>("api", "/sapi/v1/margin/isolated/account", Method::GET, None).await;
        let earn_statking_wallet = self.send_request::<Value>("api", "/sapi/v1/staking/position", Method::GET, None).await;
        let earn_locked_wallet = self.send_request::<Value>("api", "/sapi/v1/simple-earn/flexible/position", Method::GET, None).await;
        let maint_margin = self.send_request::<Value>("fapi", "/fapi/v2/account", Method::GET, None).await;
        let futures_positions = self.send_request::<Value>("fapi", "/fapi/v2/positionRisk", Method::GET, None).await;
        let m_positions = self.send_request::<Value>("dapi", "/dapi/v1/positionRisk", Method::GET, None).await;

        //Process data
        let db = "BINANCE_RAW_DATA";
        let account = given_account;


        if let Ok(wallet) = future_wallet {
            self.push_to_db(account.clone(), "Future".to_string(), wallet, db, "FUTURE_WALLET").await;
        }
        if let Ok(wallet) = m_wallet {
            self.push_to_db(account.clone(), "M".to_string(), wallet, db, "M_WALLET").await;
        }
        if let Ok(wallet) = spot_wallet {
            self.push_to_db(account.clone(), "Spot".to_string(), wallet, db, "SPOT_WALLET").await;
        }
        if let Ok(wallet) = margin_wallet {
            self.push_to_db(account.clone(), "Margin".to_string(), wallet, db, "MARGIN_WALLET").await;
        }
        if let Ok(wallet) = isolated_margin_wallet {
            self.push_to_db(account.clone(), "Isolated Margin".to_string(), wallet, db, "ISOLATED_MARGIN_WALLET").await;
        }
        if let Ok(wallet) = earn_statking_wallet {
            self.push_to_db(account.clone(), "Staking".to_string(), wallet, db, "STAKING_WALLET").await;
        }
        if let Ok(wallet) = earn_locked_wallet {
            self.push_to_db(account.clone(), "Locked".to_string(), wallet, db, "LOCKED_WALLET").await;
        }
        if let Ok(wallet) = maint_margin {
            self.push_to_db(account.clone(), "Maint Margin".to_string(), wallet, db, "MAINT_MMARGIN").await;
        }
        if let Ok(wallet) = futures_positions {
            self.push_to_db(account.clone(), "Future Positions".to_string(), wallet, db, "FUTURE_POSITIONS").await;
        }
        if let Ok(wallet) = m_positions {
            self.push_to_db(account.clone(), "M Positions".to_string(), wallet, db, "M_POSITIONS").await;
        }
    }

}

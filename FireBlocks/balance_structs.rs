use std::time::{SystemTime, UNIX_EPOCH};
use chrono::DateTime;
use serde::{Deserialize, Serialize};

pub async fn get_price(asset: &str) -> f64 {
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}USDT", asset);
    let resp = reqwest::get(url).await.unwrap();
    let json: serde_json::Value = resp.json().await.unwrap();

    json["price"]
        .as_str()
        .unwrap()
        .parse::<f64>()
        .unwrap()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullBalanceSnapshot {
    pub snapshots: Vec<BalanceSnapshot>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceSnapshot {
    chain: String,
    asset: String,
    raw_coin_amount: String, //Numbers will be to big to store as int/float types
    decimals: u8,
    normalized: f64,
    usd_value: f64,
    price: f64,
    epoch_timestamp: u64,
    timestamp: String,
}

impl BalanceSnapshot {
    pub async fn new(chain: String, asset: String, raw_coin_amount: String, decimals: u8) -> BalanceSnapshot {
        let epoch_timestamp: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let timestamp = DateTime::from_timestamp(epoch_timestamp as i64, 0)
            .unwrap()
            .format("%Y-%m-%d %H:%M UTC")
            .to_string();
        let normalized = raw_coin_amount.parse::<f64>().unwrap() / 10f64.powi(decimals as i32);
        let price = get_price(asset.as_str()).await;
        let usd_value = normalized * price;

        let snapshot = BalanceSnapshot {
            chain,
            asset,
            raw_coin_amount,
            decimals,
            normalized,
            usd_value,
            price,
            epoch_timestamp,
            timestamp,
        };
        snapshot
    }
}

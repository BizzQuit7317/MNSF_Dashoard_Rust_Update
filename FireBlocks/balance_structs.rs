use std::time::{SystemTime, UNIX_EPOCH};
use chrono::DateTime;

pub struct FullBalanceSnapshot {
    snapshots: Vec<BalanceSnapshot>
}

pub struct BalanceSnapshot {
    chain: String,
    asset: String,
    raw_coin_amount: String, //Numbers will be to big to store as int/float types
    decimals: u8,
    normalized: f64,
    epoch_timestamp: u64,
    timestamp: String,
}

impl BalanceSnapshot {
    pub fn new(chain: String, asset: String, raw_coin_amount: String, decimals: u8) -> BalanceSnapshot {
        let epoch_timestamp: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let timestamp = DateTime::from_timestamp(epoch_timestamp as i64, 0)
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string();
        let normalized = raw_coin_amount.parse::<f64>().unwrap() / 10f64.powi(decimals as i32);

        let snapshot = BalanceSnapshot {
            chain,
            asset,
            raw_coin_amount,
            decimals,
            normalized,
            epoch_timestamp,
            timestamp,
        };
        snapshot
    }
}

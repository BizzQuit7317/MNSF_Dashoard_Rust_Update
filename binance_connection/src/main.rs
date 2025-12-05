mod structs;

use mongo_connect::ClientStruct;
use mongodb::bson::doc;
use serde_json::json;
use structs::BinanceClient;
use reqwest::Method;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let key_data = run_client().await;

    let mut Binance_Client = structs::BinanceClient::new(key_data.key, key_data.secret);
    let mut MongoClient = ClientStruct::new("").await;

    /*
        Collection all raw binance calls we will need
        let _wallet = Binance_Client.polling("api", "", Method::GET, None).await;
    */
    let future_wallet = Binance_Client.polling("fapi", "/fapi/v2/balance", Method::GET, None, &mut  MongoClient, "future_wallet").await;
    let m_wallet = Binance_Client.polling("dapi", "/dapi/v1/balance", Method::GET, None, &mut  MongoClient, "m_wallet").await;
    let spot_wallet = Binance_Client.polling("api", "/sapi/v1/capital/config/getall", Method::GET, Some(json!({"type": "SPOT"})), &mut  MongoClient, "spot_wallet").await;
    let margin_wallet = Binance_Client.polling("api", "/sapi/v1/margin/account", Method::GET, None, &mut  MongoClient, "margin_wallet").await;
    let isolated_margin_wallet = Binance_Client.polling("api", "/sapi/v1/margin/isolated/account", Method::GET, None, &mut  MongoClient, "isolated_margin_wallet").await;
    let earn_statking_wallet = Binance_Client.polling("api", "/sapi/v1/staking/position", Method::GET, None, &mut  MongoClient, "earn_staking_wallet").await;
    let earn_locked_wallet = Binance_Client.polling("api", "/sapi/v1/simple-earn/flexible/position", Method::GET, None, &mut  MongoClient, "earn_locked_wallet").await;
    let maint_margin = Binance_Client.polling("fapi", "/fapi/v2/account", Method::GET, None, &mut  MongoClient, "maint_margin").await;
    let futures_positions = Binance_Client.polling("fapi", "/fapi/v2/positionRisk", Method::GET, None, &mut  MongoClient, "future_positions").await;
    let m_positions = Binance_Client.polling("dapi", "/dapi/v1/positionRisk", Method::GET, None, &mut  MongoClient, "m_positions").await;

    /*
    let mut MongoClient = ClientStruct::new("Raw_Exchange_Data").await;
    let _ = MongoClient.select_collection(String::from("test")).await;
    let _ = MongoClient.push_document_collection(doc! {"Date":"2025-07-28", "Time":"14-02"}).await;
    */
}

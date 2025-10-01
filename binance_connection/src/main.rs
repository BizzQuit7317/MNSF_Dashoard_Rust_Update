mod structs;

use mongo_connect::ClientStruct;
use mongodb::bson::doc;
use serde_json::json;
use structs::BinanceClient;
use reqwest::Method;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let mut Binance_Client = structs::BinanceClient::new(String::from(""), String::from(""));
    let response = Binance_Client.send_request::<Value>("fapi", "/fapi/v2/account", Method::GET, None).await;

    println!("{:?}", response);

    /*
    let mut MongoClient = ClientStruct::new("Raw_Exchange_Data").await;
    let _ = MongoClient.select_collection(String::from("test")).await;
    let _ = MongoClient.push_document_collection(doc! {"Date":"2025-07-28", "Time":"14-02"}).await;
    */
}

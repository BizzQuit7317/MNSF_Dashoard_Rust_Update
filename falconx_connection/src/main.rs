mod structs;

use mongo_connect::ClientStruct;
use mongodb::bson::doc;
use reqwest::Method;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let FalconX_Client = structs::FalconxClient::new(String::from(""), String::from(""), String::from(""));
    let response = FalconX_Client.send_request::<Value>("/v1/api/native-custody/accounts", Method::GET, None).await;

    println!("Response -> {:?}", response);

    /*
    let mut MongoClient = ClientStruct::new("Raw_Exchange_Data").await;
    let _ = MongoClient.select_collection(String::from("test")).await;
    let _ = MongoClient.push_document_collection(doc! {"Date":"2025-07-28", "Time":"14-06"}).await;
    */
}

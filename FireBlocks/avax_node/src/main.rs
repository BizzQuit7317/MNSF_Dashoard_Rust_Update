mod structs;
mod response_structs;

use mongo_connect::ClientStruct;
use mongodb::bson::doc;
use reqwest::Method;

#[tokio::main]
async fn main() {
    let mut Avax_Client = structs::AvaxClient::new(String::from(""), String::from(""));
    let _ = Avax_Client.get_networks().await;
    let avax_balance = Avax_Client.get_total_balance().await;

    println!("avax balance -> {:?}", avax_balance);

    /*
    let mut MongoClient = ClientStruct::new("Raw_Exchange_Data").await;
    let _ = MongoClient.select_collection(String::from("test")).await;
    let _ = MongoClient.push_document_collection(doc! {"Date":"2025-07-29", "Exchange":"avax"}).await;
    */
}
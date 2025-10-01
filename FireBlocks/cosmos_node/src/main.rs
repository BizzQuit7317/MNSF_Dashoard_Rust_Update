mod structs;
mod response_structs;

use mongo_connect::ClientStruct;
use mongodb::bson::doc;
use reqwest::Method;

#[tokio::main]
async fn main() {
    let mut Cosmos_Client = structs::CosmosClient::new(String::from(""), None); //Atom Staking
    let CosmosBalance = Cosmos_Client.get_total_balance().await;

    println!("total balance atom staked-> {:?}", CosmosBalance);

    let mut Cosmos_Client = structs::CosmosClient::new(String::from(""), None); //Atom Cold
    let CosmosBalance = Cosmos_Client.get_total_balance().await;

    println!("total balance atom cold-> {:?}", CosmosBalance);

    let mut Cosmos_Client = structs::CosmosClient::new(String::from(""), None); //SCRT
    let CosmosBalance = Cosmos_Client.get_total_balance().await;

    println!("total balance scrt-> {:?}", CosmosBalance);

    let mut Cosmos_Client = structs::CosmosClient::new(String::from(""), None); //SCRT
    let CosmosBalance = Cosmos_Client.get_total_balance().await;

    println!("total balance kava-> {:?}", CosmosBalance);

    /*
    let mut MongoClient = ClientStruct::new("Raw_Exchange_Data").await;
    let _ = MongoClient.select_collection(String::from("test")).await;
    let _ = MongoClient.push_document_collection(doc! {"Date":"2025-08-07", "Exchange":"cosmos"}).await;
    */
}
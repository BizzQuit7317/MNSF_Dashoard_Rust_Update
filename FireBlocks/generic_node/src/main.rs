mod structs;
mod response_structs;

use mongo_connect::ClientStruct;
use mongodb::bson::doc;
use reqwest::Method;
use serde_json::Value;

#[tokio::main]
async fn main() {
    /* Oasis Client
    let Oasis_Client = structs::GenericClient::new(String::from(""), String::from("http://api.oasisscan.com/mainnet"));
    let response = Oasis_Client.send_request::<Value>("/chain/account/info/", Method::GET, None, true, 5).await;
    */
    
    /*
    let BTC_Client = structs::GenericClient::new(String::from(""), String::from("https://mempool.space/api"));
    let response = BTC_Client.send_request::<response_structs::BTC_Response>("/address/", Method::GET, None, true, 100).await;
    */

    /*
    let FLOW_Client = structs::GenericClient::new(String::from(""), String::from("https://rest-mainnet.onflow.org"));
    let response = FLOW_Client.send_request::<response_structs::Flow_Response>("/v1/accounts/", Method::GET, None, true, 100).await;
    */

    let Tezos_client = structs::GenericClient::new(String::from(""), String::from("https://api.tzkt.io"));
    let Tezos_Key = ""; //In future this wont be plain text but a proper encrypted key variabled
    let Tezos_Endpoint = format!("/v1/accounts/{}/balance", Tezos_Key);
    let response = Tezos_client.send_request::<Value>(&Tezos_Endpoint, Method::GET, None, false, 100).await;
    //Doesnt need a struct as it just returns a single number that can be used as normal

    println!("{:?}", response);

    /*
    let mut MongoClient = ClientStruct::new("Raw_Exchange_Data").await;
    let _ = MongoClient.select_collection(String::from("test")).await;
    let _ = MongoClient.push_document_collection(doc! {"Date":"2025-07-29", "exchange":"btc"}).await;
    */
}

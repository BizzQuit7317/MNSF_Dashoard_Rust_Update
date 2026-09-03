mod structs;

use reqwest::Method;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let mut Bybit_Client = structs::BybitClient::new(String::from(""), String::from("")); //main
    Bybit_Client.get_data().await;

    let mut Bybit_Client_sub1 = structs::BybitClient::new(String::from(""), String::from("")); //sub1
    Bybit_Client_sub1.get_data().await;

}

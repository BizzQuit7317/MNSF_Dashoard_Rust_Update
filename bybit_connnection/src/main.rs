mod structs;

use reqwest::Method;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let mut Bybit_Client = structs::BybitClient::new(String::from("SdTNT25LySe50uxH3J"), String::from("nXzOigC0YKDYxRc5i0savmM3VYznWlUF8KPo")); //main
    Bybit_Client.get_data().await;

    let mut Bybit_Client_sub1 = structs::BybitClient::new(String::from("kOyYYhccYGkeugZlw3"), String::from("NbRbjtwaJY0dAq5VEyoINYBvhORiTHRPovjB")); //sub1
    Bybit_Client_sub1.get_data().await;

}

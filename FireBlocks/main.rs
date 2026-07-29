mod block_structs;
mod balance_struct;

use reqwest::Method;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let mut avax_client = block_structs::avax::structs::AvaxClient::new(String::from(""), String::from(""));
    avax_client.get_networks().await;
    let avax_response = avax_client.get_total_balance().await;

    println!("avax balance -> {:?}", avax_response);
    let avax_snapshot = balance_struct::BalanceSnapshot::new()

    let cartesi_client = block_structs::cartesi::structs::CartesiClient::new(String::from(""), String::from(""));
    let cartesi_response = cartesi_client.send_request::<Value>("endpoint", Method::GET, None).await;

    println!("cartesi balance -> {:?}", cartesi_response.map_or(0.0, |r| r.total_balance));

    let mut cosmos_client = block_structs::cosmos::structs::CosmosClient::new(String::from(""), None); //Atom Staking
    let CosmosBalance = cosmos_client.get_total_balance().await;

    println!("atom balance staked -> {:?}", CosmosBalance);

    let mut cosmos_client =  block_structs::cosmos::structs::CosmosClient::new(String::from(""), None); //Atom Cold
    let CosmosBalance = cosmos_client.get_total_balance().await;

    println!("atom balance cold -> {:?}", CosmosBalance);

    let mut cosmos_client =  block_structs::cosmos::structs::CosmosClient::new(String::from(""), None); //SCRT
    let CosmosBalance = cosmos_client.get_total_balance().await;

    println!("scrt balance -> {:?}", CosmosBalance);

    let mut cosmos_client =  block_structs::cosmos::structs::CosmosClient::new(String::from(""), None); //SCRT
    let CosmosBalance = cosmos_client.get_total_balance().await;

    println!("kava balance -> {:?}", CosmosBalance);

    let mut eth_client = block_structs::eth::structs::EthClient::new(String::from(""));
    let eth_balance = eth_client.get_total_balance().await;

    println!("eth balance -> {:?}", eth_balance);

    let oasis_client = block_structs::generic::structs::GenericClient::new(String::from(""), String::from("http://api.oasisscan.com/mainnet"));
    let oasis_response = oasis_client.send_request::<Value>("/chain/account/info/", Method::GET, None, true, 5).await;

    println!("oasis balance -> {:?}", oasis_response);

    let btc_client = block_structs::generic::structs::GenericClient::new(String::from(""), String::from("https://mempool.space/api"));
    let btc_response = btc_client.send_request::<block_structs::generic::response_structs::BTC_Response>("/address/", Method::GET, None, true, 100).await;

    println!("btc balance -> {:?}", btc_response);

    let flow_client = block_structs::generic::structs::GenericClient::new(String::from(""), String::from("https://rest-mainnet.onflow.org"));
    let flow_response = flow_client.send_request::<block_structs::generic::response_structs::Flow_Response>("/v1/accounts/", Method::GET, None, true, 100).await;

    println!("flow balance -> {:?}", flow_response);

    let tezos_client = block_structs::generic::structs::GenericClient::new(String::from(""), String::from("https://api.tzkt.io"));
    let tezos_Key = ""; //In future this wont be plain text but a proper encrypted key variabled
    let tezos_endpoint = format!("/v1/accounts/{}/balance", tezos_Key);
    let tezos_response = tezos_client.send_request::<Value>(&tezos_endpoint, Method::GET, None, false, 100).await;

    println!("tezos balance -> {:?}", tezos_response);

    let harmoney_client = block_structs::harmony::structs::HarmonyClient::new(String::from(""));
    let harmoney_response = harmoney_client.get_total_balance().await;

    println!("harmony balance: {}", harmoney_response);

    let lpt_client = block_structs::lpt::structs::LPTClient::new(String::from(""));
    let lpt_response = lpt_client.send_request::<Value>("", Method::GET, None).await;

    println!("lpt balance -> {}", lpt_response);

    let mut near_client = block_structs::near::structs::NearClient::new(String::from(""));
    let _ = near_client.get_networks().await;
    let near_response = near_client.get_total_balance().await;

    println!("near balance -> {:?}", near_response);

    let polkadot_keys = ["", "", "", "", "", ""];
    let mut counter = 1;

    for key in polkadot_keys {
        let polkadot_client = block_structs::polkadot::structs::PolkadotClient::new(String::from(key));
        let polkadot_response = polkadot_client.get_total_balance().await;

        println!("{} polkadot balance -> {}", counter, polkadot_response);
        counter += 1;
    }
}

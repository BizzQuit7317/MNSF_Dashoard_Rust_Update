mod block_structs;
mod balance_struct;

use reqwest::Method;
use serde_json::Value;
use mongodb::{bson::{doc, Document}, Client, Database};

#[tokio::main]
async fn main() {
    let mut all_assets = balance_struct::FullBalanceSnapshot {
        snapshots: vec![],
    };

    let mut avax_client = block_structs::avax::structs::AvaxClient::new(String::from(""), String::from(""));
    avax_client.get_networks().await;
    let avax_response = avax_client.get_total_balance().await;
    //println!("avax balance -> {:?}", avax_response);
    let avax_snapshot = balance_struct::BalanceSnapshot::new("Avax".to_string(), "AVAX".to_string(), avax_response.to_string(), 9); //9 from documentatino on decimal places
    //println!("{:?}", avax_snapshot);
    all_assets.snapshots.push(avax_snapshot);

    let cartesi_client = block_structs::cartesi::structs::CartesiClient::new(String::from(""), String::from(""));
    let cartesi_response = cartesi_client.send_request::<Value>("endpoint", Method::GET, None).await;
    //println!("cartesi balance -> {:?}", cartesi_response.map_or(0.0, |r| r.total_balance));
    let cartesi_snapshot = balance_struct::BalanceSnapshot::new("Cartesi".to_string(), "CTSI".to_string(), cartesi_response.unwrap().balance.to_string(), 18); //9 from documentatino on decimal places
    //println!("{:?}", cartesi_snapshot);
    all_assets.snapshots.push(cartesi_snapshot);

    let mut cosmos_client = block_structs::cosmos::structs::CosmosClient::new(String::from(""), None); //Atom Staking
    let cosmos_response = cosmos_client.get_total_balance().await;
    //println!("atom balance staked -> {:?}", cosmos_response);
    let cosmos_snapshot = balance_struct::BalanceSnapshot::new("Cosmos".to_string(), "ATOM".to_string(), cosmos_response.to_string(), 6); //9 from documentatino on decimal places
    //println!("{:?}", cosmos_snapshot);
    all_assets.snapshots.push(cosmos_snapshot);

    let mut cosmos_client =  block_structs::cosmos::structs::CosmosClient::new(String::from(""), None); //Atom Cold
    let cosmos_response = cosmos_client.get_total_balance().await;
    //println!("atom balance cold -> {:?}", cosmos_response);
    let cosmos_snapshot = balance_struct::BalanceSnapshot::new("Cosmos".to_string(), "ATOM".to_string(), cosmos_response.to_string(), 6); //9 from documentatino on decimal places
    //println!("{:?}", cosmos_snapshot);
    all_assets.snapshots.push(cosmos_snapshot);

    let mut cosmos_client =  block_structs::cosmos::structs::CosmosClient::new(String::from(""), None); //SCRT
    let cosmos_response = cosmos_client.get_total_balance().await;
    //println!("scrt balance -> {:?}", cosmos_response);
    let cosmos_snapshot = balance_struct::BalanceSnapshot::new("Cosmos".to_string(), "SCRT".to_string(), cosmos_response.to_string(), 6); //9 from documentatino on decimal places
    //println!("{:?}", cosmos_snapshot);
    all_assets.snapshots.push(cosmos_snapshot);

    let mut cosmos_client =  block_structs::cosmos::structs::CosmosClient::new(String::from(""), None); //SCRT
    let cosmos_response = cosmos_client.get_total_balance().await;
    //println!("kava balance -> {:?}", cosmos_response);
    let cosmos_snapshot = balance_struct::BalanceSnapshot::new("Cosmos".to_string(), "KAVA".to_string(), cosmos_response.to_string(), 6); //9 from documentatino on decimal places
    //println!("{:?}", cosmos_snapshot);
    all_assets.snapshots.push(cosmos_snapshot);

    let mut eth_client = block_structs::eth::structs::EthClient::new(String::from(""));
    let eth_response = eth_client.get_total_balance().await;
    //println!("eth balance -> {:?}", eth_balance);
    let eth_snapshot = balance_struct::BalanceSnapshot::new("Eth".to_string(), "ETH".to_string(), eth_response.to_string(), 18); //9 from documentatino on decimal places
    //println!("{:?}", eth_snapshot);
    all_assets.snapshots.push(eth_snapshot);

    let btc_client = block_structs::generic::structs::GenericClient::new(String::from(""), String::from("https://mempool.space/api"));
    let btc_response = btc_client.send_request::<block_structs::generic::response_structs::BTC_Response>("/address/", Method::GET, None, true, 100).await;
    //println!("btc balance -> {:?}\n{:?}{:?}", btc_response.as_ref().unwrap().chain_stats, btc_response.as_ref().unwrap().chain_stats.funded_txo_sum, btc_response.as_ref().unwrap().chain_stats.spent_txo_sum);
    let btc_value = btc_response.as_ref().unwrap().chain_stats.funded_txo_sum - btc_response.as_ref().unwrap().chain_stats.spent_txo_sum;
    let btc_snapshot = balance_struct::BalanceSnapshot::new("Generic".to_string(), "BTC".to_string(), btc_value.to_string(), 8); //9 from documentatino on decimal places
    //println!("{:?}", btc_snapshot);
    all_assets.snapshots.push(btc_snapshot);

    let flow_client = block_structs::generic::structs::GenericClient::new(String::from(""), String::from("https://rest-mainnet.onflow.org"));
    let flow_response = flow_client.send_request::<block_structs::generic::response_structs::Flow_Response>("/v1/accounts/", Method::GET, None, true, 100).await;
    //println!("flow balance -> {:?}", flow_response);
    let flow_snapshot = balance_struct::BalanceSnapshot::new("Generic".to_string(), "FLOW".to_string(), flow_response.unwrap().balance.to_string(), 8); //9 from documentatino on decimal places
    //println!("{:?}", flow_snapshot);
    all_assets.snapshots.push(flow_snapshot);

    let tezos_client = block_structs::generic::structs::GenericClient::new(String::from(""), String::from("https://api.tzkt.io"));
    let tezos_Key = ""; //In future this wont be plain text but a proper encrypted key variabled
    let tezos_endpoint = format!("/v1/accounts/{}/balance", tezos_Key);
    let tezos_response = tezos_client.send_request::<Value>(&tezos_endpoint, Method::GET, None, false, 100).await;
    //println!("tezos balance -> {:?}", tezos_response);
    let tezos_snapshot = balance_struct::BalanceSnapshot::new("Generic".to_string(), "XTZ".to_string(), tezos_response.unwrap().to_string(), 6); //9 from documentatino on decimal places
    //println!("{:?}", tezos_snapshot);
    all_assets.snapshots.push(tezos_snapshot);

    let harmony_client = block_structs::harmony::structs::HarmonyClient::new(String::from(""));
    let harmony_response = harmony_client.get_total_balance().await;
    //println!("harmony balance: {}", harmoney_response);
    let harmony_snapshot = balance_struct::BalanceSnapshot::new("Harmony".to_string(), "ONE".to_string(), harmony_response.to_string(), 18); //9 from documentatino on decimal places
    //println!("{:?}", harmony_snapshot);
    all_assets.snapshots.push(harmony_snapshot);

    let lpt_client = block_structs::lpt::structs::LPTClient::new(String::from(""));
    let lpt_response = lpt_client.send_request::<Value>("", Method::GET, None).await;
    //println!("lpt balance -> {}", lpt_response);
    let lpt_snapshot = balance_struct::BalanceSnapshot::new("Lpt".to_string(), "LPT".to_string(), lpt_response.to_string(), 18); //9 from documentatino on decimal places
    //println!("{:?}", lpt_snapshot);
    all_assets.snapshots.push(lpt_snapshot);

    let polkadot_keys = ["", "", "", "", "", ""];
    let mut counter = 1;

    for key in polkadot_keys {
        let polkadot_client = block_structs::polkadot::structs::PolkadotClient::new(String::from(key));
        let polkadot_response = polkadot_client.get_total_balance().await;
        //println!("{} polkadot balance -> {}", counter, polkadot_response);
        let polkadot_snapshot = balance_struct::BalanceSnapshot::new(format!("Polkadot {}", counter), "KSM".to_string(), polkadot_response.to_string(), 12); //9 from documentatino on decimal places
        //println!("{:?}", polkadot_snapshot);
        all_assets.snapshots.push(polkadot_snapshot);

        counter += 1;
    }

    let uri = std::env::var("").expect("MONGODB_URI must be set");
    let client = mongodb::Client::with_uri_str(uri).await.unwrap();
    let db = client.database("");
    let collection: mongodb::Collection<balance_struct::BalanceSnapshot> = db.collection("");
    let insert_result = collection.insert_many(&all_assets.snapshots).await.unwrap();

    println!("{:?}", all_assets);
}

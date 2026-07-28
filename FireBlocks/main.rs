mod block_structs;
mod balance_struct;

use reqwest::Method;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let mut avax_client = block_structs::avax::structs::AvaxClient::new(String::from("P-avax1vj47glgnjlytzh2nsc8g4fvtuh54et7vpmd2v9"), String::from("0x3c2de10E0E3d14aE12318441522b877A6e0a6539"));
    avax_client.get_networks().await;
    let avax_response = avax_client.get_total_balance().await;

    println!("avax balance -> {:?}", avax_response);
    let avax_snapshot = balance_struct::BalanceSnapshot::new()

    let cartesi_client = block_structs::cartesi::structs::CartesiClient::new(String::from("0x3c2de10E0E3d14aE12318441522b877A6e0a6539"), String::from("0x48381609a2f1bfe30b465e106bf8324342abe107"));
    let cartesi_response = cartesi_client.send_request::<Value>("endpoint", Method::GET, None).await;

    println!("cartesi balance -> {:?}", cartesi_response.map_or(0.0, |r| r.total_balance));

    let mut cosmos_client = block_structs::cosmos::structs::CosmosClient::new(String::from("cosmos1vzwe79zdjelepczsktulndgcfc6nug3gfq6u5z"), None); //Atom Staking
    let CosmosBalance = cosmos_client.get_total_balance().await;

    println!("atom balance staked -> {:?}", CosmosBalance);

    let mut cosmos_client =  block_structs::cosmos::structs::CosmosClient::new(String::from("cosmos14s26z3n4g3erzh9naw5arpkh8k7zqtntzvkz44"), None); //Atom Cold
    let CosmosBalance = cosmos_client.get_total_balance().await;

    println!("atom balance cold -> {:?}", CosmosBalance);

    let mut cosmos_client =  block_structs::cosmos::structs::CosmosClient::new(String::from("secret1m5twqktzu626n8g7ppfl7d9kax50hqmyk43lc4"), None); //SCRT
    let CosmosBalance = cosmos_client.get_total_balance().await;

    println!("scrt balance -> {:?}", CosmosBalance);

    let mut cosmos_client =  block_structs::cosmos::structs::CosmosClient::new(String::from("kava1m5twqktzu626n8g7ppfl7d9kax50hqmyg93tnw"), None); //SCRT
    let CosmosBalance = cosmos_client.get_total_balance().await;

    println!("kava balance -> {:?}", CosmosBalance);

    let mut eth_client = block_structs::eth::structs::EthClient::new(String::from("0x3c2de10E0E3d14aE12318441522b877A6e0a6539"));
    let eth_balance = eth_client.get_total_balance().await;

    println!("eth balance -> {:?}", eth_balance);

    let oasis_client = block_structs::generic::structs::GenericClient::new(String::from("oasis1qp2zelkjf5eljkd30leuf9lxsj8r9hs56s0u5rhu"), String::from("http://api.oasisscan.com/mainnet"));
    let oasis_response = oasis_client.send_request::<Value>("/chain/account/info/", Method::GET, None, true, 5).await;

    println!("oasis balance -> {:?}", oasis_response);

    let btc_client = block_structs::generic::structs::GenericClient::new(String::from("bc1p5p8xqcl0lrxlf3kel2v4lqslf78kss2q8ct7gt08gvuuvvukqgps22dh7c"), String::from("https://mempool.space/api"));
    let btc_response = btc_client.send_request::<block_structs::generic::response_structs::BTC_Response>("/address/", Method::GET, None, true, 100).await;

    println!("btc balance -> {:?}", btc_response);

    let flow_client = block_structs::generic::structs::GenericClient::new(String::from("0x52aad21dcc281aa5"), String::from("https://rest-mainnet.onflow.org"));
    let flow_response = flow_client.send_request::<block_structs::generic::response_structs::Flow_Response>("/v1/accounts/", Method::GET, None, true, 100).await;

    println!("flow balance -> {:?}", flow_response);

    let tezos_client = block_structs::generic::structs::GenericClient::new(String::from("tz1Ypmhk23nqduyYCnt7x9M8o8aWSuMB36tX"), String::from("https://api.tzkt.io"));
    let tezos_Key = "tz1Ypmhk23nqduyYCnt7x9M8o8aWSuMB36tX"; //In future this wont be plain text but a proper encrypted key variabled
    let tezos_endpoint = format!("/v1/accounts/{}/balance", tezos_Key);
    let tezos_response = tezos_client.send_request::<Value>(&tezos_endpoint, Method::GET, None, false, 100).await;

    println!("tezos balance -> {:?}", tezos_response);

    let harmoney_client = block_structs::harmony::structs::HarmonyClient::new(String::from("one1fvvceuv6lzd0574858ct6w7qv6lp6g4x675zg0"));
    let harmoney_response = harmoney_client.get_total_balance().await;

    println!("harmony balance: {}", harmoney_response);

    let lpt_client = block_structs::lpt::structs::LPTClient::new(String::from("0x3c2de10E0E3d14aE12318441522b877A6e0a6539"));
    let lpt_response = lpt_client.send_request::<Value>("", Method::GET, None).await;

    println!("lpt balance -> {}", lpt_response);

    let mut near_client = block_structs::near::structs::NearClient::new(String::from(""));
    let _ = near_client.get_networks().await;
    let near_response = near_client.get_total_balance().await;

    println!("near balance -> {:?}", near_response);

    let polkadot_keys = ["Ghacj2E4HSukpiKfnDCaEoMtiNLepM1Qbyy7YAaLXCbyfmb", "JJGMQu3ZXNTJUnR9bvC4dPBBKWYe5924KRWmC37Gi39N443", "CusRg4TtuzTk383288KJH59qnTbJK9bJKnvRvSFaabkwErz", "CzZqQTsfNADsFLs71uv3nH6pn2meKD1bmzQBxANqArn38wt", "EfKedUxGa2nYb4rbweJdjRas5pKej7YFy9s7iu48DE7eEhw", "Cik3FxLLH3FRgqQ3QhrWbwXxvbrpesRcs2xY5yJSYfCZGSw"];
    let mut counter = 1;

    for key in polkadot_keys {
        let polkadot_client = block_structs::polkadot::structs::PolkadotClient::new(String::from(key));
        let polkadot_response = polkadot_client.get_total_balance().await;

        println!("{} polkadot balance -> {}", counter, polkadot_response);
        counter += 1;
    }
}

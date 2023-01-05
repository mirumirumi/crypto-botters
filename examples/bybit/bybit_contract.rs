use std::env;
use log::LevelFilter;
use serde_json::json;
use crypto_botters::{Client, bybit::{BybitOption}};
use crypto_botters_bybit::BybitHttpAuth;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let key = env::var("BYBIT_API_KEY").expect("no API key found");
    let secret = env::var("BYBIT_API_SECRET").expect("no API secret found");
    let mut client = Client::new();
    client.default_option(BybitOption::Key(key));
    client.default_option(BybitOption::Secret(secret));
    client.default_option(BybitOption::RecvWindow(6000));

    let cancel_all: serde_json::Value = client.post(
        "/contract/v3/private/order/cancel-all",
        Some(json!({"symbol": "BTCUSDT"})),
        [BybitOption::HttpAuth(BybitHttpAuth::Type2)],
    ).await.expect("failed to cancel orders");
    println!("Cancel all result:\n{}", cancel_all);
}

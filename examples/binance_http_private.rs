use std::env;
use log::LevelFilter;
use rust_decimal::prelude::*;
use serde::{Deserialize, Serialize};
use crypto_botters::{
    http::Client,
    binance::{Binance, BinanceSecurity, BinanceHttpUrl},
};

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let key = env::var("BINANCE_API_KEY").expect("no API key found");
    let secret = env::var("BINANCE_API_SECRET").expect("no API secret found");
    let binance = Binance::new(Some(key), Some(secret));
    let client = Client::new();

    // typed
    #[derive(Serialize)]
    struct TradesLookupParams<'a> {
        symbol: &'a str,
        limit: u16,
    }

    #[allow(dead_code)]
    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    struct OldTrade {
        id: i64,
        #[serde(with = "rust_decimal::serde::str")]
        price: Decimal,
        #[serde(with = "rust_decimal::serde::str")]
        qty: Decimal,
        #[serde(with = "rust_decimal::serde::str")]
        quote_qty: Decimal,
        time: u64,
        is_buyer_maker: bool,
        is_best_match: bool,
    }

    let trades: Vec<OldTrade> = client.get(
        "/api/v3/historicalTrades",
        Some(&TradesLookupParams { symbol: "BTCUSDT", limit: 3 }),
        &binance.request(BinanceSecurity::Key, BinanceHttpUrl::Spot),
    ).await.expect("failed to get trades");
    println!("Trade data:\n{:?}", trades);

    // not typed
    let dusts: serde_json::Value = client.post_no_body(
        "https://api.binance.com/sapi/v1/asset/dust-btc",
        &binance.request_no_url(BinanceSecurity::Sign),
    ).await.expect("failed get dusts");
    println!("My dust assets(BTC):\n{:?}", dusts["totalTransferBtc"]);
}

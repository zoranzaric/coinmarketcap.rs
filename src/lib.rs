//! A small unofficial wrapper for the
//! [coinmarketcap.com](https://coinmarketcap.com/) API.
//!
//! This is my first Rust library.  I'm releasing this to try the whole Cargo
//! packaging and relaese workflow.

use std::io::Read;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use serde::Deserialize;
use serde::de::{self, Deserializer, Unexpected};

extern crate reqwest;


#[derive(Debug, Deserialize)]
pub struct Symbol {
    id: String,
    name: String,
    symbol: String,
    #[serde(deserialize_with = "string_as_u32")] rank: u32,
    #[serde(deserialize_with = "string_as_f64")] price_usd: f64,
    #[serde(deserialize_with = "string_as_f64")] price_btc: f64,
    #[serde(deserialize_with = "string_as_f64")] price_eur: f64,
    #[serde(deserialize_with = "string_as_f64")] market_cap_usd: f64,
    #[serde(deserialize_with = "string_as_f64")] market_cap_eur: f64,
}

#[derive(Debug, Deserialize)]
pub struct Global {
    active_assets: u32,
    active_currencies: u32,
    active_markets: u32,
    bitcoin_percentage_of_market_cap: f64,
    total_24h_volume_eur: f64,
    total_24h_volume_usd: f64,
    total_market_cap_eur: f64,
    total_market_cap_usd: f64,
}

fn string_as_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<f64>().map_err(|_err| {
        de::Error::invalid_value(Unexpected::Str(&s), &"a string representation of a f64")
    })
}

fn string_as_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    s.parse::<u32>().map_err(|_err| {
        de::Error::invalid_value(Unexpected::Str(&s), &"a string representation of a u32")
    })
}

fn parse_ticker_result_to_symbols(json: String) -> Result<Vec<Symbol>, serde_json::Error> {
    serde_json::from_str(&json)
}

fn parse_global_result(json: String) -> Result<Global, serde_json::Error> {
    serde_json::from_str(&json)
}


fn fetch(url: &str) -> Result<String, reqwest::Error> {
    let mut result: reqwest::Response = match reqwest::get(url) {
        Ok(result) => result,
        Err(err) => return Err(err),
    };

    let mut content = String::new();
    let _ = result.read_to_string(&mut content);
    Ok(content)
}
/// Fetches all symbol data
pub fn ticker() -> Vec<Symbol> {
    let content = match fetch("https://api.coinmarketcap.com/v1/ticker/?convert=EUR") {
        Ok(content) => content,
        Err(err) => panic!("{}", err),
    };

    match parse_ticker_result_to_symbols(content) {
        Ok(symbols) => symbols,
        Err(e) => panic!("{}", e),
    }
}

/// Fetches global data like total market cap and number of active assets.
pub fn global() -> Global {
    let content = match fetch("https://api.coinmarketcap.com/v1/global/?convert=EUR") {
        Ok(content) => content,
        Err(err) => panic!("{}", err),
    };

    match parse_global_result(content) {
        Ok(global) => global,
        Err(e) => panic!("{}", e),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn parsing_ticker_example_should_work() {
        let example_content = r#"[
    {
        "id": "bitcoin",
        "name": "Bitcoin",
        "symbol": "BTC",
        "rank": "1",
        "price_usd": "8667.49",
        "price_btc": "1.0",
        "24h_volume_usd": "4899070000.0",
        "market_cap_usd": "146819479859",
        "available_supply": "16939100.0",
        "total_supply": "16939100.0",
        "max_supply": "21000000.0",
        "percent_change_1h": "1.07",
        "percent_change_24h": "-3.06",
        "percent_change_7d": "12.03",
        "last_updated": "1522004666",
        "price_eur": "7013.94959525",
        "24h_volume_eur": "3964449920.75",
        "market_cap_eur": "118809993589"
    },
    {
        "id": "ethereum",
        "name": "Ethereum",
        "symbol": "ETH",
        "rank": "2",
        "price_usd": "529.528",
        "price_btc": "0.0613264",
        "24h_volume_usd": "1182010000.0",
        "market_cap_usd": "52111526356.0",
        "available_supply": "98411276.0",
        "total_supply": "98411276.0",
        "max_supply": null,
        "percent_change_1h": "1.31",
        "percent_change_24h": "-1.53",
        "percent_change_7d": "5.64",
        "last_updated": "1522004652",
        "price_eur": "428.5072958",
        "24h_volume_eur": "956512042.25",
        "market_cap_eur": "42169949915.0"
    }
]
"#;
        match parse_ticker_result_to_symbols(String::from(example_content)) {
            Ok(symbols) => {
                assert_eq!(2, symbols.len());
            }
            Err(e) => {
                panic!("{}", e);
            }
        };
    }

    #[test]
    pub fn parsing_global_example_should_work() {
        let example_content = r#"
        {
            "active_assets": 677,
            "active_currencies": 919,
            "active_markets": 9914,
            "bitcoin_percentage_of_market_cap": 45.41,
            "last_updated": 1522597167,
            "total_24h_volume_eur": 8326584597.0,
            "total_24h_volume_usd": 10267058689.0,
            "total_market_cap_eur": 198469129734.0,
            "total_market_cap_usd": 244721491658.0
        }"#;
        match parse_global_result(String::from(example_content)) {
            Ok(global) => {
                assert_eq!(677, global.active_assets);
            }
            Err(e) => {
                panic!("{}", e);
            }
        };
    }
}

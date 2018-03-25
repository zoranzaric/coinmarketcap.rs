//! A small unofficial wrapper for the
//! [coinmarketcap.com](https://coinmarketcap.com/) API.
//!
//! This is my first Rust library.  I'm releasing this to try the whole Cargo
//! packaging and relaese workflow.

use std::fmt;
use std::io::Read;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use serde::de::{self, Deserializer, Unexpected, Visitor};

extern crate reqwest;


#[derive(Debug, Deserialize)]
pub struct Symbol {
    id: String,
    name: String,
    symbol: String,
    #[serde(deserialize_with = "string_as_u32")]
    rank: u32,
    #[serde(deserialize_with = "string_as_f64")]
    price_usd: f64,
    #[serde(deserialize_with = "string_as_f64")]
    price_btc: f64,
    #[serde(deserialize_with = "string_as_f64")]
    price_eur: f64,
    #[serde(deserialize_with = "string_as_f64")]
    market_cap_usd: f64,
    #[serde(deserialize_with = "string_as_f64")]
    market_cap_eur: f64,
}

fn string_as_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where D: Deserializer<'de>
{
    deserializer.deserialize_f64(F64Visitor)
}

struct F64Visitor;
impl<'de> Visitor<'de> for F64Visitor {
    type Value = f64;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representation of a f64")
    }

    fn visit_str<E>(self, value: &str) -> Result<f64, E>
        where E: de::Error
    {
        value.parse::<f64>().map_err(|_err| {
            E::invalid_value(Unexpected::Str(value), &"a string representation of a f64")
        })
    }
}

fn string_as_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
    where D: Deserializer<'de>
{
    deserializer.deserialize_u32(U32Visitor)
}

struct U32Visitor;
impl<'de> Visitor<'de> for U32Visitor {
    type Value = u32;
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representation of a u32")
    }
    
    fn visit_str<E>(self, value: &str) -> Result<u32, E>
        where E: de::Error
    {
        value.parse::<u32>().map_err(|_err| {
            E::invalid_value(Unexpected::Str(value), &"a string representation of a u32")
        })
    }
}

pub fn parse_data(data: String) -> Vec<Symbol> {
    serde_json::from_str(&data).unwrap()
}

pub fn fetch_data() -> Vec<Symbol> {
    let mut result: reqwest::Response =
        reqwest::get("https://api.coinmarketcap.com/v1/ticker/?convert=EUR").unwrap();

    let mut content = String::new();
    let _ = result.read_to_string(&mut content);

    let symbols: Vec<Symbol> = serde_json::from_str(&content).unwrap();
    symbols
}

#[cfg(test)]
mod test {
    #[test]
    pub fn parsing_example_should_work() {
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
        let symbols = ::parse_data(String::from(example_content));

        assert_eq!(2, symbols.len());
    }
}
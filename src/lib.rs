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

pub fn fetch_data() -> Vec<Symbol> {
    let mut result: reqwest::Response =
        reqwest::get("https://api.coinmarketcap.com/v1/ticker/?convert=EUR").unwrap();

    let mut content = String::new();
    result.read_to_string(&mut content);

    let symbols: Vec<Symbol> = serde_json::from_str(&content).unwrap();
    symbols
}

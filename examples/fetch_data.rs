extern crate coinmarketcap;

pub fn main() {
    for symbol in coinmarketcap::fetch_data() {
        println!("{:#?}", symbol);
    }
}

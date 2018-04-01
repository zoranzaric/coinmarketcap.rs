extern crate coinmarketcap;

pub fn main() {
    println!("{:#?}", coinmarketcap::global());
    
    // for symbol in coinmarketcap::ticker() {
    //     println!("{:#?}", symbol);
    // }
}

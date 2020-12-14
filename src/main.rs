extern crate lib;
use lib::blockchain::Blockchain;

fn main() {
    let chain = Blockchain::new();
    println!("BlockChain display: {}", chain);
}

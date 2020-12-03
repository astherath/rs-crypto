mod blockchain;
use blockchain::Blockchain;

fn main() {
    let chain = Blockchain::new();
    println!("BlockChain display: {}", chain);
}

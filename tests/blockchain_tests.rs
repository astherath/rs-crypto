extern crate lib;
use lib::blockchain::{Block, BlockChainData, Blockchain};

#[test]
fn test_create_block_chain_ok() {
    let blockchain = Blockchain::new();
}

#[test]
fn test_append_block_to_chain_ok() {
    let mut blockchain = Blockchain::new();
    let amount_of_blocks = 5;
    let blocks = generate_blocks(amount_of_blocks);
    for block in blocks {
        blockchain.append_block(block);
    }
}

fn generate_blocks(amount: u16) -> Vec<Block> {
    let mut blocks = Vec::new();
    for _ in 0..amount {
        let data = BlockChainData::new();
        let last_block_hash = "random_hash".to_string();
        blocks.push(Block::new(data, last_block_hash));
    }
    blocks
}

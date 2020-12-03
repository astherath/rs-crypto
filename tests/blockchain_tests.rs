use crate::src::blockchain::Blockchain;

#[test]
fn test_create_block_chain_ok() {
    let blockchain = Blockchain::new();
}

#[test]
fn test_append_block_to_chain_ok() {
    let blockchain = Blockchain::new();
    let blocks = generate_blocks();
    for block in &blocks {
        blockchain.append_block(block);
    }
}

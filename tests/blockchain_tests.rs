extern crate lib;
use lib::blockchain::{Block, Blockchain, Transaction, TransactionData};

#[test]
fn test_create_block_chain_ok() {
    let _blockchain = generate_empty_blockchain();
}

#[test]
fn test_append_block_to_chain_few_ok() {
    let mut blockchain = generate_empty_blockchain();
    let amount_of_blocks = 5;
    let blocks = generate_blocks(amount_of_blocks);
    append_list_of_blocks_to_blockchain(&mut blockchain, blocks);
    assert_eq!(blockchain.len(), amount_of_blocks);
}

#[test]
fn test_append_block_to_chain_medium_ok() {
    let mut blockchain = generate_empty_blockchain();
    let amount_of_blocks = 10_000;
    let blocks = generate_blocks(amount_of_blocks);
    append_list_of_blocks_to_blockchain(&mut blockchain, blocks);
    assert_eq!(blockchain.len(), amount_of_blocks);
}

#[test]
fn test_append_block_to_chain_large_ok() {
    let mut blockchain = generate_empty_blockchain();
    let amount_of_blocks = 50_000;
    let blocks = generate_blocks(amount_of_blocks);
    append_list_of_blocks_to_blockchain(&mut blockchain, blocks);
    assert_eq!(blockchain.len(), amount_of_blocks);
}

#[test]
fn test_get_most_recent_block_ok() {
    let mut blockchain = generate_empty_blockchain();
    let block = generate_single_block();
    let block_clone = block.clone();
    blockchain.append_block(block);
    let most_recent_block = blockchain.get_most_recent_block();
    assert_eq!(block_clone, *most_recent_block.unwrap());
}

#[test]
fn test_get_most_recent_block_empty_list_failure() {
    let blockchain = generate_empty_blockchain();
    let most_recent_block = blockchain.get_most_recent_block();
    assert_eq!(None, most_recent_block);
}

#[test]
fn test_sum_transaction_inputs() {
    let transaction = Transaction{
        inputs: vec![
            TransactionData{
                to_address: "Alice".to_string(),
                value: 25
            },
            TransactionData{
                to_address: "Bob".to_string(),
                value: 120
            }
        ],
        outputs: vec![]
    };
    let transaction_inputs = &transaction.inputs;
    let input_sum = transaction.get_sum_of_values(transaction_inputs);
    assert_eq!(input_sum, 145);
}

#[test]
fn test_sum_transaction_outputs() {
    let transaction = Transaction{
        inputs: vec![],
        outputs: vec![
            TransactionData{
                to_address: "Alice".to_string(),
                value: 23
            },
            TransactionData{
                to_address: "Bob".to_string(),
                value: 67
            }
        ]
    };
    let transaction_outputs = &transaction.outputs;
    let output_sum = transaction.get_sum_of_values(transaction_outputs);
    assert_eq!(output_sum, 90);
}

// helper functions for generating empty blocks
fn generate_blocks(amount: usize) -> Vec<Block> {
    let mut blocks = Vec::new();
    for _ in 0..amount {
        let transactions = vec![
            Transaction{
                inputs: vec![],
                outputs: vec![]
            }
        ];
        let last_block_hash = "random_hash".to_string();
        blocks.push(Block::new(transactions, last_block_hash));
    }
    blocks
}
fn generate_single_block() -> Block {
    let transactions = vec![
            Transaction{
                inputs: vec![
                    TransactionData{
                        to_address: "Alice".to_string(),
                        value: 50
                    }
                ],
                outputs: vec![
                    TransactionData{
                        to_address: "Bob".to_string(),
                        value: 25
                    },
                    TransactionData{
                        to_address: "Alice".to_string(),
                        value: 25
                    }
                ]
            }
        ];
    let last_block_hash = "random_hash".to_string();
    Block::new(transactions, last_block_hash)
}

fn generate_empty_blockchain() -> Blockchain {
    Blockchain::new()
}

fn append_list_of_blocks_to_blockchain(blockchain: &mut Blockchain, blocks: Vec<Block>) {
    for block in blocks {
        blockchain.append_block(block);
    }
}

use chrono::prelude::Utc;
use crypto::{digest::Digest, sha2::Sha256};
use uuid::Uuid;
pub use super::transaction::Transaction;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Block {
    hash_address: String,
    transactions: Vec<Transaction>,
    timestamp: String,
    last_block_hash: String,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, last_block_hash: String) -> Self {
        let hash_address = Self::generate_random_hash();
        let timestamp = Self::get_now_timestamp();
        Self {
            hash_address,
            transactions,
            timestamp,
            last_block_hash,
        }
    }

    fn get_now_timestamp() -> String {
        Utc::now().to_string()
    }

    fn generate_random_hash() -> String {
        let mut hasher = Sha256::new();
        hasher.input(Uuid::new_v4().to_string().as_bytes());
        hasher.result_str()
    }
}

// #[derive(Clone, Eq, PartialEq, Debug, Copy)]
// pub struct BlockChainData {}

// impl BlockChainData {
//     #[allow(clippy::new_without_default)]
//     pub fn new() -> Self {
//         Self {}
//     }
// }

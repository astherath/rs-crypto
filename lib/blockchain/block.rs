use chrono::prelude::Utc;
use crypto::{digest::Digest, sha2::Sha256};
use uuid::Uuid;

#[derive(Clone)]
pub struct Block {
    hash_address: String,
    data: BlockChainData,
    timestamp: String,
    last_block_hash: String,
}

impl Block {
    pub fn new(data: BlockChainData, last_block_hash: String) -> Self {
        let hash_address = Self::generate_random_hash();
        let timestamp = Self::get_now_timestamp();
        Self {
            hash_address,
            data,
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

#[derive(Copy, Clone)]
pub struct BlockChainData {}

impl BlockChainData {
    pub fn new() -> Self {
        Self {}
    }
}

use crypto::{digest::Digest, sha2::Sha256};
use std::collections::HashSet;

/// Transaction data serves as outputs and inputs of a transaction.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct TransactionData {
  pub to_address: String,
  pub value: u64,
}

impl TransactionData {
  pub fn for_hash_str(&self) -> String {
    format!("{}{}", self.value, self.value)
  }
}

/// A transaction consists of inputs and outputs.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Transaction {
  pub inputs: Vec<TransactionData>,
  pub outputs: Vec<TransactionData>,
}

impl Transaction {
  /// Give each transaction a hash to be later used in merkle tree.
  pub fn transaction_hash(&self) -> String {
    let mut hasher = Sha256::new();
    
    let input_hashes = self.input_hashes(&self.inputs);
    let output_hashes = self.input_hashes(&self.outputs);

    self.add_hash_to_hasher(&input_hashes, &mut hasher);
    self.add_hash_to_hasher(&output_hashes, &mut hasher);

    hasher.result_str()
  }

  fn add_hash_to_hasher(&self, input_hashes: &HashSet<String>, hasher: &mut Sha256) {
    input_hashes
        .iter()
        .for_each(|hash| hasher.input(hash.as_bytes()));
  }

  /// Hash of output data.
  fn generate_hash(&self, to_hash: &TransactionData) -> String {
    let mut hasher = Sha256::new();
    
    let transaction_data_hash = to_hash.for_hash_str();
    
    hasher.input(transaction_data_hash.as_bytes());
    hasher.result_str()
  }

  // Sum of value of input.
  pub fn get_sum_of_values(&self, input: &Vec<TransactionData>) -> u64 {
    input
        .iter()
        .map(|transaction_data| transaction_data.value)
        .sum()
  }

  /// We will use the set of hashed inputs to remove currency from the wallet
  /// of the user and the set of hashed outputs to add to the wallet. This keeps track
  /// of unspent transactions (UTXO).

  /// HashSet of hashed input.
  pub fn input_hashes(&self, inputs: &Vec<TransactionData>) -> HashSet<String>{
    inputs.iter().map(|input| {
      self.generate_hash(input)
    }).collect::<HashSet<String>>()
  }
}
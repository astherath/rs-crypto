use crypto::{digest::Digest, sha2::Sha256};
use std::collections::HashSet;

/// Since the output structure is the same as the input structure and outputs are
/// eventually used as inputs we just need to declare an output struct.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Output {
  pub to_address: String,
  pub value: u64,
}

/// A transaction consists of inputs and outputs.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Transaction {
  pub inputs: Vec<Output>,
  pub outputs: Vec<Output>,
}

impl Transaction {
  /// Give each transaction a hash to be later used in merkle tree.
  pub fn transaction_hash(&self) -> String {
    let mut hasher = Sha256::new();
    let total_sum: u64 = self.input_value() + self.output_value();

    hasher.input(total_sum.to_string().as_bytes());
    hasher.result_str()
  }

  /// Hash the vector of output data by summing the bytes.
  fn generate_hash(&self, to_hash: Vec<u8>) -> String {
    let mut hasher = Sha256::new();
    let to_hash_sum: u8 = to_hash.iter().sum();
    
    hasher.input(to_hash_sum.to_string().as_bytes());
    hasher.result_str()
  }

  /// Turn output data into vector of bytes to later be hashed.
  fn output_bytes(&self, output: &Output) -> Vec<u8> {
    let mut bytes = vec![];
    bytes.extend(output.to_address.as_bytes());
    bytes.extend(output.value.to_string().as_bytes());

    bytes
  }

  // Sum of value of each input.
  pub fn input_value(&self) -> u64 {
    self.inputs.iter().map(|input| input.value).sum()
  }

  // Sum of value of each output.
  pub fn output_value(&self) -> u64 {
    self.outputs.iter().map(|output| output.value).sum()
  }

  /// We will use the set of hashed inputs to remove currency from the wallet
  /// of the user and the set of hashed outputs to add to the wallet. This keeps track
  /// of unspent transactions (UTXO).

  /// HashSet of hashed inputs.
  pub fn input_hashes(&self) -> HashSet<String>{
    self.inputs.iter().map(|input| {
      let input_bytes = self.output_bytes(input);
      self.generate_hash(input_bytes)
    }).collect::<HashSet<String>>()
  }

  /// HashSet of hashed outputs.
  pub fn output_hashes(&self) -> HashSet<String>{
    self.outputs.iter().map(|output| {
      let output_bytes = self.output_bytes(output);
      self.generate_hash(output_bytes)
    }).collect::<HashSet<String>>()
  }
}
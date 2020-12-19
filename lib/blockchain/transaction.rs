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
    
    self.input_hashes().iter().for_each(|hash| hasher.input(hash.as_bytes()));
    self.output_hashes().iter().for_each(|hash| hasher.input(hash.as_bytes()));

    hasher.result_str()
  }

  /// Hash of output data.
  fn generate_hash(&self, to_hash: &Output) -> String {
    let mut hasher = Sha256::new();
    
    let output_value_string = to_hash.value.to_string();
    let owned_address = to_hash.to_address.to_owned();
    let combined_data = owned_address + &output_value_string;
    
    hasher.input(combined_data.as_bytes());
    hasher.result_str()
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
      self.generate_hash(input)
    }).collect::<HashSet<String>>()
  }

  /// HashSet of hashed outputs.
  pub fn output_hashes(&self) -> HashSet<String>{
    self.outputs.iter().map(|output| {
      self.generate_hash(output)
    }).collect::<HashSet<String>>()
  }
}
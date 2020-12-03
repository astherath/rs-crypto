//! This data structure _could_ be made to include a generic but it doesn't really make
//! sense to do all of that work without purpose - we're not ever gonna use this with a
//! non-block piece of data, so it's not worth the extra effort.
use append_only_list::List;
use std::fmt;
mod block;
pub use block::{Block, BlockChainData};

pub struct Blockchain {
    block_list: List,
}

impl Blockchain {
    pub fn new() -> Self {
        let block_list = List::new();
        Self { block_list }
    }

    pub fn append_block(&mut self, block: Block) {
        self.block_list.append_block(block);
    }

    pub fn len(&self) -> usize {
        self.block_list.get_amount_of_blocks()
    }

    pub fn get_most_recent_block(&self) -> Option<&Block> {
        self.block_list.get_most_recent_block()
    }
}

impl fmt::Display for Blockchain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // FIXME
        let display_data = String::new();
        write!(f, "{}", display_data)
    }
}

/// This inner module _could_ be in a different file, and it even was for a brief moment in
/// the commit history, but it introduced an unnecessary amount of 3-way coupling between
/// this, the list, and the block modules. This would make it very easy to extract to another
/// module in the future if it grows to be unmanageable but this works better for now.
mod append_only_list {
    use super::block::Block;

    pub struct List {
        data_list: Vec<Block>,
    }

    impl List {
        /// Instanciates an **empty** list.
        /// Blocks are the only data this structure interacts with.
        pub fn new() -> Self {
            Self {
                data_list: Vec::new(),
            }
        }

        pub fn append_block(&mut self, block: Block) {
            self.data_list.push(block);
        }

        pub fn get_most_recent_block(&self) -> Option<&Block> {
            self.data_list.last()
        }

        pub fn get_amount_of_blocks(&self) -> usize {
            self.data_list.len()
        }
    }
}

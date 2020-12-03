//! This data structure _could_ be made to include a generic but it doesn't really make
//! sense to do all of that work without purpose - we're not ever gonna use this with a
//! non-block piece of data, so it's not worth the extra effort.
use append_only_linked_list::List;
use std::fmt;
mod block;
use block::Block;

pub struct Blockchain {
    block_list: List,
}

impl Blockchain {
    pub fn new() -> Self {
        let block_list = List::new();
        Self { block_list }
    }

    pub fn append_block(mut self, block: Block) {
        self.block_list.append_block(block);
    }
}

impl fmt::Display for Blockchain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_data = String::new();
        write!(f, "{}", display_data)
    }
}

/// This inner module _could_ be in a different file, and it even was for a brief moment in
/// the commit history, but it introduced an unnecessary amount of 3-way coupling between
/// this, the list, and the block modules. This would make it very easy to extract to another
/// module in the future if it grows to be unmanageable but this works better for now.
mod append_only_linked_list {
    use super::block::Block;

    pub struct List {
        data: Option<Block>,
        next: Option<Box<Self>>,
    }

    impl List {
        /// Instanciates an **empty** linked list.
        /// Blocks are the only data this structure interacts with.
        pub fn new() -> Self {
            Self {
                data: None,
                next: None,
            }
        }

        /// Private function to create an instance based on a block
        fn from_block(block: &Block) -> Self {
            Self {
                data: Some(block.clone()),
                next: None,
            }
        }

        pub fn append_block(mut self, block: Block) {
            if self.is_empty() {
                self.data = Some(block);
            } else {
                let mut last_node = self.get_last_block();
                let new_node = Self::from_block(&block);
                last_node.next = Some(Box::new(new_node));
            }
        }

        fn get_last_block(self) -> Self {
            let mut current = self.next;
            loop {
                match current {
                    None => break,
                    Some(_) => current = current.unwrap().next,
                }
            }
            *current.unwrap()
        }

        /// This isn't a _super_ necessary method but it makes it a little easier to
        /// refactor in the future if we change to node based list (or something else).
        fn is_empty(&self) -> bool {
            self.data.is_none()
        }
    }
}

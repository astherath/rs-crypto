//! This data structure _could_ be made to include a generic but it doesn't really make
//! sense to do all of that work without purpose - we're not ever gonna use this with a
//! non-block piece of data, so it's not worth the extra effort.
use std::fmt;
use AppendOnlySinglyLinkedList::List;
mod block;

pub struct Blockchain {
    block_list: List,
}

impl Blockchain {
    pub fn new() -> Self {
        let block_list = List::new();
        Self { block_list }
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
mod AppendOnlySinglyLinkedList {
    use super::block::Block;

    pub struct List {
        data: Option<Block>,
        next: Option<&'static Self>,
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
        fn from_block(block: Block) -> Self {
            Self {
                data: Some(block),
                next: None,
            }
        }

        pub fn append_node(&mut self, block: Block) {
            if self.is_empty() {
                self.data = Some(block);
            } else {
                // find the last node and append to it
                let mut last_node = self.get_last_node();
                static mut new_node: List = List::from_block(block.clone());
                last_node.next = Some(&new_node);
            }
        }

        fn get_last_node(&self) -> &Self {
            let mut head = self;
            while head.next.is_some() {
                head = head.next.unwrap();
            }
            head
        }

        /// This isn't a _super_ necessary method but it makes it a little easier to
        /// refactor in the future if we change to node based list (or something else).
        fn is_empty(&self) -> bool {
            self.data.is_none()
        }
    }
}

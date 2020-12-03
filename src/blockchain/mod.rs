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
    pub struct List {}

    impl List {
        pub fn new() -> Self {
            Self {}
        }
    }
}

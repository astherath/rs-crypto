mod append_only_singly_linked_list;
mod block;

use append_only_singly_linked_list::List;

pub struct Blockchain {
    block_list: List,
}

impl Blockchain {
    pub fn new() -> Self {
        let block_list = List::new();
        Self { block_list }
    }
}

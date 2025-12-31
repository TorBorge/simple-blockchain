mod block;

use crate::{
    chain::block::Block,
    util::transaction::{Signed, Transactions},
};
pub struct BlockChain {
    blocks: Vec<Block>,
}

impl BlockChain {
    pub fn add_block(&mut self, txns: Transactions<Signed>) {
        let prev_blok = self.blocks.last().unwrap();
        let new_block = Block::new(txns, prev_blok.block_hash().clone());

        self.blocks.push(new_block);
    }
    pub fn new() -> Self {
        let genesis_block = Block::new_genesis_block();
        Self {
            blocks: vec![genesis_block],
        }
    }
    pub fn validate_previous_hash(&self, index: usize) -> bool {
        if index == 0 {
            return true;
        }
        self.blocks[index].previous_block_hash() == self.blocks[index - 1].block_hash()
    }
    pub fn validate_time(&self, index: usize) -> bool {
        if index == 0 {
            return true;
        }
        self.blocks[index].time_stamp() >= self.blocks[index - 1].time_stamp()
    }
    pub fn validate_chain(&self) -> bool {
        for index in (0..self.blocks.len()).rev() {
            let block = &self.blocks[index];
            if !block.validate_block_hash() {
                return false;
            }
            if !block.validate_payload_hash() {
                return false;
            }
            if !self.validate_previous_hash(index) {
                return false;
            }
            if !self.validate_time(index) {
                return false;
            }
        }
        true
    }

    pub fn get_blocks(&self) -> &[Block] {
        &self.blocks
    }
}

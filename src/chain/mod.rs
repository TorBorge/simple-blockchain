mod block;

use crate::{
    chain::block::{Block, BlockIter},
    util::transaction::{Signed, Transactions},
};
pub struct BlockChain {
    latest_block: Option<Box<Block>>,
    chain_id: u64,
}

impl BlockChain {
    pub fn add_block(self, txs: Transactions<Signed>) -> Self {
        let prev_blok = self.latest_block;
        let new_block = Some(Box::new(Block::new(txs, prev_blok)));

        Self {
            latest_block: new_block,
            ..self
        }
    }
    pub fn new() -> Self {
        Self {
            latest_block: Some(Box::new(Block::new_genesis_block())),
            chain_id: rand::random(),
        }
    }
    pub fn validate_chain(&self) -> bool {
        for block in self.iter() {
            if !block.validate_block() {
                return false;
            }
        }
        true
    }

    pub fn iter(&self) -> BlockIter<'_> {
        BlockIter {
            next: self.latest_block.as_deref(),
        }
    }
}

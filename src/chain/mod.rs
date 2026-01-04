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
        let new_block = Some(Box::new(Block::new(2, txs, prev_blok)));

        Self {
            latest_block: new_block,
            ..self
        }
    }
    pub fn new() -> Self {
        Self {
            latest_block: Some(Box::new(Block::new_genesis_block(2))),
            chain_id: rand::random(),
        }
    }
    pub fn validate_chain(&self) -> bool {
        for block in self.iter() {
            if !block.validate_block().is_ok() {
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

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use crate::chain::BlockChain;

    #[test]
    fn validates_genesis_only_chain() {
        let chain = BlockChain::new();

        assert!(chain.validate_chain());
    }

    #[test]
    fn validates_chain_with_multiple_blocks() {
        let mut chain = BlockChain::new();
        thread::sleep(Duration::from_secs(1));
        chain = chain.add_block(vec![]);
        thread::sleep(Duration::from_secs(1));
        chain = chain.add_block(vec![]);

        assert!(chain.validate_chain());
    }
}

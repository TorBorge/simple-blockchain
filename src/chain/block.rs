use serde::Serialize;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::util::{
    hash::Hash,
    transaction::{Signed, Transactions},
};

type OptionalBlockPtr = Option<Box<Block>>;

#[derive(Debug)]
pub enum ValidationError {
    BlockHash {
        expected: Hash,
        found: Hash,
    },
    PayloadHash {
        expected: Hash,
        found: Hash,
    },
    TimeDescrepency {
        cur_block: SystemTime,
        prev_block: SystemTime,
    },
    PrevBlockHash {
        cur_block: Hash,
        prev_block: Hash,
    },
}
impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BlockHash { expected, found } => write!(
                f,
                "A recomputed Hash of the block did not match the existing copy, expected: {expected}, found: {found}."
            ),
            Self::PayloadHash { expected, found } => write!(
                f,
                "A recomputed Hash of the payload did not match the existing copy, expected: {expected}, found: {found}."
            ),

            Self::TimeDescrepency {
                cur_block,
                prev_block,
            } => write!(f, "The previous block was created after the current block"),
            Self::PrevBlockHash {
                cur_block,
                prev_block,
            } => write!(
                f,
                "The current Blocks copy of the previous blocks hash does not match the copy held by the previous block"
            ),
        }
    }
}

impl std::error::Error for ValidationError {}

#[derive(Serialize)]
pub struct BlockHeaders {
    version: String,
    timestamp: u64,
    prev_hash: Hash,
    body_hash: Hash,
    target: usize,
    nonce: u64,
    //merkle_root
}

impl BlockHeaders {
    fn new(target: usize, prev_hash: Hash, body_hash: Hash) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u64;

        Self {
            prev_hash,
            version: String::from("v1.0"),
            timestamp,
            body_hash,
            target,
            nonce: 0,
        }
    }
    fn compute_nonce(&mut self) {
        let leading_zeros = vec![0u8; self.target];
        while !Hash::from(&self).get_bytes().starts_with(&leading_zeros) {
            self.nonce += 1
        }
    }
}

#[derive(Serialize)]
pub struct BlockBody {
    transactions: Transactions<Signed>,
}

impl BlockBody {
    fn new(transactions: Transactions<Signed>) -> Self {
        Self { transactions }
    }
}

#[derive(Serialize)]
pub struct Block {
    headers: BlockHeaders,
    body: BlockBody,
    block_hash: Hash,
    prev_block: OptionalBlockPtr,
}

impl Block {
    pub fn new(target: usize, txs: Transactions<Signed>, prev_block: OptionalBlockPtr) -> Block {
        let body = BlockBody::new(txs);
        let body_hash = Hash::from(&body);
        let prev_hash = prev_block
            .as_deref()
            .map_or(Hash::default(), |block| block.block_hash().clone());

        let mut headers = BlockHeaders::new(target, prev_hash, body_hash);
        headers.compute_nonce();
        let block_hash = Hash::from(&headers);

        Self {
            headers,
            body,
            block_hash,
            prev_block,
        }
    }

    pub fn new_genesis_block(target: usize) -> Block {
        Self::new(target, vec![], None)
    }
    //get methods
    pub fn block_hash(&self) -> &Hash {
        &self.block_hash
    }
    pub fn prev_block_hash(&self) -> &Hash {
        &self.headers.prev_hash
    }
    pub fn body_hash(&self) -> &Hash {
        &self.headers.body_hash
    }
    pub fn timestamp(&self) -> u64 {
        self.headers.timestamp
    }
    pub fn data(&self) -> &Transactions<Signed> {
        &self.body.transactions
    }

    //hash validation
    pub fn validate_block_hash(&self) -> Result<(), ValidationError> {
        let computed_block_hash = Hash::from(&self.headers);
        if computed_block_hash != self.block_hash {
            return Err(ValidationError::BlockHash {
                expected: self.block_hash.clone(),
                found: computed_block_hash,
            });
        }
        Ok(())
    }
    pub fn validate_body_hash(&self) -> Result<(), ValidationError> {
        let computed_payload_hash = Hash::from(&self.body);
        if computed_payload_hash != self.headers.body_hash {
            return Err(ValidationError::PayloadHash {
                expected: self.headers.body_hash.clone(),
                found: computed_payload_hash,
            });
        }
        Ok(())
    }

    pub fn validate_time(&self) -> Result<(), ValidationError> {
        if let Some(ref prev_block) = self.prev_block {
            if self.timestamp() < prev_block.timestamp() {
                return Err(ValidationError::TimeDescrepency {
                    cur_block: UNIX_EPOCH
                        .checked_add(Duration::from_secs(self.timestamp()))
                        .unwrap(),
                    prev_block: UNIX_EPOCH
                        .checked_add(Duration::from_secs(prev_block.timestamp()))
                        .unwrap(),
                });
            }
        };
        Ok(())
    }
    pub fn validate_previous_hash(&self) -> Result<(), ValidationError> {
        if let Some(ref prev_block) = self.prev_block {
            if self.headers.prev_hash != prev_block.block_hash {
                return Err(ValidationError::PrevBlockHash {
                    cur_block: self.headers.prev_hash.clone(),
                    prev_block: prev_block.block_hash.clone(),
                });
            }
        };
        Ok(())
    }
    pub fn validate_block(&self) -> Result<(), Vec<ValidationError>> {
        let validators: [fn(&Block) -> Result<(), ValidationError>; 4] = [
            Block::validate_previous_hash,
            Block::validate_time,
            Block::validate_body_hash,
            Block::validate_block_hash,
        ];

        let errors: Vec<ValidationError> = validators
            .iter()
            .filter_map(|validate| validate(self).err())
            .collect();

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

pub struct BlockIter<'a> {
    pub next: Option<&'a Block>,
}

impl<'a> Iterator for BlockIter<'a> {
    type Item = &'a Block;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|block| {
            self.next = block.prev_block.as_deref();
            block
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Block;
    use crate::util::hash::Hash;

    #[test]
    fn valid_block_passes_validation() {
        let genesis = Block::new_genesis_block(0);
        assert!(genesis.validate_block().is_ok());

        let block = Block::new(0, vec![], Some(Box::new(genesis)));
        assert!(block.validate_block().is_ok());
    }

    #[test]
    fn tampered_block_hash_fails_validation() {
        let mut block = Block::new_genesis_block(0);
        assert!(block.validate_block_hash().is_ok());

        block.block_hash = Hash::default();
        assert!(block.validate_block_hash().is_err());
    }

    #[test]
    fn tampered_payload_hash_fails_validation() {
        let mut block = Block::new_genesis_block(0);
        assert!(block.validate_body_hash().is_ok());

        block.headers.body_hash = Hash::default();
        assert!(block.validate_body_hash().is_err());
    }

    #[test]
    fn incorrect_previous_hash_fails_validation() {
        let genesis = Block::new_genesis_block(0);

        let mut block = Block::new(0, vec![], Some(Box::new(genesis)));
        assert!(block.validate_previous_hash().is_ok());

        block.headers.prev_hash = Hash::default();
        assert!(block.validate_previous_hash().is_err());
    }

    #[test]
    fn non_monotonic_time_fails_validation() {
        let genesis = Block::new_genesis_block(0);
        let mut block = Block::new(0, vec![], Some(Box::new(genesis)));

        // Force prev_block.timestamp > current_block.timestamp
        let cur_ts = block.headers.timestamp;
        block.prev_block.as_mut().unwrap().headers.timestamp = cur_ts + 1;

        assert!(block.validate_time().is_err());
    }
}

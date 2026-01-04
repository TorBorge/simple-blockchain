use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::util::{
    hash::Hash,
    transaction::{Signed, Transactions},
};

#[derive(Serialize)]
pub struct BlockHeaders {
    version: String,
    time_stamp: i64,
    previous_block_hash: Hash,
    previous_block: Option<Box<Block>>,
    payload_hash: Hash,
    target: usize,
    nonce: u64,
    //merkle_root
}

impl BlockHeaders {
    fn new(previous_block: Option<Box<Block>>, payload_hash: Hash) -> Self {
        let time_stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let previous_block_hash = previous_block
            .as_deref()
            .map_or(Hash::default(), Hash::from);
        Self {
            previous_block_hash,
            previous_block: previous_block,
            version: String::from("v1.0"),
            time_stamp,
            payload_hash,
            target: 2,
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
}

impl Block {
    pub fn new(txs: Transactions<Signed>, prev_block: Option<Box<Block>>) -> Block {
        let body = BlockBody::new(txs);
        let payload_hash = Hash::from(&body);

        let mut headers = BlockHeaders::new(prev_block, payload_hash);
        headers.compute_nonce();
        let block_hash = Hash::from(&headers);

        Self {
            headers,
            body,
            block_hash,
        }
    }

    pub fn new_genesis_block() -> Block {
        Self::new(vec![], None)
    }
    //get methods
    pub fn block_hash(&self) -> &Hash {
        &self.block_hash
    }
    pub fn prev_block_hash(&self) -> &Hash {
        &self.headers.previous_block_hash
    }
    pub fn payload_hash(&self) -> &Hash {
        &self.headers.payload_hash
    }
    pub fn time_stamp(&self) -> i64 {
        self.headers.time_stamp
    }
    pub fn data(&self) -> &Transactions<Signed> {
        &self.body.transactions
    }

    //hash validation
    pub fn validate_block_hash(&self) -> bool {
        Hash::from(&self.headers) == self.block_hash
    }
    pub fn validate_payload_hash(&self) -> bool {
        Hash::from(&self.body) == self.headers.payload_hash
    }

    pub fn validate_time(&self) -> bool {
        if let Some(ref block) = self.headers.previous_block {
            return self.time_stamp() > block.time_stamp();
        };
        true
    }
    pub fn validate_previous_hash(&self) -> bool {
        if let Some(ref block) = self.headers.previous_block {
            return self.prev_block_hash() == block.block_hash();
        };
        true
    }
    pub fn validate_block(&self) -> bool {
        self.validate_previous_hash()
            && self.validate_time()
            && self.validate_payload_hash()
            && self.validate_block_hash()
    }
}

pub struct BlockIter<'a> {
    pub next: Option<&'a Block>,
}

impl<'a> Iterator for BlockIter<'a> {
    type Item = &'a Block;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|block| {
            self.next = block.headers.previous_block.as_deref();
            block
        })
    }
}

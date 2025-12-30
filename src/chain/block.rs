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
    payload_hash: Hash,
    target: usize,
    nonce: u64,
    //merkle_root
}

impl BlockHeaders {
    fn new(previous_block_hash: Hash, payload_hash: Hash) -> Self {
        let time_stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        Self {
            version: String::from("v1.0"),
            time_stamp,
            previous_block_hash,
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

pub struct Block {
    headers: BlockHeaders,
    body: BlockBody,
    block_hash: Hash,
}

impl Block {
    pub fn new(txns: Transactions<Signed>, prev_block_hash: Hash) -> Block {
        let body = BlockBody::new(txns);
        let payload_hash = Hash::from(&body);

        let mut headers = BlockHeaders::new(prev_block_hash.clone(), payload_hash);
        headers.compute_nonce();
        let block_hash = Hash::from(&headers);

        Self {
            headers,
            body,
            block_hash,
        }
    }

    pub fn new_genesis_block() -> Block {
        Self::new(vec![], Hash::default())
    }
    //get methods
    pub fn block_hash(&self) -> &Hash {
        &self.block_hash
    }
    pub fn previous_block_hash(&self) -> &Hash {
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
}

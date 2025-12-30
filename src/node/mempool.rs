use std::time::SystemTimeError;

use crate::util::{
    hash::Hash,
    transaction::{Signed, Transaction},
};

type Result<T> = std::result::Result<T, MempoolError>;

#[derive(Debug, Clone)]
pub enum MempoolError {
    SearchErr(String),
    InsertErr(String),
}

impl From<SystemTimeError> for MempoolError {
    fn from(value: SystemTimeError) -> Self {
        MempoolError::InsertErr(format!(
            "fatal error occured while dating transaction: {}",
            value,
        ))
    }
}

impl From<bcs::Error> for MempoolError {
    fn from(value: bcs::Error) -> Self {
        MempoolError::InsertErr(format!(
            "fatal error occured while serializing transaction: {}",
            value,
        ))
    }
}

enum PoolPriority {
    Low,
    Normal,
    High,
}

pub struct PoolEntry {
    id: Hash,
    time_stamp: i64,
    size: usize,
    fee: u64,
    value_ratio: u64,
    txn: Transaction<Signed>,
    prio: PoolPriority,
}

pub struct Mempool {
    pool: Vec<PoolEntry>,
}

impl Mempool {
    pub fn new() -> Self {
        Self { pool: Vec::new() }
    }

    pub fn insert_transaction(
        &mut self,
        txn: Transaction<Signed>,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let time_stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs() as i64;
        let txn_bytes = bcs::to_bytes(&txn)?;

        let fee = txn.get_fee();
        let size = txn_bytes.len();

        let entry = PoolEntry {
            id: Hash::raw(&txn_bytes),
            time_stamp,
            size,
            fee,
            value_ratio: (size as u64) / fee,

            txn: txn,
            prio: PoolPriority::Normal,
        };
        self.pool.push(entry);

        Ok(())
    }
    pub fn find_txn_by_id(&self, txn_id: Hash) -> Result<&PoolEntry> {
        if let Some(entry) = self.pool.iter().find(|txn| txn.id == txn_id) {
            Ok(entry)
        } else {
            Err(MempoolError::SearchErr(format!(
                "unable to locate transaction with id: {:?}",
                txn_id
            )))
        }
    }
}

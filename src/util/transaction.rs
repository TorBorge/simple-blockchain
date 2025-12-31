use ed25519_dalek::{Signature, VerifyingKey};
use serde::Serialize;
use serde_bytes::ByteArray;

use crate::util::hash::Address;

pub type Payload = Option<ByteArray<256>>;

pub trait SigState {}

#[derive(Serialize, Debug)]
pub struct Signed {
    sig: Signature,
}

#[derive(Serialize, Debug)]
pub struct Unsigned;

impl SigState for Signed {}
impl SigState for Unsigned {}

#[derive(Serialize, Debug)]
pub struct TxnCtx {
    sender_pub_key: VerifyingKey,
    recipient: Address,
    data: Payload,
    version: u16,
    chain_id: u64,
    nonce: u64,
    value: std::num::NonZeroU64,
    fee: std::num::NonZeroU64,
}

#[derive(Serialize, Debug)]
pub struct Transaction<S: SigState> {
    state: S,
    ctx: TxnCtx,
}

impl Transaction<Unsigned> {
    pub fn new(
        sender_pub_key: VerifyingKey,
        recipient: Address,
        data: Payload,
        version: u16,
        chain_id: u64,
        nonce: u64,
        value: std::num::NonZeroU64,
        fee: std::num::NonZeroU64,
    ) -> Self {
        Self {
            state: Unsigned {},
            ctx: TxnCtx {
                sender_pub_key,
                recipient,
                data,
                version,
                chain_id,
                nonce,
                value,
                fee,
            },
        }
    }
}

impl<S: SigState> Transaction<S> {
    pub fn get_fee(&self) -> std::num::NonZeroU64 {
        self.ctx.fee
    }
    pub fn get_sender_pub_key(&self) -> VerifyingKey {
        self.ctx.sender_pub_key
    }
}

impl Transaction<Unsigned> {
    pub fn sign(self, sig: Signature) -> Transaction<Signed> {
        Transaction {
            state: Signed { sig },
            ctx: self.ctx,
        }
    }
}

impl Transaction<Signed> {
    pub fn get_sig(&self) -> &Signature {
        &self.state.sig
    }
}

pub type Transactions<S> = Vec<Transaction<S>>;

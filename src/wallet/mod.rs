use ed25519_dalek::{SigningKey, VerifyingKey, ed25519::signature::SignerMut};
use rand::rngs::OsRng;

use crate::util::{
    hash::Address,
    transaction::{Payload, Signed, Transaction, Unsigned},
};

pub struct Wallet {
    sig_key: SigningKey,
}

impl Wallet {
    fn new(sig_key: SigningKey) -> Self {
        Self { sig_key }
    }
    pub fn new_random() -> Self {
        let mut csprng = OsRng;
        let signing_key = SigningKey::generate(&mut csprng);
        Self::new(signing_key)
    }

    pub fn create_transaction(
        &self,
        recipient: Address,
        data: Payload,
        version: u16,
        chain_id: u64,
        nonce: u64,
        value: std::num::NonZeroU64,
        fee: std::num::NonZeroU64,
    ) -> Transaction<Unsigned> {
        Transaction::new(
            self.sig_key.verifying_key(),
            recipient,
            data,
            version,
            chain_id,
            nonce,
            value,
            fee,
        )
    }
    pub fn sign(&mut self, txn: Transaction<Unsigned>) -> bcs::Result<Transaction<Signed>> {
        let bytes = bcs::to_bytes(&txn)?;
        let sig = self.sig_key.sign(&bytes);

        Ok(txn.sign(sig))
    }
    pub fn get_pub_key(&self) -> VerifyingKey {
        self.sig_key.verifying_key()
    }
    pub fn get_address(&self) -> Address {
        Address::from(&self.get_pub_key())
    }
}

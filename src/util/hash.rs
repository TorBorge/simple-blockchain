use serde::Serialize;
use serde_bytes::ByteArray;
use sha2::{Digest, Sha256};

const HASH_LEN: usize = 32;

#[derive(Serialize, Clone, PartialEq, Eq, Default)]
pub struct Hash {
    bytes: ByteArray<HASH_LEN>,
}

impl Hash {
    pub fn from<T>(data: &T) -> Self
    where
        T: ?Sized + Serialize,
    {
        let bytes = bcs::to_bytes(&data).expect("failed to convert data to bytes when hashing");
        Self::raw(&bytes)
    }
    pub fn raw(bytes: &[u8]) -> Self {
        let hash = Sha256::digest(&bytes);

        let mut buf = [0u8; HASH_LEN];
        buf.copy_from_slice(hash.as_slice());

        Self {
            bytes: ByteArray::from(buf),
        }
    }
    pub fn get_bytes(&self) -> &ByteArray<HASH_LEN> {
        &self.bytes
    }
}

impl std::fmt::Debug for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", hex::encode(self.bytes.as_slice()))
    }
}

pub type Address = Hash;

use std::fmt;

use bincode::{Decode, Encode};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode)]
pub struct Hash(pub [u8; 32]);

impl Hash {
    pub fn hash(data: &[u8]) -> Self {
        Self(Sha256::digest(data).into())
    }

    pub fn zero() -> Self {
        Self([0; 32])
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.0.iter() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

//! 1. Proof-of-work implementation:
//!    a. Serialize all fields in [Block] except [Block::hash] with [Block::nonce] set to 0,
//!    b. Hash the serialized data using a hashing function such as [blake3::hash] or any other library.
//!    c. Iterate over [Block::nonce] until the first byte of [Block::hash] is 0 (most significant byte),
//!    d. Set the hash and nonce to the block.
//!    e. 🎉 That's it! You just mined the first block.

use serde::Serialize;

/// Simplified block structure.
#[derive(Debug, Default, Serialize)]
struct Block {
    /// Index of the block in the blockchain
    index: u64,
    /// Data stored in the block
    data: String,
    /// Hash of the previous block
    previous_hash: [u8; 32],
    /// Hash of the current block
    #[serde(skip_serializing)]
    hash: [u8; 32],
    /// Nonce
    nonce: u128,
}

pub impl Block {
    pub fn calculate_hash(&self) -> String {
        let block_data = self.clone();
        let serialized_block_data = serde_json::to_string(&block_data).unwrap();
        format!("{:?}", blake3::hash(serialized_block_data.as_bytes()).as_bytes().first())
    }
    pub fn mine(&mut self) {
        for nonce in 0.. {
            self.nonce = nonce;
            let serialized_block = serde_json::to_vec(&self).unwrap();
            self.hash = *blake3::hash(&serialized_block).as_bytes();
            if self.hash[0] == 0 {
                return;
            }
        }
    }
}

//! Implement a simplified blockchain.
//!
//! We are developing a simple blockchain system that stores strings within blocks.
//!
//! A [Block] is a data structure that holds information, such as a list of transactions,
//! and is uniquely identified by its hash.
//!
//!           BLOCK #n
//!   ┌─────────┬───────────────┐
//!   │ index N │ previous_hash |
//!   ├─────────┴───────────────┤
//!   │ data                    │
//!   ├─────────────┬───────────┤
//!   │ nonce       │     hash  │
//!   └─────────────┴───────────┘
//!
//! The difficulty target can be defined as the number of leading zeroes in the hash. The nonce is
//! a number that miners adjust in order to find the right hash value that meets the difficulty target.
//!
//! A blockchain is a sequence of blocks, where each block refers to the hash of the previous block.
//!
//!           BLOCK #n                      BLOCK #n+1
//!   ┌─────────┬───────────────┐      ┌───────────┬───────────────┐
//!   │ index N │ previous_hash |      │ index N+1 │ previous_hash ├──┐
//!   ├─────────┴───────────────┤      ├───────────┴───────────────┤  |
//!   │ data                    │      │ data                      │  |
//!   ├─────────────┬───────────┤      ├───────────────┬───────────┤  |
//!   │ nonce       │     hash  │◄──┐  │ nonce         │      hash │  |
//!   └─────────────┴───────────┘   |  └───────────────┴───────────┘  |
//!                                 |                                 |
//!                                 └─────────────────────────────────┘
//!
//! 1. Proof-of-work implementation:
//!    a. Serialize all fields in [Block] except [Block::hash] with [Block::nonce] set to 0,
//!    b. Hash the serialized data using a hashing function such as [blake3::hash] or any other library.
//!    c. Iterate over [Block::nonce] until the first byte of [Block::hash] is 0 (most significant byte),
//!    d. Set the hash and nonce to the block.
//!    e. 🎉 That's it! You just mined the first block.
//!
//! 2. Implement the mining difficulty:
//!    In step 1c., we implemented a difficulty target equals to 1,
//!
//!    a. The code should be updated to compute a hash with a difficulty target set to [DIFFICULTY_TARGET].
//!
//! 3. Implement a chain of blocks:
//!    a. The first block has a previous_hash set to [0; 32],
//!    b. Create a block with the hash of the previous and a random string,
//!    c. Compute the nonce and hash to meet the difficulty target,
//!    d. Add it to the list of blocks.
//!
//! 4. Spawn two [tokio::task]s that exchange data across a [tokio::sync::mpsc::channel]:
//!    a. One task sends random strings every 500 ms to the channel (see [data_feed]),
//!    b. The other tasks mines a block with this string and adds it to the blockchain.

mod block;

use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::Serialize;
use std::time::Duration;
use tokio::sync::mpsc::Sender;



const DIFFICULTY_TARGET: usize = 2;

/// Return a 30-character random string.
fn get_random_string() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

/// Send a random string every 500ms to a channel.
async fn data_feed(tx: Sender<String>) {
    loop {
        let data = get_random_string();

        if let Err(err) = tx.send(data).await {
            eprintln!("failed to send data: {err:?}");
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

#[tokio::main]
async fn main() {
    let mut block = Block::default();
    block.data = get_random_string();

    println!("block: {block:?}");
}
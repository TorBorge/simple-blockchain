#![allow(dead_code)]
mod chain;
mod node;
mod util;
mod wallet;

use chain::BlockChain;
use core::time;
use std::time::SystemTime;

fn main() {
    let chain = BlockChain::new();
    std::thread::sleep(time::Duration::from_secs(1));
    let chain = chain.add_block(Vec::new());
    std::thread::sleep(time::Duration::from_secs(1));
    let chain = chain.add_block(Vec::new());
    println!("{:?}", SystemTime::now());

    for block in chain.iter() {
        println!("time stamp {:?}", block.time_stamp());
        println!("Hash of the block {:?}", block.block_hash());
        println!("All the transactions: {:?}", block.data());
    }
    println!("{}", chain.validate_chain());
}

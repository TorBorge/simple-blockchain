mod mempool;
mod mining_loop;
use crate::{
    chain::BlockChain,
    node::{mempool::Mempool, mining_loop::MiningLoop},
};

pub struct Node {
    chain: BlockChain,
    mempool: Mempool,
    mining_loop: MiningLoop,
    //networking
}

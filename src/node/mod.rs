mod mempool;
mod mining_loop;
use crate::{
    chain::BlockChain,
    node::mining_loop::MiningLoop,
    util::transaction::{Signed, Transactions},
};

pub struct Node {
    chain: BlockChain,
    mempool: Transactions<Signed>,
    mining_loop: MiningLoop,
    //networking
}

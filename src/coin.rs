use zcash_primitives::consensus::{BlockHeight, BranchId, Network};
use serde::{Serialize, Deserialize};

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum CoinType {
    Ycash, Zcash,
}

pub fn get_coin_type(coin: u8) -> CoinType {
    match coin {
        0 => CoinType::Zcash,
        1 => CoinType::Ycash,
        _ => CoinType::Zcash,
    }
}

pub fn get_coin_id(coin: CoinType) -> u8 {
    match coin {
        CoinType::Zcash => 0,
        CoinType::Ycash => 1,
    }
}

struct YCASH;
struct ZCASH;

pub fn get_coin_chain(c: CoinType) -> &'static (dyn CoinChain + Send) {
    match c {
        CoinType::Ycash => &YCASH,
        CoinType::Zcash => &ZCASH,
    }
}

pub trait CoinChain: Send + Sync {
    fn network(&self) -> &'static Network;
    fn ticker(&self) -> &'static str;
}

impl CoinChain for YCASH {
    fn network(&self) -> &'static Network {
        &Network::YCashMainNetwork
    }

    fn ticker(&self) -> &'static str {
        "ycash"
    }
}

impl CoinChain for ZCASH {
    fn network(&self) -> &'static Network {
        &Network::MainNetwork
    }

    fn ticker(&self) -> &'static str {
        "zcash"
    }
}

pub fn get_branch(network: &Network, height: u32) -> BranchId {
    BranchId::for_height(network, BlockHeight::from_u32(height))
}


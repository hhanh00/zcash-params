use zcash_primitives::consensus::{BlockHeight, BranchId, Network};

struct YCASH;
struct ZCASH;
struct PIRATECHAIN;
struct OTHER;

pub trait CoinChain: Send + Sync {
    fn network(&self) -> &'static Network;
    fn ticker(&self) -> &'static str;
    fn has_transparent(&self) -> bool;
    fn has_unified(&self) -> bool;
}

impl CoinChain for YCASH {
    fn network(&self) -> &'static Network {
        &Network::YCashMainNetwork
    }

    fn ticker(&self) -> &'static str {
        "ycash"
    }

    fn has_transparent(&self) -> bool {
        true
    }

    fn has_unified(&self) -> bool {
        false
    }
}

impl CoinChain for ZCASH {
    fn network(&self) -> &'static Network {
        &Network::MainNetwork
    }

    fn ticker(&self) -> &'static str {
        "zcash"
    }

    fn has_transparent(&self) -> bool {
        true
    }

    fn has_unified(&self) -> bool {
        true
    }
}

impl CoinChain for PIRATECHAIN {
    fn network(&self) -> &'static Network {
        &Network::PirateChainMainNetwork
    }

    fn ticker(&self) -> &'static str {
        "pirate-chain"
    }

    fn has_transparent(&self) -> bool {
        false
    }

    fn has_unified(&self) -> bool {
        false
    }
}

impl CoinChain for OTHER {
    fn network(&self) -> &'static Network {
        unimplemented!()
    }

    fn ticker(&self) -> &'static str {
        unimplemented!()
    }

    fn has_transparent(&self) -> bool {
        true
    }

    fn has_unified(&self) -> bool {
        false
    }
}

pub fn get_branch(network: &Network, height: u32) -> BranchId {
    BranchId::for_height(network, BlockHeight::from_u32(height))
}

pub fn get_coin_chain(coin: u8) -> &'static (dyn CoinChain + Send) {
    match coin {
        0 => &ZCASH,
        1 => &YCASH,
        _ => &OTHER,
    }
}

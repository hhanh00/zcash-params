use zcash_primitives::consensus::{BlockHeight, BranchId, Network};

pub const NETWORK: Network = Network::YCashMainNetwork;
pub const TICKER: &str = "ycash";
pub fn get_branch(height: u32) -> BranchId {
    BranchId::for_height(&NETWORK, BlockHeight::from_u32(height))
}

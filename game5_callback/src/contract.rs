// use crate::interface::RPCiege5SwapCallbackTrait;
use soroban_sdk::{contractimpl, contracttype, Address, Env};

pub struct RPCiege5SwapCallback;

#[contracttype]
pub enum DataKey {
    LiquidityPool,
}

#[allow(unused_variables)]
#[contractimpl]
// impl RPCiege5SwapCallbackTrait for RPCiege5SwapCallback {
impl RPCiege5SwapCallback {
    fn swap_callback(
        env:Env,
        liqpool:Address,
        token_id:Address,
        amount:i128,
        initiator:Option<Address>,
    ) {
        // no
    }
    
}
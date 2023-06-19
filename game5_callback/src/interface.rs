use crate::types::Error;
use soroban_sdk::{Address, Env};

pub trait RPCiege5SwapCallbackTrait {
// pub trait LiquidityPoolTrait {

    fn deposit_liquidity(
        env: Env,
        addr: Address,
        desired_a: i128,
        min_a: i128,
        desired_b: i128,
        min_b: i128,
    ) -> Result<(), Error>;

    fn swap(
        env: Env,
        from: Option<Address>,
        callback: Address,
        buys_a: bool,
        amount: i128,
        _nft_dest: Address,
    ) -> Result<(), Error>;
}
use crate::types::Error;
use soroban_sdk::{Address, Env};

pub trait LiquidityPoolTrait {
    fn initialize(env: Env, token_a: Address, token_b: Address) -> Result<(), Error>;

    /// Deposits the liquidity from `addr` into the liquidity pool
    /// ab = k => reserve_a * amount_b = reserve_b * amount_a
    /// Calculates either amount_a or amount_b within the provided boundaries (desired and min)
    fn deposit_liquidity(
        env: Env,
        addr: Address,
        desired_a: i128,
        min_a: i128,
        desired_b: i128,
        min_b: i128,
    ) -> Result<(), Error>;

    /// Makes the swap happen
    /// When `buys_a` is true, it swaps `amount` of token B for \Deltay of token A
    /// When `buys_a` is false, it swaps `amount` of token A for \Deltay of token B
    /// before swap: XY = k
    /// dx is the amount in and dy the amount out
    /// after the swap: X'Y' = k => X'Y' = XY => (X + dx)(Y - dy) = XY
    /// We need to find dy => Y - dy = k / (X + dx) => dy = Y - XY / (x + dx) => dy = (YX + Ydx - XY) / (X + dx) => dy = Ydx / (X + dx)
    fn swap(
        env: Env,
        from: Option<Address>,
        callback: Address,
        buys_a: bool,
        amount: i128,
        _nft_dest: Address,
    ) -> Result<(), Error>;

    fn get_reserves(env: Env) -> Result<(i128, i128), Error>;
}
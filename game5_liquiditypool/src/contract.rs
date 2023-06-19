use crate::{
    callback,
    interface::LiquidityPoolTrait,
    storage::{
        has_token_a, read_reserve_a, read_reserve_b, read_token_a, read_token_b, write_reserve_a,
        write_reserve_b, write_token_a, write_token_b,
    },
    EVENT_MSG,
};
use soroban_sdk::{contractimpl, token, Address, BytesN, Env, Symbol};

use crate::types::Error;

pub struct LiquidityPool;

#[contractimpl]
impl LiquidityPoolTrait for LiquidityPool {
    fn initialize(env: Env, token_a: Address, token_b: Address) -> Result<(), Error> {
        if has_token_a(&env) {
            return Err(Error::AlreadyInitialized);
        }

        write_token_a(&env, token_a);
        write_token_b(&env, token_b);
        Ok(())
    }

    fn deposit_liquidity(
        env: Env,
        addr: Address,
        desired_a: i128,
        min_a: i128,
        desired_b: i128,
        min_b: i128,
    ) -> Result<(), Error> {
        if !has_token_a(&env) {
            return Err(Error::NotInitialized);
        }

        let token_a = token::Client::new(&env, &read_token_a(&env));
        let token_b = token::Client::new(&env, &read_token_b(&env));

        let reserve_a = read_reserve_a(&env);
        let reserve_b = read_reserve_b(&env);

        //  If there is no liquidity yet, deposit the desired amounts
        // `reserve_a` will be 0 only when `reserve_b` is also 0 (XY = k [lim X->0 (k/X) = \infty ; lim X->\infty (k/X) = 0])
        if reserve_a == 0 {
            // we don't want the first deposit to be less or equal than one stroop
            if desired_a <= 1 || desired_b <= 1 {
                return Err(Error::InvalidAmount);
            }

            //  tranfer the deposit into the pool and update the reserves
            token_a.transfer(&addr, &env.current_contract_address(), &desired_a);
            token_b.transfer(&addr, &env.current_contract_address(), &desired_b);

            write_reserve_a(&env, desired_a);
            write_reserve_b(&env, desired_b);

            // here emit "you've been scammed" event
            env.events().publish(
                (Symbol::short("deposit"), addr),
                BytesN::from_array(&env, &EVENT_MSG),
            );

            //  deposit in this case ends now
            return Ok(());
        }

        //  solve X / Y = X' / Y' to find the `amount_b` to deposit starting from the desired amount of a to deposit
        //  X / Y = (X + dx) / (Y + dy) => X(Y + dy) = Y(X+dx) => Xdy = Ydx => dy = Ydx / X
        let amount_b = reserve_b * desired_a / reserve_a;

        // compute the amount of token a and token be to deposit. Requires the two amounts to fit within the boundaries specified by the invoker
        // check that `amount_b` doesn't exceed the desired amount of b
        let (in_a, in_b) = if amount_b <= desired_b {
            // check that amount_b isn't less than the minimum amount chosen for b
            if amount_b < min_b {
                return Err(Error::InvalidAmount);
            }

            (desired_a, amount_b)
        }
        //  if `amount_b` exceeds the desired amount, calculate amount_a (dx) starting from the desired amount of a
        else {
            let amount_a = reserve_a * desired_b / reserve_b;

            // if the computed amount still doesn't fit within the boundaries specified by the invoker return an error
            if amount_a > desired_a || amount_a < min_a {
                return Err(Error::InvalidAmount);
            }

            (amount_a, desired_b)
        };

        // transfer the funds into the LP and update the reserves
        token_a.transfer(&addr, &env.current_contract_address(), &in_a);
        token_b.transfer(&addr, &env.current_contract_address(), &in_b);

        write_reserve_a(&env, reserve_a + in_a);
        write_reserve_b(&env, reserve_b + in_b);

        // here emit "you've been scammed" event
        env.events().publish(
            (Symbol::short("deposit"), addr),
            BytesN::from_array(&env, &EVENT_MSG),
        );

        Ok(())
    }

    fn swap(
        env: Env,
        from: Option<Address>,
        callback: Address,
        buys_a: bool,
        amount: i128,
        _nft_dest: Address,
    ) -> Result<(), Error> {
        if !has_token_a(&env) {
            return Err(Error::NotInitialized);
        }

        // if the initiator wants to specify an initiator, require auth for it.
        if let Some(addr) = &from {
            addr.require_auth();
        }

        // Use the `buys_a` bool to sort which are the tokens that are being received/sent out with their respective reserves
        let (token_in, token_out, reserve_in, reserve_out) = if buys_a {
            (
                read_token_b(&env),
                read_token_a(&env),
                read_reserve_b(&env),
                read_reserve_a(&env),
            )
        } else {
            (
                read_token_a(&env),
                read_token_b(&env),
                read_reserve_a(&env),
                read_reserve_b(&env),
            )
        };

        let token_in_client = token::Client::new(&env, &token_in);
        let token_out_client = token::Client::new(&env, &token_out);

        // save in memory the balance before calling the callback
        let in_before = token_in_client.balance(&env.current_contract_address());

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        // fee of 0.3%
        let amount_in_with_fee = (amount * 997) / 1000;
        let amount_out = (reserve_out * amount_in_with_fee) / (reserve_in + amount_in_with_fee);

        // call the callback function on the callback, for the swap to be successful the callback should send the specified `amount` of `token_in` to the liquidity pool
        let callback_client = callback::Client::new(&env, &callback);
        callback_client.swap_callback(&env.current_contract_address(), &token_in, &amount, &from);

        // we check that at least the correct amount was sent to the liquidity pool, if not we return the liquidity pool's `InNotSent` error.
        if in_before + amount < token_in_client.balance(&env.current_contract_address()) {
            return Err(Error::InNotSent);
        } // the bug is in this check, which asserts that no more than amount was deposited, while it should check that at least amount was deposited. Solidity-wise, like in Uniswap, it was require(in_before + amount >= token_client.balance(), "ERR"), which in Rust becomes in_before + amount < token_in_client.balance(). We could also change it to something like `if in_before + amount >= token_client.balance() { token_out.transfer() } else { Error }` to have it look more like the Uniswap bug.
          // the fix would be to reverse the operator, so `if in_before + amount > token_in_client.balance() { Error }`, which would assert that no less than amount was deposited. Alternatively in Soroban Rust, the easiest way **in this case** (where you want the user to be the one making the transfer since the contract directly interfaces with them, so wanting the balance to just increase is not the only requirement) would be to catch the transfer error (`if try_transfer().is_err() { Error }`)

        // we make the swap happen
        token_out_client.transfer(&env.current_contract_address(), &callback, &amount_out);

        // update the reserves
        if buys_a {
            write_reserve_a(
                &env,
                token_out_client.balance(&env.current_contract_address()),
            );
            write_reserve_b(
                &env,
                token_in_client.balance(&env.current_contract_address()),
            );
        } else {
            write_reserve_b(
                &env,
                token_out_client.balance(&env.current_contract_address()),
            );
            write_reserve_a(
                &env,
                token_in_client.balance(&env.current_contract_address()),
            );
        }

        Ok(())
    }

    fn get_reserves(env: Env) -> Result<(i128, i128), Error> {
        if !has_token_a(&env) {
            return Err(Error::NotInitialized);
        }

        Ok((read_reserve_a(&env), read_reserve_b(&env)))
    }
}
// #![no_std]

// use soroban_sdk::{contractimpl, Address, Env ,String};

// pub struct Contract;

// #[contractimpl]
// impl Contract {
//     pub fn game_1(env: Env, _nft_dest: Address) -> String{

//         let s = String::from_slice(&env, "1694-1727");
//         s
//     }
// }

// ------------------------------------------------------------
#![no_std]
use soroban_sdk::{
    contracterror, contractimpl, panic_with_error, xdr::ToXdr, Address, Env, Symbol,
};
pub struct Contract;
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    MissingPew = 1,
    UsedPew = 2,
}
#[contractimpl]
impl Contract {
    pub fn game_3(env: Env,  _nft_dest: Address) -> Result<(), Error> {
        let symbol = Symbol::new(&env, "rea219409");

        if env.storage().has(&symbol) {
            panic_with_error!(env, Error::UsedPew);
        }
        let bytes = symbol.clone().to_xdr(&env);
        let hash = env.crypto().sha256(&bytes);
        let mut i = 0;
        let mut has_pew = false;
        for v in hash.clone().into_iter() {
            if v == 112
                && hash.get(i + 1).unwrap_or(0) == 101
                && hash.get(i + 2).unwrap_or(0) == 119
            {
                has_pew = true;
            }
            i += 1;
        }
        if !has_pew {
            panic_with_error!(env, Error::MissingPew);
        } else {
            env.storage().set(&symbol, &true);
        }
        Ok(())
    }
}

#[cfg(test)]
mod test;
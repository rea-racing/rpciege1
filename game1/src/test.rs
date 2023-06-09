#![cfg(test)]

use std::println;

// use super::{Contract, ContractClient, Address};
use alloc::string::ToString;
use soroban_sdk::{Env, Symbol, xdr::ToXdr};

extern crate alloc;
extern crate std;

// #[test]
// fn test() {
//     let env = Env::default();
//     let contract_id = env.register_contract(None, Contract);
//     let client = ContractClient::new(&env, &contract_id);

//     let nft_dest = Address::random(&env);

// }

#[test]
#[ignore]
fn generate_pew_symbol() {
    let env = Env::default();
        env.budget().reset_unlimited();

    let mut iters: i32 = 0;
    let mut has_pew = false;
    let mut symbol = Symbol::new(&env,"");

    while !has_pew {
        symbol = Symbol::new(&env, iters.clone().to_string().as_str());

        let bytes = symbol.clone().to_xdr(&env);
        let hash = env.crypto().sha256(&bytes);
        let mut i = 0;

        for v in hash.clone().into_iter() {
            if v == 112
                && hash.get(i + 1).unwrap_or(0) == 101
                && hash.get(i + 2).unwrap_or(0) == 119
            {
                has_pew = true;

                if has_pew {
                    println!("has_pew: {:?}", has_pew);
                    println!("{:?}", bytes);
                    println!("{:?}", hash);
                }
            }
            i += 1;
        }

        // if iters % 10000 == 0 {
        //     // println!("{:?}", &hash);
        //     println!("{:?}", &iters);
        // }

        // if iters >= 100000 {
        //     break;
        // }

        iters += 1;
    }

    println!("has_pew: {:?}", has_pew);
    println!("{:?}", symbol);
    
}

#[test]
#[ignore]
fn generate_pew_symbol_2() {
    let env = Env::default();
        env.budget().reset_unlimited();

    let symbol = Symbol::new(&env,"1702871");

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
            println!("{:?}", bytes);
            println!("{:?}", hash);
        }
        i += 1;
    }
    println!("has_pew: {:?}", has_pew);
}
use soroban_sdk::{contracterror, contracttype};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 0,
    NotInitialized = 1,
    InvalidAmount = 2,
    InNotSent = 3,
    InvalidCallback = 4,
}

#[contracttype]
pub enum DataKey {
    TokenA,
    TokenB,
    ReserveA,
    ReserveB,
}
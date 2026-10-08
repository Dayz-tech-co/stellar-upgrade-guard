#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype};

#[contracttype]
pub struct Account {
    pub id: u64,
}

#[contract]
pub struct FixtureContract;

#[contractimpl]
impl FixtureContract {
    pub fn id(account: Account) -> u64 {
        account.id
    }
}

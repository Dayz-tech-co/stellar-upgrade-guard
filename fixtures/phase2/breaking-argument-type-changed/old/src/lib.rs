#![no_std]

use soroban_sdk::{contract, contractimpl};

#[contract]
pub struct FixtureContract;

#[contractimpl]
impl FixtureContract {
    pub fn set(value: u32) {
        let _ = value;
    }
}

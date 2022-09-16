use std::sync::Arc;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId, near_bindgen, Balance, PanicOnDefault, PromiseOrValue, Promise, Timestamp, BorshStorageKey, ext_contract, PromiseResult, Gas};
use near_sdk::collections::{LookupMap, Vector};

mod command;
use command::*;

pub type CommandId = String;
pub type NameProduct = String;
pub const TRANSFER_GAS: Gas = Gas(10_000_000_000_000);

impl Ord for Command {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.get_price_per_product()).cmp(&other.get_price_per_product())
    }
}

impl PartialOrd for Command {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Command {
    fn eq(&self, other: &Self) -> bool {
        self.get_price_per_product() == other.get_price_per_product()
    }
}

impl Eq for Command { }

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[near_bindgen]
struct Contract {
    owned_id: AccountId,
    sell_command: LookupMap<CommandId, Command>,
    buy_command: LookupMap<CommandId, Command>,
    cheapest_sell: BinaryHeap<Command>,
    most_expensive_buy: BinaryHeap<Command>,
}


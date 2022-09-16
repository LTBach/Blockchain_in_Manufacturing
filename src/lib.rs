use std::sync::Arc;
use std::cmp::Ordering;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId, near_bindgen, Balance, PanicOnDefault, PromiseOrValue, Promise, Timestamp, BorshStorageKey, ext_contract, PromiseResult, Gas};
use near_sdk::collections::{LookupMap, Vector, TreeMap};

mod command;
use command::*;
mod key_for_map;
use key_for_map::*;

pub type CommandId = String;
pub type NameProduct = String;
pub const TRANSFER_GAS: Gas = Gas(10_000_000_000_000);

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[near_bindgen]
struct Contract {
    owner_id: AccountId,
    sell_command: LookupMap<CommandId, Command>,
    buy_command: LookupMap<CommandId, Command>,
    cheapest_sell: TreeMap<KeyForMap, Command>,
    most_expensive_buy: TreeMap<KeyForMap, Command>,
}

#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
enum StorageKey{
    SellKey,
    BuyKey,
    CheapKey,
    ExpensiveKey,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            sell_command: LookupMap::new(StorageKey::SellKey),
            buy_command: LookupMap::new(StorageKey::BuyKey),
            cheapest_sell: TreeMap::new(StorageKey::CheapKey),
            most_expensive_buy: TreeMap::new(StorageKey::ExpensiveKey),
        }
    }
    
}


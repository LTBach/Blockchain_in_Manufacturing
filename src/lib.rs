use std::sync::Arc;

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

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[near_bindgen]
struct Contract {
    owned_id: AccountId,
    sell_command: LookupMap<CommandId, Command>,
    buy_command: LookupMap<CommandId, Command>,
    cheapest_sell: LookupMap<NameProduct, Command>,
}

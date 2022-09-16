use std::sync::Arc;
use std::cmp::Ordering;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId, near_bindgen, Balance, PanicOnDefault, PromiseOrValue, Promise, Timestamp, BorshStorageKey, ext_contract, PromiseResult, Gas};
use near_sdk::collections::{LookupMap, Vector, TreeMap};

mod command;
use command::{*,quality::*};
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
    pub fn add_commnand(&mut self, command_id: CommandId, name_product: NameProduct, is_sell: bool, 
        amount_product: U128, price_per_product: U128, quality: Option<Quality>) {
        let mut amount_product_mut = amount_product;
        // assert!(env::)
        if is_sell {
            while self.most_expensive_buy.len() != 0 && amount_product_mut.0 != 0{
                let mut highest_buy = self.most_expensive_buy.get(&self.most_expensive_buy.max().expect("CAN NOT BUG 1")).expect("CAN NOT BUG 2");
                if highest_buy.get_price_per_product() >= price_per_product {
                    let amount_product_highest_buy = highest_buy.get_amount_product();
                    if amount_product_mut < amount_product_highest_buy {
                        //Thanh toan
                        highest_buy.set_amount_product(U128(amount_product_highest_buy.0 - amount_product_mut.0));
                        amount_product_mut = U128(0);
                        self.buy_command.insert(&highest_buy.get_command_id(), &highest_buy);
                        self.most_expensive_buy.insert(&highest_buy.get_key_for_map(),&highest_buy);
                        break;
                    } else {
                        //Thanh toan
                        amount_product_mut = U128(amount_product_mut.0 - amount_product_highest_buy.0);
                        self.most_expensive_buy.remove(&highest_buy.get_key_for_map());
                        self.buy_command.remove(&highest_buy.get_command_id());
                    }
                }
            }
            if amount_product_mut.0 != 0 {
                let command = Command::new(command_id.clone(), name_product, is_sell,
                    amount_product_mut, price_per_product, quality);
                self.sell_command.insert(&command_id, &command);
            }
        } else {
            while self.cheapest_sell.len() != 0 && amount_product_mut.0 != 0{
                let mut lowest_sell = self.cheapest_sell.get(&self.cheapest_sell.min().expect("CAN NOT BUG 3")).expect("CAN NOT BUG 4");
                if lowest_sell.get_price_per_product() <= price_per_product {
                    let amount_product_lowest_sell = lowest_sell.get_amount_product();
                    if amount_product_mut < amount_product_lowest_sell {
                        //Thanh toan
                        lowest_sell.set_amount_product(U128(amount_product_lowest_sell.0 - amount_product_mut.0));
                        amount_product_mut = U128(0);
                        self.sell_command.insert(&lowest_sell.get_command_id(), &lowest_sell);
                        self.cheapest_sell.insert(&lowest_sell.get_key_for_map(),&lowest_sell);
                        break;
                    } else {
                        //Thanh toan
                        amount_product_mut = U128(amount_product_mut.0 - amount_product_lowest_sell.0);
                        self.cheapest_sell.remove(&lowest_sell.get_key_for_map());
                        self.sell_command.remove(&lowest_sell.get_command_id());
                    }
                }
            }
            if amount_product_mut.0 != 0 {
                let command = Command::new(command_id.clone(), name_product, is_sell,
                    amount_product_mut, price_per_product, quality);
                self.sell_command.insert(&command_id, &command);
            }
        }
    }
}


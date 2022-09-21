use std::cmp::Ordering;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::signer_account_id;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId, near_bindgen, PanicOnDefault, Promise, Timestamp, BorshStorageKey, Gas};
use near_sdk::collections::{LookupMap, TreeMap};

mod command;
use command::{*,quality::*};
mod key_for_tree;
use key_for_tree::*;

pub type CommandId = String;
pub type NameProduct = String;
pub const TRANSFER_GAS: Gas = Gas(10_000_000_000_000);

#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[near_bindgen]
struct Contract {
    owner_id: AccountId,
    commands: LookupMap<CommandId, Command>,
    ordered_sell: LookupMap<NameProduct, TreeMap<KeyForTree, Command>>,
    ordered_buy: LookupMap<NameProduct, TreeMap<KeyForTree, Command>>,
    number_of_item_in_sell: i64,
    number_of_item_in_buy: i64,
}

#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
enum StorageKey{
    CommandKey,
    OrderedSellKey,
    OrderedBuyKey,
    TreeBuyKey(i64),
    TreeSellKey(i64),
}

#[near_bindgen]
#[allow(dead_code)]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        // assert_eq!(env::signer_account_id(), owner_id, "WRONG ACCOUNT");
        Self {
            owner_id,
            commands: LookupMap::new(StorageKey::CommandKey),
            ordered_sell: LookupMap::new(StorageKey::OrderedSellKey),
            ordered_buy: LookupMap::new(StorageKey::OrderedBuyKey),
            number_of_item_in_sell: 0,
            number_of_item_in_buy: 0,
        }
    }
    #[payable]

    pub fn add_command(&mut self, command_id: CommandId, name_product: NameProduct, is_sell: bool, 
        amount_product: U128, price_per_product: U128, quality: Option<Quality>) {
        let mut amount_product_mut = amount_product;
        if is_sell {
            assert!(env::attached_deposit() == 0, "SELL COMMAND NOT USE DEPOSIT");
            let mut amount_seller_reiceive: u128 = 0;
            match self.ordered_buy.get(&name_product.clone()) {
                Some(mut treemap) => {
                    while amount_product_mut.0 != 0 {
                        match treemap.max() {
                            Some(highest_buy_key_for_map) => {
                                let mut highest_buy = treemap.get(&highest_buy_key_for_map).expect("CAN NOT BUG 1");
                                if highest_buy.get_price_per_product() >= price_per_product {
                                    let amount_product_highest_buy = highest_buy.get_amount_product();
                                    let price_per_product_highest_buy = highest_buy.get_price_per_product();
                                    if amount_product_mut < amount_product_highest_buy {
                                        amount_seller_reiceive = amount_seller_reiceive + 
                                                                 amount_product_mut.0 * price_per_product_highest_buy.0;
                                        highest_buy.set_amount_product(U128(amount_product_highest_buy.0 - amount_product_mut.0));
                                        amount_product_mut = U128(0);
                                        self.commands.insert(&highest_buy.get_command_id(), &highest_buy);
                                        treemap.insert(&highest_buy.get_key_for_map(),&highest_buy);
                                        self.ordered_buy.insert(&name_product, &treemap);
                                        break;
                                    } else {
                                        amount_seller_reiceive = amount_seller_reiceive + 
                                                                 amount_product_highest_buy.0 * price_per_product_highest_buy.0;
                                        amount_product_mut = U128(amount_product_mut.0 - amount_product_highest_buy.0);
                                        treemap.remove(&highest_buy.get_key_for_map());
                                        self.commands.remove(&highest_buy.get_command_id());
                                    }
                                } else {
                                    break;
                                }
                            }
                            None => {
                                break;
                            }
                        }
                    }
                }
                None => {},
            }
            if amount_seller_reiceive != 0 {
                Promise::new(signer_account_id())
                .transfer(amount_seller_reiceive);
            }
            if amount_product_mut.0 != 0 {
                let command = Command::new(command_id.clone(), name_product.clone(), is_sell,
                    amount_product_mut, price_per_product, quality);
                self.commands.insert(&command_id, &command);
                match self.ordered_sell.get(&name_product) {
                    Some(mut treemap) => {
                        treemap.insert(&KeyForTree::new(price_per_product, command_id), &command);
                    }
                    None => {
                        let mut treemap = TreeMap::new(StorageKey::TreeSellKey(self.number_of_item_in_sell));
                        self.number_of_item_in_sell = self.number_of_item_in_sell + 1;
                        treemap.insert(&KeyForTree::new(price_per_product, command_id), &command);
                        self.ordered_sell.insert(&name_product, &treemap);
                    }
                }
            }
        } else {
            assert!(env::attached_deposit() >= price_per_product.0 * amount_product.0,
            "BUY COMMAND HAS NOT ENGOUGH DEPOSIT.IT NEED AT LEAST {}.YOU HAVE {}",price_per_product.0 * amount_product.0,env::attached_deposit());
            let mut amount_buyer_exceed: u128 = 0;
            if env::attached_deposit() > price_per_product.0 * amount_product.0 {
                Promise::new(signer_account_id()).transfer(env::attached_deposit() - price_per_product.0 * amount_product.0);
            }
            match self.ordered_sell.get(&name_product) {
                Some(mut treemap) => {
                    while amount_product_mut.0 != 0 {
                        match treemap.min() {
                            Some(lowest_sell_key_for_map) => {
                                let mut lowest_sell = treemap.get(&lowest_sell_key_for_map).expect("CAN NOT BUG 2");
                                if lowest_sell.get_price_per_product() <= price_per_product {
                                    let amount_product_lowest_sell = lowest_sell.get_amount_product();
                                    let price_per_product_lowest_sell = lowest_sell.get_price_per_product();
                                    if amount_product_mut < amount_product_lowest_sell {
                                        amount_buyer_exceed = amount_buyer_exceed +
                                                              amount_product_mut.0 * (price_per_product.0 - price_per_product_lowest_sell.0);
                                        Promise::new(lowest_sell.get_command_owner_id())
                                        .transfer(amount_product_mut.0 * price_per_product_lowest_sell.0);
                                        lowest_sell.set_amount_product(U128(amount_product_lowest_sell.0 - amount_product_mut.0));
                                        amount_product_mut = U128(0);
                                        self.commands.insert(&lowest_sell.get_command_id(), &lowest_sell);
                                        treemap.insert(&lowest_sell.get_key_for_map(),&lowest_sell);
                                        self.ordered_sell.insert(&name_product, &treemap);
                                        break;
                                    } else {
                                        amount_buyer_exceed = amount_buyer_exceed +
                                                              amount_product_lowest_sell.0 * (price_per_product.0 - price_per_product_lowest_sell.0);
                                        Promise::new(lowest_sell.get_command_owner_id())
                                        .transfer(amount_product_lowest_sell.0 * price_per_product_lowest_sell.0);
                                        amount_product_mut = U128(amount_product_mut.0 - amount_product_lowest_sell.0);
                                        treemap.remove(&lowest_sell.get_key_for_map());
                                        self.commands.remove(&lowest_sell.get_command_id());
                                    }
                                } else {
                                    break;
                                }
                            }
                            None => {
                                break;
                            }
                        }
                    }
                }
                None => {},
            }
            if amount_buyer_exceed != 0 {
                Promise::new(signer_account_id())
                .transfer(amount_buyer_exceed);
            }
            if amount_product_mut.0 != 0 {
                let command = Command::new(command_id.clone(), name_product.clone(), is_sell,
                    amount_product_mut, price_per_product, quality);
                self.commands.insert(&command_id, &command);
                match self.ordered_buy.get(&name_product) {
                    Some(mut treemap) => {
                        treemap.insert(&KeyForTree::new(price_per_product, command_id), &command);
                    }
                    None => {
                        let mut treemap = TreeMap::new(StorageKey::TreeBuyKey(self.number_of_item_in_buy));
                        self.number_of_item_in_buy = self.number_of_item_in_buy + 1;
                        treemap.insert(&KeyForTree::new(price_per_product, command_id), &command);
                        self.ordered_buy.insert(&name_product, &treemap);
                    }
                }
            }
        }
    }
    pub fn get_product_order_way(&self, name_product: NameProduct, is_sell: bool) -> Vec<Command> {
        if is_sell {
            match self.ordered_sell.get(&name_product) {
                Some(treemap) => {
                    let mut ans = Vec::new();
                    for (_a, b) in treemap.iter() {
                        ans.push(b);
                    }
                    ans
                },
                None => {
                    Vec::new()
                }
            }
        } else {
            match self.ordered_buy.get(&name_product) {
                Some(treemap) => {
                    let mut ans = Vec::new();
                    for (_a, b) in treemap.iter_rev() {
                        ans.push(b);
                    }
                    ans
                },
                None => {
                    Vec::new()
                }
            }
        }
    }
    pub fn remove_command(&mut self, command_id: CommandId) {
        let command = self.commands.get(&command_id).expect("ERROR COMMAND NOT FOUND");
        assert_eq!(env::signer_account_id(), command.get_command_owner_id(), "ERROR YOU ARE NOT OWNER OF COMMAND");
        if !command.get_is_sell() {
            Promise::new(env::signer_account_id())
            .transfer(command.get_amount_product().0 * command.get_price_per_product().0);
        }
        self.commands.remove(&command_id);
    }
    pub fn get_command(&self, command_id: CommandId) -> Command{
        self.commands.get(&command_id).expect("ERROR COMMAND NOT FOUND")
    }
    pub fn none(&self) {
    }
}
#[allow(unused_imports)]
#[cfg(all(test, not(target_arch = "wasm32")))]
mod test {
    use super::*;
    use near_sdk::serde::ser::SerializeTupleStruct;
    use near_sdk::test_utils::{VMContextBuilder, accounts};
    use near_sdk::{testing_env, MockedBlockchain, Balance};

    const CONTRACT_ACCOUNT: &str = "manufacturing.uitdev.testnet";
    
    fn set_context(predecessor: &str, balance: Balance, deposit: Balance) {
        let mut builder = VMContextBuilder::new();
        builder.current_account_id(predecessor.parse().unwrap())
            .signer_account_id(predecessor.parse().unwrap())
            .predecessor_account_id(predecessor.parse().unwrap())
            .account_balance(balance)
            .attached_deposit(deposit);
        testing_env!(builder.build());
    }

    #[test]
    fn test_add_buy_command() {
        let mut contract = Contract::new(CONTRACT_ACCOUNT.parse().unwrap());

        set_context("buy_account", 1000, 1000);

        contract.add_command("command_1".to_owned(), "Iphone_14".to_owned()
                             , false, U128(2), U128(500), None);

        let test_command = contract.get_command("command_1".to_owned());

        //test
        assert_eq!(test_command.get_command_id(), "command_1".to_owned());
        assert_eq!(test_command.get_name_product(), "Iphone_14".to_owned());
        assert_eq!(test_command.get_is_sell(), false);
        assert_eq!(test_command.get_amount_product(), U128(2));
        assert_eq!(test_command.get_price_per_product(), U128(500));
        assert_eq!(test_command.get_command_owner_id(), "buy_account".parse().unwrap());
    }

    #[test]
    fn test_add_sell_command() {
        let mut contract = Contract::new(CONTRACT_ACCOUNT.parse().unwrap());

        set_context("sell_account", 0, 0);

        contract.add_command("command_1".to_owned(), "Iphone_14".to_owned()
                             , true, U128(2), U128(500), None);

        let test_command = contract.get_command("command_1".to_owned());

        //test
        assert_eq!(test_command.get_command_id(), "command_1".to_owned());
        assert_eq!(test_command.get_name_product(), "Iphone_14".to_owned());
        assert_eq!(test_command.get_is_sell(), true);
        assert_eq!(test_command.get_amount_product(), U128(2));
        assert_eq!(test_command.get_price_per_product(), U128(500));
        assert_eq!(test_command.get_command_owner_id(), "sell_account".parse().unwrap());
    }

    #[test]
    #[should_panic(expected = "BUY COMMAND HAS NOT ENGOUGH DEPOSIT.IT NEED AT LEAST 1000.YOU HAVE 999")]
    fn test_add_buy_command_with_lack_attached_deposit() {
        let mut contract = Contract::new(CONTRACT_ACCOUNT.parse().unwrap());

        set_context("buy_account", 1000, 999);

        contract.add_command("command_1".to_owned(), "Iphone_14".to_owned()
                             , false, U128(2), U128(500), None);

        let test_command = contract.get_command("command_1".to_owned());

        //test
        assert_eq!(test_command.get_command_id(), "command_1".to_owned());
        assert_eq!(test_command.get_name_product(), "Iphone_14".to_owned());
        assert_eq!(test_command.get_is_sell(), false);
        assert_eq!(test_command.get_amount_product(), U128(2));
        assert_eq!(test_command.get_price_per_product(), U128(500));
        assert_eq!(test_command.get_command_owner_id(), "buy_account".parse().unwrap());
    }

    #[test]
    #[should_panic(expected = "SELL COMMAND NOT USE DEPOSIT")]
    fn test_add_sell_command_with_exceed_attached_deposit() {
        let mut contract = Contract::new(CONTRACT_ACCOUNT.parse().unwrap());

        set_context("sell_account", 1000, 1000);

        contract.add_command("command_1".to_owned(), "Iphone_14".to_owned()
                             , true, U128(2), U128(500), None);

        let test_command = contract.get_command("command_1".to_owned());

        //test
        assert_eq!(test_command.get_command_id(), "command_1".to_owned());
        assert_eq!(test_command.get_name_product(), "Iphone_14".to_owned());
        assert_eq!(test_command.get_is_sell(), true);
        assert_eq!(test_command.get_amount_product(), U128(2));
        assert_eq!(test_command.get_price_per_product(), U128(500));
        assert_eq!(test_command.get_command_owner_id(), "sell_account".parse().unwrap());
    }
}
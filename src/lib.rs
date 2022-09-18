// use std::sync::Arc;
use std::cmp::Ordering;
// use std::thread::{__FastLocalKeyInner, __OsLocalKeyInner};

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
    orderd_sell: LookupMap<NameProduct, TreeMap<KeyForTree, Command>>,
    orderd_buy: LookupMap<NameProduct, TreeMap<KeyForTree, Command>>,
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
        Self {
            owner_id,
            commands: LookupMap::new(StorageKey::CommandKey),
            orderd_sell: LookupMap::new(StorageKey::OrderedSellKey),
            orderd_buy: LookupMap::new(StorageKey::OrderedBuyKey),
            number_of_item_in_sell: 0,
            number_of_item_in_buy: 0,
        }
    }
    #[payable]

    pub fn add_commnand(&mut self, command_id: CommandId, name_product: NameProduct, is_sell: bool, 
        amount_product: U128, price_per_product: U128, quality: Option<Quality>) {
        let mut amount_product_mut = amount_product;
        if is_sell {
            assert!(env::attached_deposit() == 0, "SELL COMMAND NOT USE DEPOSIT");
            let mut amount_seller_reiceive: u128 = 0;
            match self.orderd_buy.get(&name_product.clone()) {
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
                                        self.orderd_buy.insert(&name_product, &treemap);
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
                match self.orderd_sell.get(&name_product) {
                    Some(mut treemap) => {
                        treemap.insert(&KeyForTree::new(price_per_product, command_id), &command);
                    }
                    None => {
                        let mut treemap = TreeMap::new(StorageKey::TreeSellKey(self.number_of_item_in_sell));
                        self.number_of_item_in_sell = self.number_of_item_in_sell + 1;
                        treemap.insert(&KeyForTree::new(price_per_product, command_id), &command);
                        self.orderd_sell.insert(&name_product, &treemap);
                    }
                }
            }
        } else {
            assert!(env::attached_deposit() >= price_per_product.0 * amount_product.0,"BUY COMMAND HAS NOT ENGOUGH DEPOSIT");
            let mut amount_buyer_exceed: u128 = 0;
            if env::attached_deposit() > price_per_product.0 * amount_product.0 {
                Promise::new(signer_account_id()).transfer(price_per_product.0 * amount_product.0 - env::attached_deposit());
            }
            match self.orderd_sell.get(&name_product) {
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
                                        self.orderd_sell.insert(&name_product, &treemap);
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
                match self.orderd_buy.get(&name_product) {
                    Some(mut treemap) => {
                        treemap.insert(&KeyForTree::new(price_per_product, command_id), &command);
                    }
                    None => {
                        let mut treemap = TreeMap::new(StorageKey::TreeBuyKey(self.number_of_item_in_buy));
                        self.number_of_item_in_buy = self.number_of_item_in_buy + 1;
                        treemap.insert(&KeyForTree::new(price_per_product, command_id), &command);
                        self.orderd_buy.insert(&name_product, &treemap);
                    }
                }
            }
        }
    }
    pub fn get_product_order_way(&self, name_product: NameProduct, is_sell: bool) -> Vec<Command> {
        if is_sell {
            match self.orderd_sell.get(&name_product) {
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
            match self.orderd_buy.get(&name_product) {
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
}
#[allow(unused_imports)]
#[cfg(all(test, not(target_arch = "wasm32")))]
mod test {
    use super::*;
    use near_sdk::test_utils::{VMContextBuilder, accounts};
    use near_sdk::{testing_env, MockedBlockchain};

    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.current_account_id(accounts(0))
            .signer_account_id(accounts(0))
            .predecessor_account_id(accounts(0))
            .is_view(is_view);
        builder
    }

    #[test]
    fn test_add_buy_command() {
        let mut context = get_context(false);
        let contract_account = accounts(0);
        
        context.account_balance(1000)
            .predecessor_account_id(contract_account.clone())
            .attached_deposit(1000)
            .signer_account_id(contract_account.clone());
        testing_env!(context.build());

        let mut context = get_context(false);
        let buyer_1 = accounts(1);
        
        context.account_balance(1000)
            .predecessor_account_id(buyer_1.clone())
            .attached_deposit(1000)
            .signer_account_id(buyer_1.clone());
        testing_env!(context.build());

        let mut contract = Contract::new(contract_account.clone());
        let amount_product = U128(2);
        let price_per_product = U128(500);
        let quality = None;
        let is_sell = false;

        contract.add_commnand("command_1".to_owned(), "Iphone_14".to_owned()
                             , is_sell, amount_product, price_per_product, quality);

        let test_command = contract.get_command("command_1".to_owned());

        //test
        assert_eq!(test_command.get_command_id(), "command_1".to_owned());
        assert_eq!(test_command.get_name_product(), "Iphone_14".to_owned());
        assert_eq!(test_command.get_is_sell(), false);
        assert_eq!(test_command.get_amount_product(), U128(2));
        assert_eq!(test_command.get_price_per_product(), U128(500));
        assert_eq!(test_command.get_command_owner_id(), buyer_1);
    }
    #[test]
    #[should_panic(expected = "BUY COMMAND HAS NOT ENGOUGH DEPOSIT")]
    fn test_add_buy_command_with_lack_attached_deposit() {
        let mut context = get_context(false);
        let contract_account = accounts(0);
        
        context.account_balance(1000)
            .predecessor_account_id(contract_account.clone())
            .attached_deposit(1000)
            .signer_account_id(contract_account.clone());
        testing_env!(context.build());

        let mut context = get_context(false);
        let buyer_2 = accounts(1);
        
        context.account_balance(999)
            .predecessor_account_id(buyer_2.clone())
            .attached_deposit(999)
            .signer_account_id(buyer_2.clone());
        testing_env!(context.build());

        let mut contract = Contract::new(contract_account.clone());
        let amount_product = U128(2);
        let price_per_product = U128(500);
        let quality = None;
        let is_sell = false;

        contract.add_commnand("command_2".to_owned(), "Iphone_14".to_owned()
                            , is_sell, amount_product, price_per_product, quality);

        let test_command = contract.get_command("command_2".to_owned());

        //test
        assert_eq!(test_command.get_command_id(), "command_2".to_owned());
        assert_eq!(test_command.get_name_product(), "Iphone_14".to_owned());
        assert_eq!(test_command.get_is_sell(), false);
        assert_eq!(test_command.get_amount_product(), U128(2));
        assert_eq!(test_command.get_price_per_product(), U128(500));
        assert_eq!(test_command.get_command_owner_id(), buyer_2);
    }
}

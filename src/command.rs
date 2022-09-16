use crate::*;

mod quality;
use quality::*;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Command {
    command_id: CommandId,
    name_product: NameProduct,
    is_sell: bool,
    amount_product: U128,
    reiceived_amount: Balance,
    price_per_product: U128,
    quality: Option<Quality>,
    create_at: Timestamp,
    command_owner_id: AccountId,
}

impl Command {
    pub fn new(command_id: CommandId,name_product: NameProduct,is_sell: bool
        ,amount_product: U128,price_per_product: U128, quality: Option<Quality>) -> Self {
        Self {
            command_id,
            name_product,
            is_sell,
            amount_product,
            reiceived_amount: env::attached_deposit(),
            price_per_product,
            quality,
            create_at: env::block_timestamp(),
            command_owner_id: env::signer_account_id(),
        }
    }
    pub fn get_price_per_product(&self) -> U128{
        self.price_per_product
    }
    pub fn get_command_id(&self) -> CommandId {
        self.command_id.clone()
    }
}
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
    price_per_product: U128,
    quality: Option<Quality>,
    create_at: Timestamp,
    command_owner_id: AccountId,
}

impl Command {
    pub fn get_price_per_product(&self) -> U128{
        self.price_per_product
    }
}
use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize,Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct KeyForTree {
    price_per_product: Balance,
    command_id: CommandId,
}
impl KeyForTree {
    pub fn new(price_per_product: Balance, command_id: CommandId) -> Self {
        Self {
            price_per_product,
            command_id,
        }
    }
    pub fn get_price_per_product(&self) -> Balance{
        self.price_per_product
    }
    pub fn get_command_id(&self) -> CommandId {
        self.command_id.clone()
    }
}
impl Ord for KeyForTree {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.get_price_per_product() == other.get_price_per_product() {
            self.get_command_id().cmp(&other.get_command_id())
        } else {
            self.get_price_per_product().cmp(&other.get_price_per_product())
        }
    }
}

impl PartialOrd for KeyForTree {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for KeyForTree {
    fn eq(&self, other: &Self) -> bool {
        self.get_price_per_product() == other.get_price_per_product()
        && self.get_command_id() == other.get_command_id()
    }
}

impl Eq for KeyForTree { }

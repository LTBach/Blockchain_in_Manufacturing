use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Quality {
    certificate: Vec<String>,
    stage: Vec<String>,
}
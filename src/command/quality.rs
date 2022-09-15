use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]

pub struct Quality {
    certificate: Vec<String>,
    stage: Vec<String>,
}
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr};
use cw_storage_plus::{Item, Map};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub signees: i32,
}

pub const CONFIG: Item<State> = Item::new("config");


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Signature {
    pub signee: CanonicalAddr,
    pub martian_date: String,
    pub martian_time: String,
}

pub const SIGNATURES: Map<&[u8], Signature> = Map::new("signatures");

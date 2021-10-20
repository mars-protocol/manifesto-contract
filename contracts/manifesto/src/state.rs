use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

use mars_community::manifesto::MedalMetaData;

pub const CONFIG: Item<Config> = Item::new("config");
pub const STATE: Item<State> = Item::new("state");
pub const METADATA: Map<&[u8], MedalMetaData> = Map::new("metadata");
pub const SIGNATURES: Map<&[u8], Signature> = Map::new("signatures");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub medal_addr: Addr,
    pub medal_redeem_addr: Addr,
    pub max_signees_allowed: u64,
    pub admin: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub signees_count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Signature {
    pub signee: Addr,
    pub martian_date: String,
    pub martian_time: String,
}

impl Default for Signature {
    fn default() -> Self {
        Signature {
            signee: Addr::unchecked("".to_string()),
            martian_date: "".to_string(),
            martian_time: "".to_string(),
        }
    }
}

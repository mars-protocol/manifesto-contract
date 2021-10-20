use crate::metadata::Metadata;
use cosmwasm_std::{Addr, Api, StdResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub medal_addr: Option<String>,
    pub medal_redeem_addr: Option<String>,
    pub max_signees_limit: u64,
    pub admin: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateAdmin {
        new_admin: String,
    },
    UpdateMedalConfig {
        medal_addr: String,
        metadata: Metadata,
    },
    UpdateMedalRedeemConfig {
        medal_redeem_addr: String,
        metadata: MedalMetaData,
    },
    SignManifesto {
        martian_date: String,
        martian_time: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    State {},
    GetSignature { signee: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MedalExecuteMsg {
    UpdateMedalRedeemConfig {
        medal_redeem_addr: String,
        metadata: MedalMetaData,
    },
    Mint(MintMsg),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MintMsg {
    /// Unique ID of the NFT
    pub token_id: String,
    /// The owner of the newly minter NFT
    pub owner: String,
    /// Identifies the asset to which this NFT represents
    pub name: String,
    /// Describes the asset to which this NFT represents (may be empty)
    pub description: Option<String>,
    /// A URI pointing to an image representing the asset
    pub image: Option<String>,
    /// Any custom extension used by this contract
    pub extension: Metadata,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub medal_addr: Addr,
    pub max_signees_allowed: u64,
    pub admin: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StateResponse {
    pub signee_count: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SigneeResponse {
    pub is_signee: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SignatureResponse {
    pub signee: String,
    pub martian_date: String,
    pub martian_time: String,
}

/// Used when unwrapping an optional address sent in a contract call by a user.
/// Validates addreess if present, otherwise uses a given default value.
pub fn option_string_to_addr(
    api: &dyn Api,
    option_string: Option<String>,
    default: Addr,
) -> StdResult<Addr> {
    match option_string {
        Some(input_addr) => api.addr_validate(&input_addr),
        None => Ok(default),
    }
}

pub fn zero_address() -> Addr {
    Addr::unchecked("")
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MedalMetaData {
    pub name_prefix: String,
    pub description: String,
    pub image: String,
    pub token_uri: String,
}

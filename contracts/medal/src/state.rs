use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use cosmwasm_std::{Addr, BlockInfo, StdResult, Storage};

use cw721::{ContractInfoResponse, CustomMsg, Cw721, Expiration};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, MultiIndex};

pub struct Cw721Contract<'a, T, C>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub contract_info: Item<'a, ContractInfoResponse>,
    pub minter: Item<'a, Addr>,
    pub medal_redeem: Item<'a, Addr>,
    pub medal_redeem_info: Item<'a, MedalMetaData>,
    pub token_count: Item<'a, u64>,
    pub redeem_count: Item<'a, u64>,
    /// Stored as (granter, operator) giving operator full control over granter's account
    pub operators: Map<'a, (&'a Addr, &'a Addr), Expiration>,
    pub tokens: IndexedMap<'a, &'a str, TokenInfo<T>, TokenIndexes<'a, T>>,
    pub(crate) _custom_response: PhantomData<C>,
}

// This is a signal, the implementations are in other files
impl<'a, T, C> Cw721<T, C> for Cw721Contract<'a, T, C>
where
    T: Serialize + DeserializeOwned + Clone,
    C: CustomMsg,
{
}

impl<T, C> Default for Cw721Contract<'static, T, C>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn default() -> Self {
        Self::new(
            "nft_info",
            "minter",
            "medal_redeem",
            "medal_redeem_info",
            "num_tokens",
            "num_redeemed_tokens",
            "operators",
            "tokens",
            "tokens__owner",
        )
    }
}

impl<'a, T, C> Cw721Contract<'a, T, C>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn new(
        contract_key: &'a str,
        minter_key: &'a str,
        medal_redeem_key: &'a str,
        medal_redeem_info_key: &'a str,
        token_count_key: &'a str,
        redeemed_token_count_key: &'a str,
        operator_key: &'a str,
        tokens_key: &'a str,
        tokens_owner_key: &'a str,
    ) -> Self {
        let indexes = TokenIndexes {
            owner: MultiIndex::new(token_owner_idx, tokens_key, tokens_owner_key),
        };
        Self {
            contract_info: Item::new(contract_key),
            minter: Item::new(minter_key),
            medal_redeem: Item::new(medal_redeem_key),
            medal_redeem_info: Item::new(medal_redeem_info_key),
            token_count: Item::new(token_count_key),
            redeem_count: Item::new(redeemed_token_count_key),
            operators: Map::new(operator_key),
            tokens: IndexedMap::new(tokens_key, indexes),
            _custom_response: PhantomData,
        }
    }

    /// Returns the MEDAL (Redeemed) contract address
    pub fn get_medal_redeem_addr(&self, storage: &dyn Storage) -> StdResult<Addr> {
        self.medal_redeem.load(storage)
    }

    /// Updates the MEDAL (Redeemed) contract address
    pub fn update_medal_redeem_addr(
        &self,
        storage: &mut dyn Storage,
        medal_redeem_addr: Addr,
    ) -> StdResult<Addr> {
        self.medal_redeem.save(storage, &medal_redeem_addr)?;
        self.medal_redeem.load(storage)
    }

    /// Returns the MEDAL (Redeemed) Metadata
    pub fn get_medal_redeem_info(&self, storage: &dyn Storage) -> StdResult<MedalMetaData> {
        self.medal_redeem_info.load(storage)
    }

    /// Updates the MEDAL (Redeemed) Metadata
    pub fn update_medal_redeem_info(
        &self,
        storage: &mut dyn Storage,
        medal_redeem_info: MedalMetaData,
    ) -> StdResult<MedalMetaData> {
        self.medal_redeem_info.save(storage, &medal_redeem_info)?;
        self.medal_redeem_info.load(storage)
    }

    /// Returns the current count of MEDAL Tokens
    pub fn token_count(&self, storage: &dyn Storage) -> StdResult<u64> {
        Ok(self.token_count.may_load(storage)?.unwrap_or_default())
    }

    /// Increments the current count of MEDAL Tokens
    pub fn increment_tokens(&self, storage: &mut dyn Storage) -> StdResult<u64> {
        let val = self.token_count(storage)? + 1;
        self.token_count.save(storage, &val)?;
        Ok(val)
    }

    /// Decrements the current count of MEDAL Tokens
    pub fn decrement_tokens(&self, storage: &mut dyn Storage) -> StdResult<u64> {
        let val = self.token_count(storage)? - 1;
        self.token_count.save(storage, &val)?;
        Ok(val)
    }

    /// Returns the current count of MEDAL Tokens that have been redeemed
    pub fn redeemed_tokens_count(&self, storage: &dyn Storage) -> StdResult<u64> {
        Ok(self.redeem_count.may_load(storage)?.unwrap_or_default())
    }

    /// Increments the current count of MEDAL Tokens that have been redeemed
    pub fn increment_redeemed_tokens(&self, storage: &mut dyn Storage) -> StdResult<u64> {
        let val = self.redeemed_tokens_count(storage)? + 1;
        self.redeem_count.save(storage, &val)?;
        Ok(val)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MedalMetaData {
    pub name_prefix: String,
    pub description: String,
    pub image: String,
    pub token_uri: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfo<T> {
    /// The owner of the newly minted NFT
    pub owner: Addr,
    /// Approvals are stored here, as we clear them all upon transfer and cannot accumulate much
    pub approvals: Vec<Approval>,

    /// Identifies the asset to which this NFT represents
    pub name: String,
    /// Describes the asset to which this NFT represents
    pub description: String,
    /// A URI pointing to an image representing the asset
    pub image: Option<String>,

    /// You can add any custom metadata here when you extend cw721-base
    pub extension: T,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct Approval {
    /// Account that can transfer/send the token
    pub spender: Addr,
    /// When the Approval expires (maybe Expiration::never)
    pub expires: Expiration,
}

impl Approval {
    pub fn is_expired(&self, block: &BlockInfo) -> bool {
        self.expires.is_expired(block)
    }
}

pub struct TokenIndexes<'a, T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    // pk goes to second tuple element
    pub owner: MultiIndex<'a, (Addr, Vec<u8>), TokenInfo<T>>,
}

impl<'a, T> IndexList<TokenInfo<T>> for TokenIndexes<'a, T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<TokenInfo<T>>> + '_> {
        let v: Vec<&dyn Index<TokenInfo<T>>> = vec![&self.owner];
        Box::new(v.into_iter())
    }
}

pub fn token_owner_idx<T>(d: &TokenInfo<T>, k: Vec<u8>) -> (Addr, Vec<u8>) {
    (d.owner.clone(), k)
}

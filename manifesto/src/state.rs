use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Api, Storage, StdResult};
use cw_storage_plus::{Bound, Item, Map};

pub static CONFIG_KEY: &[u8] = b"config";
// pub static  SIGNATURES: &[u8] = b"signatures";
pub static  SIGNATURES_BUCKET: &[u8] = b"signatures_bucket";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub signees: i32,
}

pub const CONFIG: Item<State> = Item::new("config");

// pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
//     singleton(storage, CONFIG_KEY)
// }

// pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
//     singleton_read(storage, CONFIG_KEY)
// }


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Signature {
    pub signee: CanonicalAddr,
    pub martian_date: String,
    pub martian_time: String,
}

pub const SIGNATURES: Map<&[u8], Signature> = Map::new("signatures");

// pub fn create_signature<S: Storage>(storage: &mut S, signee_addr:String, signature:Signature ) -> StdResult<()> {
//     let mut signatures_bucket: Bucket<S, Signature> = Bucket::new(SIGNATURES_BUCKET, storage);
//     signatures_bucket.save( signee_addr.as_bytes() , &signature )?;
//     Ok(())    
// }

// pub fn read_signature<S: ReadonlyStorage>(storage: &S, signee_addr:String) -> StdResult<Signature> {
//     let signatures_bucket: ReadonlyBucket<S, Signature> = ReadonlyBucket::new(SIGNATURES_BUCKET, storage);
//     signatures_bucket.load(signee_addr.as_bytes())
// }


pub fn store_signee<S: Storage>(storage: &mut S) -> Bucket<S, bool> { 
    bucket(SIGNATURES, storage)
}

pub fn read_signee<S: Storage>(storage: &S) -> ReadonlyBucket<S, bool> { 
    bucket_read(SIGNATURES, storage)
}
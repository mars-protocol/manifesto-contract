use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton,ReadonlyBucket,Bucket, bucket_read, bucket};

pub static CONFIG_KEY: &[u8] = b"config";
pub static  SIGNEES: &[u8] = b"signee";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub signees: i32,
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}

pub fn store_signee<S: Storage>(storage: &mut S) -> Bucket<S, bool> { 
    bucket(SIGNEES, storage)
}

pub fn read_signee<S: Storage>(storage: &S) -> ReadonlyBucket<S, bool> { 
    bucket_read(SIGNEES, storage)
}
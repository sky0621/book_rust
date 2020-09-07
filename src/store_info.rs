use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StoreInfo {
    pub(crate) kvs: Vec<KV>,
}
impl StoreInfo {
    pub fn new(kvs: Vec<KV>) -> StoreInfo {
        StoreInfo { kvs }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KV {
    key: String,
    val: String,
}
impl KV {
    pub fn new(key: String, val: String) -> KV {
        KV { key, val }
    }
}

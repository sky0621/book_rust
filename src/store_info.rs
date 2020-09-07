use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// アトリビュートにてJSONパーサーのシリアライズ機能を持たせる
#[derive(Serialize, Deserialize, Debug)]
pub struct StoreInfo {
    pub kvs: HashMap<String, String>,
}
impl StoreInfo {
    pub fn new(kvs: HashMap<String, String>) -> StoreInfo {
        StoreInfo { kvs }
    }
}

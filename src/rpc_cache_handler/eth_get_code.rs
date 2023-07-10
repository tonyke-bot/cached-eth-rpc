use serde_json::Value;

use crate::rpc_cache_handler::common::extract_address_cache_key;
use crate::rpc_cache_handler::RpcCacheHandler;

#[derive(Default, Clone)]
pub struct EthGetCode;

impl RpcCacheHandler for EthGetCode {
    fn method_name(&self) -> &'static str {
        "eth_getCode"
    }

    fn extract_cache_key(&self, params: &Value) -> anyhow::Result<Option<String>> {
        extract_address_cache_key(params)
    }
}

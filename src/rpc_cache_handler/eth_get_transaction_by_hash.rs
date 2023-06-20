use anyhow::Context;
use serde_json::Value;

use crate::rpc_cache_handler::RpcCacheHandler;

#[derive(Default, Clone)]
pub struct EthGetTransactionByHash;

impl RpcCacheHandler for EthGetTransactionByHash {
    fn method_name(&self) -> &'static str {
        "eth_getTransactionByHash"
    }

    fn extract_cache_key(&self, params: &Value) -> anyhow::Result<Option<String>> {
        let params = params
            .as_array()
            .context("params not found or not an array")?;

        let tx_hash = params[0].as_str().context("params[0] not a string")?;

        Ok(Some(tx_hash.to_string()))
    }

    fn extract_cache_value(&self, result: &Value) -> anyhow::Result<(bool, String)> {
        let can_cache = result.is_object() && !result["blockHash"].is_null();

        Ok((can_cache, serde_json::to_string(result)?))
    }
}
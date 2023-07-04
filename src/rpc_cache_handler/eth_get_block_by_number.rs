use anyhow::Context;
use serde_json::Value;

use crate::rpc_cache_handler::RpcCacheHandler;

#[derive(Default, Clone)]
pub struct EthGetBlockByNumber;

impl RpcCacheHandler for EthGetBlockByNumber {
    fn method_name(&self) -> &'static str {
        "eth_getBlockByNumber"
    }

    fn extract_cache_key(&self, params: &Value) -> anyhow::Result<Option<String>> {
        let params = params
            .as_array()
            .context("params not found or not an array")?;

        let block_number = params[0].as_str().context("params[0] not a string")?;
        let transaction_detail = params[1].as_bool().context("params[1] not a bool")?;

        let block_number = u64::from_str_radix(&block_number[2..], 16)
            .context("block number not a hex string")?;

        Ok(Some(format!("0x{:x}-{}", block_number, transaction_detail)))
    }
}
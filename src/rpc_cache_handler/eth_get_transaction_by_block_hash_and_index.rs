use anyhow::Context;
use primitive_types::H256;
use serde_json::Value;
use std::str::FromStr;

use crate::rpc_cache_handler::{common, RpcCacheHandler};

#[derive(Default, Clone)]
pub struct EthGetTransactionByBlockHashAndIndex;

impl RpcCacheHandler for EthGetTransactionByBlockHashAndIndex {
    fn method_name(&self) -> &'static str {
        "eth_getTransactionByBlockHashAndIndex"
    }

    fn extract_cache_key(&self, params: &Value) -> anyhow::Result<Option<String>> {
        let params = params
            .as_array()
            .context("params not found or not an array")?;

        let block_tag = params[0].as_str().context("params[0] not a string")?;
        let block_tag = H256::from_str(block_tag.trim_start_matches("0x"))
            .context("block tag not a valid hash")?;

        let tx_index = params[1].as_str().context("params[1] not a string")?;
        let tx_index = u64::from_str_radix(tx_index.trim_start_matches("0x"), 16)
            .context("tx index not a hex string")?;

        Ok(Some(format!("0x{:x}-{}", block_tag, tx_index)))
    }

    fn extract_cache_value(&self, result: &Value) -> anyhow::Result<(bool, String)> {
        common::extract_transaction_cache_value(result)
    }
}

use std::str::FromStr;
use anyhow::Context;
use primitive_types::H256;
use serde_json::Value;

use crate::rpc_cache_handler::{common, RpcCacheHandler};

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
        let tx_hash = H256::from_str(tx_hash.trim_start_matches("0x"))
            .context("params[0] not a valid hash")?;

        Ok(Some(format!("0x{:x}", tx_hash)))
    }

    fn extract_cache_value(&self, result: &Value) -> anyhow::Result<(bool, String)> {
        common::extract_transaction_cache_value(result)
    }
}
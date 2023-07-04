use anyhow::Context;
use serde_json::Value;
use sha1::{Digest, Sha1};

use crate::rpc_cache_handler::RpcCacheHandler;

#[derive(Default, Clone)]
pub struct EthCall;

impl RpcCacheHandler for EthCall {
    fn method_name(&self) -> &'static str {
        "eth_call"
    }

    fn extract_cache_key(&self, params: &Value) -> anyhow::Result<Option<String>> {
        let params = params
            .as_array()
            .context("params not found or not an array")?;

        let tx = serde_json::to_string(params[0].as_object().expect("params[0] not an object"))?;
        let block_tag = params[1].as_str().context("params[2] not a string")?;

        if !block_tag.starts_with("0x") { return Ok(None); }
        let block_number = u64::from_str_radix(&block_tag[2..], 16).context("block number not a hex string")?;

        let mut hasher = Sha1::new();
        hasher.update(tx.as_str());
        let result = hasher.finalize();

        let tx_hash = hex::encode(result.as_slice());


        Ok(Some(format!("0x{:x}-{}", block_number, tx_hash)))
    }
}
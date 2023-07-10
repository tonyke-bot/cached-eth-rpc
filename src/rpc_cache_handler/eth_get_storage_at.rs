use anyhow::Context;
use serde_json::Value;

use crate::rpc_cache_handler::RpcCacheHandler;

#[derive(Default, Clone)]
pub struct EthGetStorageAt;

impl RpcCacheHandler for EthGetStorageAt {
    fn method_name(&self) -> &'static str {
        "eth_getStorageAt"
    }

    fn extract_cache_key(&self, params: &Value) -> anyhow::Result<Option<String>> {
        let params = params
            .as_array()
            .context("params not found or not an array")?;

        let account = params[0].as_str().context("params[0] not a string")?;
        let slot = params[1].as_str().context("params[1] not a string")?;
        let block_tag = params[2].as_str().context("params[2] not a string")?;

        if !block_tag.starts_with("0x") {
            return Ok(None);
        }
        let block_number =
            u64::from_str_radix(&block_tag[2..], 16).context("block number not a hex string")?;

        Ok(Some(format!(
            "0x{:x}-{}-{}",
            block_number,
            account.to_lowercase(),
            slot
        )))
    }
}

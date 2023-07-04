use serde_json::Value;

use crate::rpc_cache_handler::RpcCacheHandler;

#[derive(Default, Clone)]
pub struct EthChainId;

impl RpcCacheHandler for EthChainId {
    fn method_name(&self) -> &'static str {
        "eth_chainId"
    }

    fn extract_cache_key(&self, _: &Value) -> anyhow::Result<Option<String>> {
        Ok(Some("static".to_string()))
    }
}
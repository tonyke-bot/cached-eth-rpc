use serde_json::Value;

use crate::rpc_cache_handler::{common, EthGetTransactionByHash, RpcCacheHandler};

#[derive(Default, Clone)]
pub struct EthGetTransactionReceipt(EthGetTransactionByHash);

impl RpcCacheHandler for EthGetTransactionReceipt {
    fn method_name(&self) -> &'static str {
        "eth_getTransactionReceipt"
    }

    fn extract_cache_key(&self, params: &Value) -> anyhow::Result<Option<String>> {
        self.0.extract_cache_key(params)
    }

    fn extract_cache_value(&self, result: &Value) -> anyhow::Result<(bool, String)> {
        common::extract_transaction_cache_value(result)
    }
}
use anyhow::Result;
use serde_json::Value;

pub use eth_get_balance::EthGetBalance;
pub use eth_get_block_by_number::EthGetBlockByNumber;
pub use eth_get_code::EthGetCode;
pub use eth_get_transaction_count::EthGetTransactionCount;
pub use eth_get_transaction_by_hash::EthGetTransactionByHash;
pub use eth_get_storage_at::EthGetStorageAt;

mod common;
mod eth_get_balance;
mod eth_get_block_by_number;
mod eth_get_code;
mod eth_get_transaction_count;
mod eth_get_transaction_by_hash;
mod eth_get_storage_at;

pub trait RpcCacheHandler: Send + Sync {
    fn method_name(&self) -> &'static str;

    fn extract_cache_key(&self, params: &Value) -> Result<Option<String>>;

    fn extract_cache_value(&self, result: &Value) -> Result<(bool, String)> {
        Ok((true, serde_json::to_string(result)?))
    }
}

pub type RpcCacheHandlerFactory = fn() -> Box<dyn RpcCacheHandler>;

pub fn all_factories() -> Vec<RpcCacheHandlerFactory> {
    vec![
        || Box::new(EthGetBalance::default()) as Box<dyn RpcCacheHandler>,
        || Box::new(EthGetCode::default()) as Box<dyn RpcCacheHandler>,
        || Box::new(EthGetTransactionCount::default()) as Box<dyn RpcCacheHandler>,
        || Box::new(EthGetBlockByNumber::default()) as Box<dyn RpcCacheHandler>,
        || Box::new(EthGetStorageAt::default()) as Box<dyn RpcCacheHandler>,
        || Box::new(EthGetTransactionByHash::default()) as Box<dyn RpcCacheHandler>,
    ]
}
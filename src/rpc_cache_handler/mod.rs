use anyhow::Result;
use serde_json::Value;

pub use eth_call::EthCall;
pub use eth_chainid::EthChainId;
pub use eth_get_balance::EthGetBalance;
pub use eth_get_block_by_number::EthGetBlockByNumber;
pub use eth_get_code::EthGetCode;
pub use eth_get_storage_at::EthGetStorageAt;
pub use eth_get_transaction_by_hash::EthGetTransactionByHash;
pub use eth_get_transaction_count::EthGetTransactionCount;

mod common;
mod eth_call;
mod eth_chainid;
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
        Ok((!result.is_null(), serde_json::to_string(result)?))
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
        || Box::new(EthCall::default()) as Box<dyn RpcCacheHandler>,
        || Box::new(EthChainId::default()) as Box<dyn RpcCacheHandler>,
    ]
}
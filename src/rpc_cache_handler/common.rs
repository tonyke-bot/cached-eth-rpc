use crate::rpc_cache_handler::common;
use anyhow::Context;
use primitive_types::H256;
use serde_json::Value;
use std::str::FromStr;

pub(super) fn extract_address_cache_key(params: &Value) -> anyhow::Result<Option<String>> {
    let params = params
        .as_array()
        .context("params not found or not an array")?;

    let account = params[0].as_str().context("params[0] not a string")?;
    let block_tag = common::extract_and_format_block_tag(&params[1])?;

    if block_tag.is_none() {
        return Ok(None);
    }

    let block_tag = block_tag.unwrap();

    Ok(Some(format!("{}-{}", block_tag, account)))
}

pub(super) fn extract_transaction_cache_value(result: &Value) -> anyhow::Result<(bool, String)> {
    let can_cache = result.is_object() && !result["blockHash"].is_null();

    Ok((can_cache, serde_json::to_string(result)?))
}

pub(super) fn extract_and_format_block_tag(value: &Value) -> anyhow::Result<Option<String>> {
    match value {
        Value::String(block_tag) => {
            if !block_tag.starts_with("0x") {
                return Ok(None);
            }

            let block_number =
                u64::from_str_radix(&block_tag[2..], 16).context("block tag not a hex string")?;

            Ok(Some(format!("0x{:x}", block_number)))
        }
        Value::Object(block_tag) => {
            if let Some(Value::String(block_number_str)) = block_tag.get("blockNumber") {
                let block_number = u64::from_str_radix(&block_number_str, 16)
                    .context("block number not a hex string")?;

                return Ok(Some(format!("0x{}", block_number)));
            } else if let Some(Value::String(block_hash_str)) = block_tag.get("blockHash") {
                let hash = H256::from_str(block_hash_str).context("block hash not a hex string")?;

                return Ok(Some(format!("0x{:x}", hash)));
            }

            Ok(None)
        }
        _ => Err(anyhow::anyhow!("block tag not a string or object")),
    }
}

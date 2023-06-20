use anyhow::Context;
use serde_json::Value;

pub(super) fn extract_address_cache_key(params: &Value) -> anyhow::Result<Option<String>> {
    let params = params
        .as_array()
        .context("params not found or not an array")?;

    let account = params[0].as_str().context("params[0] not a string")?;
    let block_tag = params[1].as_str().context("params[1] not a string")?;

    if !block_tag.starts_with("0x") { return Ok(None); }
    let block_number = u64::from_str_radix(&block_tag[2..], 16).context("block number not a hex string")?;

    Ok(Some(format!("{:x}-{}", block_number, account)))
}

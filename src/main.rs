use std::collections::HashMap;
use std::sync::Mutex;

use actix_web::{App, Error, error, HttpResponse, HttpServer, web};
use anyhow::Context;
use clap::Parser;
use env_logger::Env;
use reqwest::Url;
use serde_json::{json, Value};

use crate::cli::Cli;
use crate::rpc_cache_handler::RpcCacheHandler;

mod cli;
mod rpc_cache_handler;

struct ChainState {
    rpc_url: Url,

    // method_name -> (handler, cache_key -> cache_value)
    cache: HashMap<String, (Box<dyn RpcCacheHandler>, Mutex<HashMap<String, String>>)>,
}

#[derive(Default)]
struct AppState {
    chains: HashMap<String, ChainState>,
}

enum CacheStatus {
    Cached,
    New,
    CannotCache,
}

async fn request_rpc(rpc_url: Url, body: &Value) -> anyhow::Result<Value> {
    let client = reqwest::Client::new();

    let result = client
        .post(rpc_url)
        .json(body)
        .send()
        .await?
        .json::<Value>()
        .await?;

    Ok(result)
}

async fn handle(
    handler: &Box<dyn RpcCacheHandler>,
    cache_store: &Mutex<HashMap<String, String>>,
    params: &Value,
    rpc_url: &Url,
    body: &Value,
) -> anyhow::Result<Option<(CacheStatus, String, Value)>> {
    let cache_key = handler
        .extract_cache_key(params)
        .context("fail to extract cache key")?;

    if cache_key.is_none() {
        return Ok(None);
    }

    let cache_key = cache_key.unwrap();
    let mut cache = cache_store.lock().unwrap();

    let value = (*cache).get(&cache_key)
        .map(|v| v.clone());
    let mut cache_status = CacheStatus::Cached;

    let result = if let Some(value) = value {
        serde_json::from_str::<Value>(&value)
            .context("fail to deserialize cache value")?
    } else {
        let mut result = request_rpc(rpc_url.clone(), body)
            .await
            .context("fail to make rpc request")?;

        let result_value = result.get_mut("result").unwrap().take();

        let extracted_value = handler
            .extract_cache_value(&result_value)
            .context("fail to extract cache value")?;

        let (can_cache, cache_value) = extracted_value;
        if can_cache {
            cache_status = CacheStatus::New;
            (*cache).insert(cache_key.clone(), cache_value.clone());
        } else {
            cache_status = CacheStatus::CannotCache;
        }

        result_value
    };

    Ok(Some((cache_status, cache_key, result)))
}

#[actix_web::post("/{chain}")]
async fn rpc_call(
    path: web::Path<(String, )>,
    data: web::Data<AppState>,
    body: web::Json<Value>,
) -> Result<HttpResponse, Error> {
    let (chain, ) = path.into_inner();
    let chain_state = data
        .chains
        .get(&chain.to_uppercase())
        .ok_or_else(|| error::ErrorNotFound("endpoint not supported"))?;

    let id = body["id"].as_u64().ok_or_else(|| error::ErrorBadRequest("id not found"))?;
    let method = body["method"].as_str().ok_or_else(|| error::ErrorBadRequest("method not found"))?;
    let params = &body["params"];

    if let Some((handler, cache_store)) = chain_state.cache.get(method) {
        let result = handle(handler, cache_store, params, &chain_state.rpc_url, &body).await;

        if let Err(err) = result {
            log::error!("fail to execute {} because: {}", method, err);
        } else if let Ok(Some((cache_status, cache_key, result))) = result {
            match cache_status {
                CacheStatus::Cached => log::info!("method {} is cached with key {}", method, cache_key),
                CacheStatus::New => log::info!("method {} is newly inserted into cache with key {}", method, cache_key),
                CacheStatus::CannotCache => log::info!("method {} is not cached", method),
            };

            return Ok(HttpResponse::Ok().json(json!({
                "jsonrpc": "2.0",
                "id": id,
                "result": result,
            })));
        }
    }

    log::info!("{} request is skip for caching", method);

    let result = request_rpc(chain_state.rpc_url.clone(), &body).await
        .map_err(|err| {
            log::error!("fail to execute {} because: {}", method, err);
            error::ErrorInternalServerError(err)
        })?;

    Ok(HttpResponse::Ok().json(result))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let arg = Cli::parse();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let mut app_state = AppState::default();
    let handler_factories = rpc_cache_handler::all_factories();

    log::info!("Provisioning cache tables");

    for (name, rpc_url) in arg.endpoints.iter() {
        log::info!("Adding endpoint {} linked to {}", name, rpc_url);

        let mut chain_state = ChainState {
            rpc_url: rpc_url.clone(),
            cache: HashMap::new(),
        };

        for factory in &handler_factories {
            let handler = factory();
            chain_state.cache.insert(
                handler.method_name().to_string(),
                (handler, Default::default()));
        }

        app_state.chains.insert(name.to_string(), chain_state);
    }

    let app_state = web::Data::new(app_state);

    HttpServer::new(move ||
        App::new()
            .service(rpc_call)
            .app_data(app_state.clone()))
        .bind(("127.0.0.1", arg.port))?
        .run()
        .await
}

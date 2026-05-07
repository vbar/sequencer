use axum::extract::Query;
use axum::routing::{get, post};
use axum::{Json, Router};
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

#[derive(serde_derive::Deserialize)]
struct EthToStrkOracleQuery {
    timestamp: u64,
}

async fn eth_to_strk_oracle_get_price(
    Query(query): Query<EthToStrkOracleQuery>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "timestamp": query.timestamp,
        "price": "0x3635c9adc5dea00000", // 10^21
        "decimals": 18,
    }))
}

async fn feeder_gateway_get_contract_addresses() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "Starknet": "0x5fbdb2315678afecb367f032d93f642f64180aa3",
        "GpsStatementVerifier": "0x47312450B3Ac8b5b8e247a6bB6d523e7605bDb60",
        "strk_l2_token_address": "0x4718f5a0fc34cc1af16a1cdee98ffb20c31f5cd61d6ab07201858f4287c938d",
        "eth_l2_token_address": "0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7"
    }))
}

async fn feeder_gateway_get_public_key() -> Json<serde_json::Value> {
    Json(serde_json::json!("0x48253ff2c3bed7af18bde0b611b083b39445959102d4947c51c4db6aa4f4e58"))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let format = tracing_subscriber::fmt::format().compact();
    tracing_subscriber::fmt()
        .event_format(format)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let listener_address = std::env::args().nth(1).unwrap_or_else(|| "[::]:8080".to_owned());
    tracing::info!(%listener_address, "Starting up");
    let listener = tokio::net::TcpListener::bind(listener_address).await?;

    let router = Router::new()
        .route("/cende_recorder/write_blob", post(move || async { "" }))
        .route("/cende_recorder/write_pre_confirmed_block", post(move || async { "" }))
        .route("/eth_to_strk_oracle", get(eth_to_strk_oracle_get_price))
        .route("/feeder_gateway/get_contract_addresses", get(feeder_gateway_get_contract_addresses))
        .route("/feeder_gateway/get_public_key", get(feeder_gateway_get_public_key));

    axum::serve(listener, router.layer(TraceLayer::new_for_http())).await
}

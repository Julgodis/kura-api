use std::net::SocketAddr;

use axum::extract::Query;
use kura_api::indexer::RecentRequest;
use kura_api::indexer::RecentResponse;
use kura_api::indexer::SearchRequest;
use kura_api::indexer::SearchResponse;
use kura_api::Result;
use kura_api::health::HealthRequest;
use kura_api::health::HealthResponse;

#[axum::debug_handler]
async fn indexer_recent_handler(
    Query(request): Query<RecentRequest>,
) -> Result<RecentResponse> {
    request.validate()?;

    let releases = vec![];

    Ok(RecentResponse {
        since: chrono::Utc::now(),
        until: chrono::Utc::now(),
        releases,
    })
}

#[axum::debug_handler]
async fn indexer_search_handler(
    Query(request): Query<SearchRequest>,
) -> Result<SearchResponse> {
    request.validate()?;

    let releases = vec![];

    Ok(SearchResponse { releases })
}

#[axum::debug_handler]
async fn health_handler(Query(_): Query<HealthRequest>) -> Result<HealthResponse> {
    let uptime = chrono::Duration::seconds(3600);
    let response = HealthResponse {
        service: "kura-indexer".to_string(),
        version: "1.0.0".to_string(),
        commit: "abc123".to_string(),
        status: "ok".to_string(),
        uptime,
        is_indexer: true,
        is_parser: false,
        is_classifier: false,
        is_profiler: false,
        is_scorer: false,
    };
    Ok(response)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = axum::Router::new()
        .route(
            "/api/v1/indexer/recent",
            axum::routing::get(indexer_recent_handler),
        )
        .route(
            "/api/v1/indexer/search",
            axum::routing::get(indexer_search_handler),
        )
        .route("/api/v1/health", axum::routing::get(health_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}

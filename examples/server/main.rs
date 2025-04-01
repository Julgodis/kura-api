use std::net::SocketAddr;

use axum::extract::Query;
use kura_indexer::Result;
use kura_indexer::health::HealthRequest;
use kura_indexer::health::HealthResponse;
use kura_indexer::releases;

#[axum::debug_handler]
async fn releases_recent_handler(
    Query(request): Query<releases::RecentRequest>,
) -> Result<releases::RecentResponse> {
    request.validate()?;

    let releases = vec![
        releases::Release::builder("1", chrono::Utc::now())
            .add_category(releases::Category::Anime(releases::AnimeCategory::Movie))
            .add_tag("tag1".to_string())
            .build(),
        releases::Release::builder("2", chrono::Utc::now())
            .add_category(releases::Category::Anime(releases::AnimeCategory::TV))
            .add_tag("tag2".to_string())
            .build(),
    ];

    Ok(releases::RecentResponse {
        since: chrono::Utc::now(),
        until: chrono::Utc::now(),
        releases,
    })
}

#[axum::debug_handler]
async fn releases_search_handler(
    Query(request): Query<releases::SearchRequest>,
) -> Result<releases::SearchResponse> {
    request.validate()?;

    let releases = vec![
        releases::Release::builder("1", chrono::Utc::now())
            .add_category(releases::Category::Anime(releases::AnimeCategory::Movie))
            .add_tag("tag1".to_string())
            .build(),
        releases::Release::builder("2", chrono::Utc::now())
            .add_category(releases::Category::Anime(releases::AnimeCategory::TV))
            .add_tag("tag2".to_string())
            .build(),
    ];

    Ok(releases::SearchResponse { releases })
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
    };
    Ok(response)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = axum::Router::new()
        .route(
            "/api/v1/releases/recent",
            axum::routing::get(releases_recent_handler),
        )
        .route(
            "/api/v1/releases/search",
            axum::routing::get(releases_search_handler),
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

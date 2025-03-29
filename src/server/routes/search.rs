use axum::{Extension, Json};

use crate::{
    api,
    server::{Indexer},
};

pub async fn post<I: Indexer>(
    Extension(context): Extension<I::Context>,
    Json(request): Json<api::SearchRequest>,
) -> api::Result<Json<api::SearchResponse>> {
    tracing::debug!("search_handler: {:?}", request);
    let response = I::search(context, request).await?;
    Ok(Json(response))
}

use axum::{Extension, Json};

use crate::{
    api,
    server::{Indexer},
};

pub async fn post<I: Indexer>(
    Extension(context): Extension<I::Context>,
    Json(request): Json<api::RecentRequest>,
) -> api::Result<Json<api::RecentResponse>> {
    tracing::debug!("recent_handler: {:?}", request);
    let response = I::recent(context, request).await?;
    Ok(Json(response))
}

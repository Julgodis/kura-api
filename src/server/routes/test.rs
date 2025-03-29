use axum::{Extension, Json};

use crate::{api, server::Indexer};

pub async fn post<I: Indexer>(
    Extension(context): Extension<I::Context>,
    Json(request): Json<api::TestRequest>,
) -> api::Result<Json<api::TestResponse>> {
    tracing::debug!("test_handler: {:?}", request);
    let response = I::test(context, request).await?;
    Ok(Json(response))
}

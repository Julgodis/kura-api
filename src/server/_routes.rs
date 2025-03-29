use std::convert::Infallible;

use axum::{
    Extension, Json, Router,
    extract::State,
    response::{Sse, sse::Event},
    routing::get,
};
use futures::Stream;

use crate::{
    api::{
        self, RecentRequest, RecentResponse,
        search::{SearchRequest, SearchResponse},
        subscribe::{SubscribeEvent, SubscribeRequest},
        test::{TestRequest, TestResponse},
    },
    server::{server_state_subscribe, server_state_unsubscribe},
};

use super::{Indexer, ServerState};

pub fn routes<S: ServerState<I> + Clone + Sync + Send + 'static, I: Indexer + 'static>() -> Router {
    Router::new()
        .route("/kura/indexer/test", get(test_handler::<S, I>))
        .route("/kura/indexer/search", get(search_handler::<S, I>))
        .route("/kura/indexer/recent", get(recent_handler::<S, I>))
}

async fn test_handler<S: ServerState<I>, I: Indexer>(
    Extension(state): Extension<S>,
    Json(request): Json<TestRequest>,
) -> api::Result<Json<TestResponse>> {
    tracing::debug!("test_handler: {:?}", request);

    I::test(state.context(), request).await?;

    Ok(Json(TestResponse {}))
}

async fn subscribe_handler<S: ServerState<I>, I: Indexer>(
    Extension(state): Extension<S>,
    Json(request): Json<SubscribeRequest>,
) -> Sse<impl Stream<Item = api::Result<Event>>> {
    tracing::debug!("subscribe_handler: {:?}", request);

    let (sender, mut receiver) = tokio::sync::mpsc::channel(128);

    let category = request.category.clone();
    let stream = async_stream::try_stream! {
        let subscription = server_state_subscribe(state.clone(), category, sender).await?;
        let uuid = subscription.uuid;

        tracing::debug!("subscribe_handler: stream started");

        while let Some(event) = receiver.recv().await {
            yield Event::default().data("test");
        }

        tracing::debug!("subscribe_handler: stream closed");
        server_state_unsubscribe(state, uuid).await?;
    };

    Sse::new(stream)
}

async fn search_handler<S: ServerState<I>, I: Indexer>(
    Extension(state): Extension<S>,
    Json(request): Json<SearchRequest>,
) -> Json<SearchResponse> {
    tracing::debug!("search_handler: {:?}", request);
    Json(SearchResponse {})
}

async fn recent_handler<S: ServerState<I>, I: Indexer>(
    Extension(state): Extension<S>,
    Json(request): Json<RecentRequest>,
) -> api::Result<Json<RecentResponse>> {
    tracing::debug!("recent_handler: {:?}", request);
    let response = I::recent(state.context(), request).await?;
    Ok(Json(response))
}

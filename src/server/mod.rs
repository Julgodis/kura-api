use std::{collections::HashMap, net::SocketAddr, sync::Arc, task::Context};

use axum::{Extension, Router, routing::MethodRouter};
use serde_json::value::Index;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::api;

mod helper;
mod routes;

#[derive(Clone, Debug)]
pub struct SubscriptionData {
    pub date: chrono::DateTime<chrono::Utc>,
    pub data: String,
}

#[derive(Clone, Debug)]
pub struct Subscription {
    pub uuid: Uuid,
    pub category: String,
    pub sender: tokio::sync::mpsc::Sender<SubscriptionData>,
    pub reference_count: usize,
}

#[async_trait::async_trait]
pub trait Indexer: Send + Sync + Clone {
    type Context: Send + Sync + Clone + 'static;

    async fn test(
        context: Self::Context,
        request: crate::api::test::TestRequest,
    ) -> api::Result<api::TestResponse>;

    async fn search(
        context: Self::Context,
        request: crate::api::search::SearchRequest,
    ) -> api::Result<api::search::SearchResponse>;

    async fn recent(
        context: Self::Context,
        request: crate::api::recent::RecentRequest,
    ) -> api::Result<api::recent::RecentResponse>;
}

pub struct ServerBuilder {
    addr: SocketAddr,
    context: (),
    router: Router<()>,
}

impl ServerBuilder {
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            addr,
            context: (),
            router: Router::new(),
        }
    }

    pub fn with_indexer<I>(self, context: I::Context) -> ServerBuilderIndexer<I>
    where
        I: Indexer + 'static,
    {
        ServerBuilderIndexer::<I> {
            addr: self.addr,
            context: context,
            router: Router::new(),
        }
    }

    pub fn with_indexer_state<I, S>(self, context: I::Context) -> ServerBuilderIndexerState<I, S>
    where
        I: Indexer + 'static,
        S: Clone + Send + Sync + 'static,
    {
        ServerBuilderIndexerState::<I, S> {
            addr: self.addr,
            context: context,
            router: Router::new(),
        }
    }
}

pub struct ServerBuilderIndexer<I>
where
    I: Indexer + 'static,
{
    addr: SocketAddr,
    context: I::Context,
    router: Router<()>,
}

impl<I> ServerBuilderIndexer<I>
where
    I: Indexer + 'static,
{
    pub fn with_default_routes(mut self) -> Self {
        self.router = self
            .router
            .route(
                "/kura/indexer/test",
                axum::routing::post(routes::test::post::<I>),
            )
            .route(
                "/kura/indexer/search",
                axum::routing::post(routes::search::post::<I>),
            )
            .route(
                "/kura/indexer/recent",
                axum::routing::post(routes::recent::post::<I>),
            );
        self
    }

    pub fn route(mut self, path: &str, route: MethodRouter<()>) -> Self {
        self.router = self.router.route(path, route);
        self
    }

    pub fn with_router(mut self, router: impl Fn(Router<()>) -> Router<()>) -> Self {
        self.router = router(self.router);
        self
    }

    pub async fn start(self) -> api::Result<()> {
        let Self {
            context,
            addr,
            router,
        } = self;

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, router.layer(Extension(context))).await?;

        Ok(())
    }
}

pub struct ServerBuilderIndexerState<I, S>
where
    I: Indexer + 'static,
{
    addr: SocketAddr,
    context: I::Context,
    router: Router<S>,
}

impl<I, S> ServerBuilderIndexerState<I, S>
where
    I: Indexer + 'static,
    S: Clone + Send + Sync + 'static,
{
    pub fn with_default_routes(mut self) -> Self {
        self.router = self
            .router
            .route(
                "/kura/indexer/test",
                axum::routing::post(routes::test::post::<I>),
            )
            .route(
                "/kura/indexer/search",
                axum::routing::post(routes::search::post::<I>),
            )
            .route(
                "/kura/indexer/recent",
                axum::routing::post(routes::recent::post::<I>),
            );
        self
    }

    pub fn route(mut self, path: &str, route: MethodRouter<S>) -> Self {
        self.router = self.router.route(path, route);
        self
    }

    pub async fn start(self, state: S) -> api::Result<()> {
        let Self {
            context,
            addr,
            router,
        } = self;

        let app = router.layer(Extension(context)).with_state(state);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}

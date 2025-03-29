pub mod error;
pub mod search;
pub mod subscribe;
pub mod recent;
pub mod test;

pub use crate::api::error::{ApiError, Result};
pub use crate::api::search::{SearchRequest, SearchResponse};
pub use crate::api::subscribe::{SubscribeEvent, SubscribeRequest};
pub use crate::api::test::{TestRequest, TestResponse};
pub use crate::api::recent::{RecentRequest, RecentResponse, RecentItem};

pub trait ResponseTrait {}
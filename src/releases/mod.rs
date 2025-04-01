use serde::{Deserialize, Serialize};

mod recent;
mod search;

pub use recent::{RecentRequest, RecentResponse};
pub use search::{SearchRequest, SearchResponse};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Category {
    #[serde(rename = "anime")]
    Anime(AnimeCategory),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AnimeCategory {
    #[serde(rename = "movie")]
    Movie,
    #[serde(rename = "tv")]
    TV,
    #[serde(rename = "ova")]
    OVA,
    #[serde(rename = "ona")]
    ONA,
    #[serde(rename = "special")]
    Special,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Release {
    pub id: String,
    pub date: chrono::DateTime<chrono::Utc>,
    pub categories: Vec<Category>,
    pub tags: Vec<String>,
}

impl Release {
    pub fn builder(id: impl AsRef<str>, date: chrono::DateTime<chrono::Utc>) -> ReleaseBuilder {
        ReleaseBuilder::new(id, date)
    }
}

pub struct ReleaseBuilder {
    id: String,
    date: chrono::DateTime<chrono::Utc>,
    categories: Vec<Category>,
    tags: Vec<String>,
}

impl ReleaseBuilder {
    pub fn new(id: impl AsRef<str>, date: chrono::DateTime<chrono::Utc>) -> Self {
        Self {
            id: id.as_ref().to_string(),
            date,
            categories: Vec::new(),
            tags: Vec::new(),
        }
    }

    pub fn add_category(mut self, category: Category) -> Self {
        self.categories.push(category);
        self
    }

    pub fn add_tag(mut self, tag: impl AsRef<str>) -> Self {
        self.tags.push(tag.as_ref().to_string());
        self
    }

    pub fn build(self) -> Release {
        Release {
            id: self.id,
            date: self.date,
            categories: self.categories,
            tags: self.tags,
        }
    }
}

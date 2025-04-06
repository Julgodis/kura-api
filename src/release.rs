use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReleaseType {
    #[serde(rename = "movie")]
    Movie,
    #[serde(rename = "series")]
    Series,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VideoResolution {
    #[serde(rename = "480p")]
    P480,
    #[serde(rename = "720p")]
    P720,
    #[serde(rename = "1080p")]
    P1080,
    #[serde(rename = "2160p")]
    P2160,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VideoCodec {
    #[serde(rename = "h264")]
    H264,
    #[serde(rename = "h265")]
    H265,
    #[serde(rename = "vp9")]
    VP9,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AudioCodec {
    #[serde(rename = "aac")]
    AAC,
    #[serde(rename = "ac3")]
    AC3,
    #[serde(rename = "dts")]
    DTS,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AudioChannels {
    #[serde(rename = "2.0")]
    Stereo,
    #[serde(rename = "5.1")]
    Surround51,
    #[serde(rename = "7.1")]
    Surround71,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReleaseSource {
    #[serde(rename = "bluray")]
    BluRay,
    #[serde(rename = "web")]
    Web,
    #[serde(rename = "dvd")]
    DVD,
    #[serde(untagged)]
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Release {
    pub release_title: String,
    pub release_type: ReleaseType,

    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub year: Option<usize>,

    #[serde(default)]
    pub season: Option<usize>,
    #[serde(default)]
    pub episode: Option<usize>,
    #[serde(default)]
    pub absolute_episode: Option<usize>,
    #[serde(default)]
    pub special_episode: Option<usize>,

    #[serde(default)]
    pub video_resolution: Option<VideoResolution>,
    #[serde(default)]
    pub video_codec: Option<VideoCodec>,

    #[serde(default)]
    pub audio_codec: Option<AudioCodec>,
    #[serde(default)]
    pub audio_channels: Option<AudioChannels>,

    #[serde(default)]
    pub release_source: Option<ReleaseSource>,
    #[serde(default)]
    pub release_group: Option<String>,
    #[serde(default)]
    pub release_hash: Option<String>,

    #[serde(default)]
    pub fields: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub tags: HashSet<String>,
    #[serde(default)]
    pub classes: HashSet<String>,
    #[serde(default)]
    pub profiles: HashSet<String>,
}

impl Release {
    pub fn new(
        release_title: impl Into<String>,
        release_type: ReleaseType,
    ) -> Self {
        Self {
            release_title: release_title.into(),
            release_type,
            title: None,
            year: None,
            season: None,
            episode: None,
            absolute_episode: None,
            special_episode: None,
            video_resolution: None,
            video_codec: None,
            audio_codec: None,
            audio_channels: None,
            release_source: None,
            release_group: None,
            release_hash: None,
            fields: HashMap::new(),
            tags: HashSet::new(),
            classes: HashSet::new(),
            profiles: HashSet::new(),
        }
    }

    pub fn release_title(mut self, value: impl Into<String>) -> Self {
        self.release_title = value.into();
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn year(mut self, value: usize) -> Self {
        self.year = Some(value);
        self
    }

    pub fn season(mut self, value: usize) -> Self {
        self.season = Some(value);
        self
    }

    pub fn episode(mut self, value: usize) -> Self {
        self.episode = Some(value);
        self
    }

    pub fn absolute_episode(mut self, value: usize) -> Self {
        self.absolute_episode = Some(value);
        self
    }

    pub fn special_episode(mut self, value: usize) -> Self {
        self.special_episode = Some(value);
        self
    }

    pub fn video_resolution(mut self, value: VideoResolution) -> Self {
        self.video_resolution = Some(value);
        self
    }

    pub fn video_codec(mut self, value: VideoCodec) -> Self {
        self.video_codec = Some(value);
        self
    }

    pub fn audio_codec(mut self, value: AudioCodec) -> Self {
        self.audio_codec = Some(value);
        self
    }

    pub fn audio_channels(mut self, value: AudioChannels) -> Self {
        self.audio_channels = Some(value);
        self
    }

    pub fn release_source(mut self, value: ReleaseSource) -> Self {
        self.release_source = Some(value);
        self
    }

    pub fn release_group(mut self, value: impl Into<String>) -> Self {
        self.release_group = Some(value.into());
        self
    }

    pub fn release_hash(mut self, value: impl Into<String>) -> Self {
        self.release_hash = Some(value.into());
        self
    }
}
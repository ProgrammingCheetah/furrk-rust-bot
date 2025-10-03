use std::{fmt::Display, str::FromStr};

use anyhow::anyhow;
use url::Url;

pub enum Platform {
    Twitter,
    E621,
    FurAffinity,
    Unknown,
}

impl Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Platform::Twitter => "Twitter",
            Platform::E621 => "e621",
            Platform::FurAffinity => "FurAffinity",
            Platform::Unknown => "Unknown",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Platform {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Twitter" => Ok(Platform::Twitter),
            "E621" => Ok(Platform::E621),
            "FurAffinity" => Ok(Platform::FurAffinity),
            _ => Err(anyhow!("'{}' is not a valid platform", s)),
        }
    }
}

pub struct MediaItem {
    pub platform: Platform,
    pub source_url: Url,
    pub media_url: Url,
    pub author: String,
    pub file_type: String,
}

pub struct MediaItemQueue {
    pub media_item: MediaItem,
}

pub struct MediaItemQueueResult {
    pub media_item_id: i32
}

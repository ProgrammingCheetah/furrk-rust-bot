use async_trait::async_trait;
use reqwest::Client;
use reqwest::redirect::Policy;
use std::path::Path;
use url::Url;

use crate::errors::ExtractionError;
use crate::extractors::MediaExtractor;
use crate::models::{MediaItem, Platform};
pub struct TwitterExtractor {
    client: Client,
}

impl TwitterExtractor {
    pub fn new() -> Result<Self, reqwest::Error> {
        let client = Client::builder().redirect(Policy::none()).build()?;
        Ok(Self { client })
    }
}

#[async_trait]
impl MediaExtractor for TwitterExtractor {
    async fn extract(&self, url: &Url, author: &str) -> Result<MediaItem, ExtractionError> {
        let mut fixup_url = url.clone();
        fixup_url
            .set_host(Some("d.fixupx.com"))
            .map_err(|_| ExtractionError::ContentNotFound)?;

        let response = self
            .client
            .get(fixup_url.as_str())
            .send()
            .await
            .map_err(|_| ExtractionError::ContentNotFound)?;

        let final_url = response
            .headers()
            .get("location")
            .and_then(|val| val.to_str().ok())
            .ok_or(ExtractionError::ContentNotFound)?;

        let media_url = Url::parse(final_url).map_err(|_| ExtractionError::ContentNotFound)?;

        let file_type = Path::new(media_url.path())
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();

        Ok(MediaItem {
            platform: Platform::Twitter,
            source_url: url.clone(),
            media_url,
            author: author.to_string(),
            file_type,
        })
    }
}

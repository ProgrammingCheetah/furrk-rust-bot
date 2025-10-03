use std::path::Path;

use async_trait::async_trait;
use url::Url;

use crate::{
    errors::ExtractionError,
    extractors::MediaExtractor,
    models::{MediaItem, Platform},
};

pub struct E621Extractor;

#[async_trait]
impl MediaExtractor for E621Extractor {
    async fn extract(&self, url: &Url, author: &str) -> Result<MediaItem, ExtractionError> {
        let path = Path::new(url.path());
        let file_type = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_string())
            .ok_or(ExtractionError::ContentNotFound)?;

        Ok(MediaItem {
            platform: Platform::E621,
            source_url: url.clone(),
            media_url: url.clone(),
            author: author.to_string(),
            file_type,
        })
    }
}

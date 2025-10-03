// pub mod e621;
// pub mod furaffinity;
// pub mod twitter;
// use async_trait::async_trait;
// use url::Url;

// use crate::{
//     errors::ExtractionError,
//     extractors::{
//         e621::E621Extractor, furaffinity::FurAffinityExtractor, twitter::TwitterExtractor,
//     },
//     models::{MediaItem, Platform},
// };

// #[async_trait]
// pub trait MediaExtractor {
//     async fn extract(&self, url: &Url, author: &str) -> Result<MediaItem, ExtractionError>;
// }

// fn identify_platform(url: &Url) -> Platform {
//     match url.host_str() {
//         Some("twitter.com") | Some("x.com") => Platform::Twitter,
//         Some("e621.net") => Platform::E621,
//         Some("furaffinity.net") => Platform::FurAffinity,
//         _ => Platform::Unknown,
//     }
// }

// fn extractor_factory(platform: &Platform) -> Option<Box<dyn MediaExtractor>> {
//     match platform {
//         Platform::Twitter => TwitterExtractor::new()
//             .ok()
//             .map(|extractor| Box::new(extractor) as Box<dyn MediaExtractor>),
//         Platform::FurAffinity => Some(Box::new(FurAffinityExtractor)),
//         Platform::E621 => Some(Box::new(E621Extractor)),
//         Platform::Unknown => None,
//     }
// }

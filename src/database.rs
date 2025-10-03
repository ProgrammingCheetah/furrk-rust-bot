// use crate::models::MediaItem;
// use anyhow::Result;
// use sqlx::PgPool;
// use url::Url;

// pub struct MediaItemRepository;

// impl MediaItemRepository {
//     async fn exists(pool: &PgPool, source_url: &Url) -> Result<bool> {
//         let exists = sqlx::query_scalar!(
//             "SELECT EXISTS(SELECT 1 FROM media_item WHERE source_url = $1)",
//             source_url.as_str()
//         )
//         .fetch_one(pool)
//         .await?;
//         Ok(exists.unwrap_or(false))
//     }
//     async fn save(pool: &PgPool, item: &MediaItem) -> Result<()> {
//         let author_id = sqlx::query_scalar!(
//             "SELECT id FROM channel_user WHERE telegram_id = $1",
//             &item.author
//         )
//         .fetch_one(pool)
//         .await?;
//         let file_type_id = sqlx::query_scalar!(
//             "SELECT id FROM file_type WHERE f_type = $1",
//             &item.file_type
//         )
//         .fetch_one(pool)
//         .await?;

//         sqlx::query!(
//             r#"
//         INSERT INTO media_item (platform, source_url, media_url, author_id, file_type_id)
//         VALUES ($1, $2, $3, $4, $5)
//         ON CONFLICT (source_url) DO NOTHING;
//         "#,
//             item.platform.to_string(),
//             item.source_url.as_str(),
//             item.media_url.as_str(),
//             author_id,
//             file_type_id
//         )
//         .execute(pool)
//         .await?;

//         Ok(())
//     }
//     async fn get_random(pool: &PgPool) -> Result<Option<MediaItem>> {
//         let item = sqlx::query_as!(
//             DbMediaItem,
//             r#"
//             SELECT
//             mi.platform,
//             mi.source_url,
//             mi.media_url,
//             cu.telegram_id AS author,
//             ft.f_type AS file_type
//             FROM media_item AS mi
//             JOIN channel_user AS cu ON mi.author_id = cu.id
//             JOIN file_type AS ft ON mi.file_type_id = ft.id
//             ORDER BY RANDOM()
//             LIMIT 1;  
//             "#
//         )
//         .fetch_optional(pool)
//         .await?;

//         if let Some(row) = item {
//             let item = MediaItem {
//                 platform: row.platform.parse()?,
//                 source_url: row.source_url.parse()?,
//                 media_url: row.media_url.parse()?,
//                 author: row.author,
//                 file_type: row.file_type,
//             };
//             Ok(Some(item))
//         } else {
//             Ok(None)
//         }
//     }
// }

// struct DbMediaItem {
//     platform: String,
//     source_url: String,
//     media_url: String,
//     author: String,
//     file_type: String,
// }

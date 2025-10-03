use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: u64,
    exp: usize,
}

impl Claims {
    fn get_secret() -> String {
        dotenv::var("JWT_SECRET").unwrap_or_else(|_| "testing".to_string())
    }

    pub fn token(&self) -> Option<String> {
        let secret = Self::get_secret();
        encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .ok()
    }
}

impl From<u64> for Claims {
    fn from(user_id: u64) -> Self {
        let expiration = Utc::now()
            .checked_add_signed(Duration::weeks(52))
            .expect("valid timestamp")
            .timestamp() as usize;
        Claims {
            sub: user_id,
            exp: expiration,
        }
    }
}

use std::sync::Arc;

use frankenstein::client_reqwest::Bot;

pub struct AppState {
    pub bot: Bot,
}
pub type SharedState = Arc<AppState>;

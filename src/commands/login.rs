use std::collections::HashMap;

use anyhow::{Context, Result};
use async_trait::async_trait;
use frankenstein::{AsyncTelegramApi, types::Message};
use serde::Deserialize;

use crate::{commands::CommandType, state::SharedState};

#[derive(Deserialize)]
struct LoginReturn {
    token: String,
}

pub struct LoginCommand;

#[async_trait]
impl CommandType for LoginCommand {
    fn name(&self) -> &str {
        "login"
    }

    fn description(&self) -> &str {
        "Login to the bot"
    }

    fn usage(&self) -> &str {
        "/login"
    }

    fn aliases(&self) -> &[&str] {
        &["login"]
    }

    async fn run(&self, message: &Message, state: SharedState) -> Result<()> {
        let client = reqwest::Client::new();
        let mut map = HashMap::new();
        let telegram_id = message
            .from
            .as_ref()
            .map(|user| user.id)
            .context("Could not retrieve user")?;
        map.insert("telegram_id", telegram_id.to_string());
        let response = client
            .post("http://localhost:3000/login")
            .json(&map)
            .header("Content-Type", "application/json")
            .send()
            .await?;
        if response.status().is_success() {
            dbg!("Success");
            let body = response.text().await?;
            let response =
                serde_json::from_str::<LoginReturn>(&body).context("Could not parse response")?;
            let response_text = format!("Your login token:\n```\n{}\n```", response.token);
            let send_message_params = frankenstein::methods::SendMessageParams::builder()
                .chat_id(message.chat.id)
                .text(response_text)
                .parse_mode(frankenstein::ParseMode::MarkdownV2)
                .build();
            state.bot.send_message(&send_message_params).await?;
        } else {
            let message_builder = frankenstein::methods::SendMessageParams::builder()
                .chat_id(message.chat.id)
                .text("Failed to login")
                .build();
            state.bot.send_message(&message_builder).await?;
        }
        Ok(())
    }
}

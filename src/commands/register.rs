use anyhow::{Context, Result};
use async_trait::async_trait;
use frankenstein::{AsyncTelegramApi, types::Message};

use crate::{commands::CommandType, jwt_token::Claims, state::SharedState};

pub struct RegisterCommand;

#[async_trait]
impl CommandType for RegisterCommand {
    fn name(&self) -> &str {
        "register"
    }

    fn description(&self) -> &str {
        "Registers a new user"
    }

    fn usage(&self) -> &str {
        "/register <username>"
    }

    fn aliases(&self) -> &[&str] {
        &["reg", "signup"]
    }

    async fn run(&self, message: &Message, state: SharedState) -> Result<()> {
        let user_id = message
            .from
            .as_ref()
            .map(|user| user.id)
            .context("Could not retrieve user")?;

        let reply_text = Claims::from(user_id)
            .token()
            .map(|token| format!("Your registration token:\n```\n{}\n```", token))
            .context("Could not create token")?;

        let send_message_params = frankenstein::methods::SendMessageParams::builder()
            .chat_id(message.chat.id)
            .text(reply_text)
            .parse_mode(frankenstein::ParseMode::MarkdownV2)
            .build();

        state.bot.send_message(&send_message_params).await?;
        Ok(())
    }
}

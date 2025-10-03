use anyhow::Result;
use async_trait::async_trait;
use frankenstein::{AsyncTelegramApi, types::Message};

use crate::{commands::CommandType, state::SharedState};

pub struct StartCommand;

#[async_trait]
impl CommandType for StartCommand {
    fn name(&self) -> &str {
        "start"
    }

    fn description(&self) -> &str {
        "Starts the bot"
    }

    fn usage(&self) -> &str {
        "/start"
    }

    fn aliases(&self) -> &[&str] {
        &["begin", "init"]
    }

    async fn run(&self, message: &Message, state: SharedState) -> Result<()> {
        let send_message_params = frankenstein::methods::SendMessageParams::builder()
            .chat_id(message.chat.id)
            .text("Welcome! Use /help to see available commands.")
            .build();

        state.bot.send_message(&send_message_params).await?;
        Ok(())
    }
}

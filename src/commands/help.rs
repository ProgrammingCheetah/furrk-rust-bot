use anyhow::Result;
use async_trait::async_trait;
use frankenstein::{AsyncTelegramApi, types::Message};

use crate::{
    commands::{CommandType, get_all_commands},
    state::SharedState,
};

pub struct HelpCommand;

#[async_trait]
impl CommandType for HelpCommand {
    fn name(&self) -> &str {
        "help"
    }

    fn description(&self) -> &str {
        "Shows help information"
    }

    fn usage(&self) -> &str {
        "/help"
    }

    fn aliases(&self) -> &[&str] {
        &["h", "info"]
    }

    async fn run(&self, message: &Message, state: SharedState) -> Result<()> {
        let commands = get_all_commands()
            .iter()
            .map(|cmd| format!("/{} - {}", cmd.name(), cmd.description()))
            .collect::<Vec<_>>()
            .join("\n");

        let send_message_params = frankenstein::methods::SendMessageParams::builder()
            .chat_id(message.chat.id)
            .text(format!("Available Commands:\n\n{}", commands))
            .build();

        state.bot.send_message(&send_message_params).await?;
        Ok(())
    }
}

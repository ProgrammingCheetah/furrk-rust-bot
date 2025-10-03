use crate::commands::get_command;
use crate::state::{AppState, SharedState};
use anyhow::{Context, Result};
use dotenv;
use frankenstein::AsyncTelegramApi;
use frankenstein::client_reqwest::Bot;
use frankenstein::methods::GetUpdatesParams;
use frankenstein::types::{AllowedUpdate, ChatType, Message};
use frankenstein::updates::UpdateContent;
use std::sync::Arc;

mod commands;
mod database;
mod errors;
mod extractors;
mod jwt_token;
// mod models;
mod state;

#[tokio::main]
async fn main() {
    let token = dotenv::var("BOT_TOKEN").expect("BOT_TOKEN must be set in .env");
    let bot = Bot::new(&token);
    bot.set_my_commands(
        &frankenstein::methods::SetMyCommandsParams::builder()
            .commands(vec![
                frankenstein::types::BotCommand {
                    command: "start".to_string(),
                    description: "Start the bot".to_string(),
                },
                frankenstein::types::BotCommand {
                    command: "help".to_string(),
                    description: "Show help message".to_string(),
                },
                frankenstein::types::BotCommand {
                    command: "register".to_string(),
                    description: "Register an account".to_string(),
                },
            ])
            .build(),
    )
    .await
    .expect("Failed to set bot commands");
    dotenv::dotenv().ok();
    let mut update_params = GetUpdatesParams::builder()
        .allowed_updates(vec![AllowedUpdate::Message])
        .build();

    let state = Arc::new(AppState { bot });
    loop {
        let result = state.bot.get_updates(&update_params).await;
        match result {
            Ok(response) => {
                for update in response.result {
                    if let UpdateContent::Message(message) = update.content {
                        if message.chat.type_field != ChatType::Private {
                            continue;
                        }
                        let current_state = state.clone();
                        tokio::spawn(async move {
                            let _ = process_message(*message, current_state)
                                .await
                                .map_err(|e| eprintln!("Error processing message: {:?}", e));
                        });
                    }
                    update_params.offset = Some(i64::from(update.update_id) + 1);
                }
            }
            Err(e) => {
                eprintln!("Error getting updates: {:?}", e);
            }
        }
    }
}

async fn process_message(message: Message, state: SharedState) -> Result<()> {
    let command_text = message.text.clone().unwrap_or_default();
    if let Some(handler) = get_command(command_text.as_str()) {
        handler
            .run(&message, state)
            .await
            .context(format!("Failed to run command: {}", &command_text))?;
    }
    Ok(())
}

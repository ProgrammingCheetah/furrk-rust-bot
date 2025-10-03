use crate::{
    commands::{
        help::HelpCommand, login::LoginCommand, register::RegisterCommand, start::StartCommand,
    },
    state::SharedState,
};
use anyhow::Result;
use async_trait::async_trait;
use frankenstein::types::Message;

mod help;
mod login;
mod register;
mod start;

#[async_trait]
pub trait CommandType: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn usage(&self) -> &str;
    fn aliases(&self) -> &[&str];
    fn is_of_type(&self, command: &str) -> bool {
        command == self.name() || self.aliases().contains(&command)
    }
    async fn run(&self, message: &Message, state: SharedState) -> Result<()>;
}

pub fn get_all_commands() -> Vec<Box<dyn CommandType>> {
    vec![
        Box::new(StartCommand),
        Box::new(HelpCommand),
        Box::new(RegisterCommand), // Add other commands here
        Box::new(LoginCommand),
    ]
}

pub fn get_command(command: &str) -> Option<Box<dyn CommandType>> {
    let commands = get_all_commands();
    let parsed_command = command
        .split_whitespace()
        .next()
        .map(|s| s.strip_prefix('/').unwrap_or(s))
        .unwrap_or_default();

    for cmd in commands {
        if cmd.is_of_type(parsed_command) {
            return Some(cmd);
        }
    }
    None
}

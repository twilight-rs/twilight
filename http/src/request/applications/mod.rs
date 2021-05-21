use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

mod create_guild_command;
mod delete_guild_command;
mod get_guild_commands;
mod set_guild_commands;
mod update_guild_command;

mod create_global_command;
mod delete_global_command;
mod get_global_commands;
mod set_global_commands;
mod update_global_command;

mod delete_original_response;
mod interaction_callback;
mod update_original_response;

pub use self::create_guild_command::CreateGuildCommand;
pub use self::delete_guild_command::DeleteGuildCommand;
pub use self::get_guild_commands::GetGuildCommands;
pub use self::set_guild_commands::SetGuildCommands;
pub use self::update_guild_command::UpdateGuildCommand;

pub use self::create_global_command::CreateGlobalCommand;
pub use self::delete_global_command::DeleteGlobalCommand;
pub use self::get_global_commands::GetGlobalCommands;
pub use self::set_global_commands::SetGlobalCommands;
pub use self::update_global_command::UpdateGlobalCommand;

pub use self::delete_original_response::DeleteOriginalResponse;
pub use self::interaction_callback::InteractionCallback;
pub use self::update_original_response::UpdateOriginalResponse;

/// The error created if the creation of interaction fails.
#[derive(Debug)]
pub struct InteractionError {
    pub(crate) kind: InteractionErrorType,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum InteractionErrorType {
    ApplicationIdNotPresent,
}

impl InteractionError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub fn kind(&self) -> &InteractionErrorType {
        &self.kind
    }
}

impl Display for InteractionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            InteractionErrorType::ApplicationIdNotPresent => {
                f.write_str("application id not present")
            }
        }
    }
}

impl Error for InteractionError {}

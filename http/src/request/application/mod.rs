use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

mod create_guild_command;
mod delete_guild_command;
mod create_global_command;
mod delete_global_command;
mod delete_original_response;
mod get_global_commands;
mod get_guild_commands;
mod interaction_callback;
mod set_global_commands;
mod set_guild_commands;
mod update_global_command;
mod update_guild_command;
mod update_original_response;

pub use self::create_global_command::CreateGlobalCommand;
pub use self::create_guild_command::CreateGuildCommand;
pub use self::delete_global_command::DeleteGlobalCommand;
pub use self::delete_guild_command::DeleteGuildCommand;
pub use self::delete_original_response::DeleteOriginalResponse;
pub use self::get_global_commands::GetGlobalCommands;
pub use self::get_guild_commands::GetGuildCommands;
pub use self::interaction_callback::InteractionCallback;
pub use self::set_global_commands::SetGlobalCommands;
pub use self::set_guild_commands::SetGuildCommands;
pub use self::update_global_command::UpdateGlobalCommand;
pub use self::update_guild_command::UpdateGuildCommand;
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

    /// Consume the error, returning the source error if there is any.
    #[allow(clippy::unused_self)]
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        InteractionErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
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

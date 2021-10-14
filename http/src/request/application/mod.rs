pub mod command;
pub mod interaction;

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// The error created if the creation of interaction fails.
#[derive(Debug)]
pub struct InteractionError {
    pub(crate) kind: InteractionErrorType,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum InteractionErrorType {
    /// Application id was not set on the client.
    ApplicationIdNotPresent,
    /// Command name validation failed.
    CommandNameValidationFailed,
    /// Command description validation failed.
    CommandDescriptionValidationFailed,
    /// Required command options have to be passed before optional ones.
    CommandOptionsRequiredFirst {
        /// Index of the option that failed validation.
        index: usize,
    },
    /// More than 10 permission overwrites were set.
    TooManyCommandPermissions,
    /// Too many commands have been provided.
    ///
    /// The maximum number of commands is defined by
    /// [`InteractionError::GUILD_COMMAND_LIMIT`].
    TooManyCommands,
}

impl InteractionError {
    /// Maximum number of commands an application may have in an individual
    /// guild.
    pub const GUILD_COMMAND_LIMIT: usize = 100;

    /// Maximum number of permission overwrites an application may have in an
    /// individual guild command.
    pub const GUILD_COMMAND_PERMISSION_LIMIT: usize = 10;

    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &InteractionErrorType {
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
    pub fn into_parts(self) -> (InteractionErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, None)
    }
}

impl Display for InteractionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.kind {
            InteractionErrorType::ApplicationIdNotPresent => {
                f.write_str("application id not present")
            }
            InteractionErrorType::CommandNameValidationFailed => {
                f.write_str("command name must be between 3 and 32 characters")
            }
            InteractionErrorType::CommandDescriptionValidationFailed => {
                f.write_str("command description must be between 1 and 100 characters")
            }
            InteractionErrorType::CommandOptionsRequiredFirst { .. } => {
                f.write_str("optional command options must be added after required")
            }
            InteractionErrorType::TooManyCommandPermissions => {
                f.write_str("more than ")?;
                Display::fmt(&InteractionError::GUILD_COMMAND_PERMISSION_LIMIT, f)?;

                f.write_str(" permission overwrites were set")
            }
            InteractionErrorType::TooManyCommands => {
                f.write_str("more than ")?;
                Display::fmt(&InteractionError::GUILD_COMMAND_LIMIT, f)?;

                f.write_str(" commands were set")
            }
        }
    }
}

impl Error for InteractionError {}

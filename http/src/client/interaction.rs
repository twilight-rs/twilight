use crate::request::application::{
    command::{
        CreateGlobalCommand, CreateGuildCommand, DeleteGlobalCommand, DeleteGuildCommand,
        GetCommandPermissions, GetGlobalCommand, GetGlobalCommands, GetGuildCommand,
        GetGuildCommandPermissions, GetGuildCommands, SetCommandPermissions, SetGlobalCommands,
        SetGuildCommands, UpdateCommandPermissions, UpdateGlobalCommand, UpdateGuildCommand,
    },
    interaction::{
        CreateFollowupMessage, DeleteFollowupMessage, DeleteOriginalResponse, GetFollowupMessage,
        GetOriginalResponse, InteractionCallback, UpdateFollowupMessage, UpdateOriginalResponse,
    },
    InteractionError,
};

use super::Client;
use twilight_model::{
    application::{
        callback::InteractionResponse,
        command::{permissions::CommandPermissions, Command},
    },
    id::{ApplicationId, CommandId, GuildId, InteractionId, MessageId},
};

/// Client interface for using interactions.
///
/// # Examples
///
/// Retrieve the application ID and then use an interaction request:
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<std::error::Error>> {
/// use std::env;
/// use twilight_http::Client;
/// use twilight_model::id::ApplicationId;
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let application_id = ApplicationId::new(123).expect("non zero id");
///
/// let interaction_client = client.interaction(application_id);
///
/// let commands = interaction_client
///     .get_global_commands()
///     .exec()
///     .await?
///     .models()
///     .await?;
///
/// println!("there are {} global commands", commands.len());
/// # Ok(()) }
/// ```
#[derive(Debug)]
pub struct InteractionClient<'a> {
    application_id: ApplicationId,
    client: &'a Client,
}

impl<'a> InteractionClient<'a> {
    /// Create a new interface for using interactions.
    pub(super) const fn new(client: &'a Client, application_id: ApplicationId) -> Self {
        Self {
            application_id,
            client,
        }
    }

    /// Respond to an interaction, by ID and token.
    ///
    /// For variants of [`InteractionResponse`] that contain a [`CallbackData`],
    /// there is an [associated builder] in the [`twilight-util`] crate.
    ///
    /// [`CallbackData`]: twilight_model::application::callback::CallbackData
    /// [`twilight-util`]: https://docs.rs/twilight-util/latest/index.html
    /// [associated builder]: https://docs.rs/twilight-util/latest/builder/struct.CallbackDataBuilder.html
    pub const fn interaction_callback(
        &'a self,
        interaction_id: InteractionId,
        interaction_token: &'a str,
        response: &'a InteractionResponse,
    ) -> InteractionCallback<'a> {
        InteractionCallback::new(self.client, interaction_id, interaction_token, response)
    }

    /// Get the original message, by its token.
    pub const fn get_interaction_original(
        &'a self,
        interaction_token: &'a str,
    ) -> GetOriginalResponse<'a> {
        GetOriginalResponse::new(self.client, self.application_id, interaction_token)
    }

    /// Edit the original message, by its token.
    pub const fn update_interaction_original(
        &'a self,
        interaction_token: &'a str,
    ) -> UpdateOriginalResponse<'a> {
        UpdateOriginalResponse::new(self.client, self.application_id, interaction_token)
    }

    /// Get a followup message of an interaction.
    pub const fn followup_message(
        &'a self,
        interaction_token: &'a str,
        message_id: MessageId,
    ) -> GetFollowupMessage<'a> {
        GetFollowupMessage::new(
            self.client,
            self.application_id,
            interaction_token,
            message_id,
        )
    }

    /// Delete the original message, by its token.
    pub const fn delete_interaction_original(
        &'a self,
        interaction_token: &'a str,
    ) -> DeleteOriginalResponse<'a> {
        DeleteOriginalResponse::new(self.client, self.application_id, interaction_token)
    }

    /// Create a followup message, by an interaction token.
    pub const fn create_followup_message(
        &'a self,
        interaction_token: &'a str,
    ) -> CreateFollowupMessage<'a> {
        CreateFollowupMessage::new(self.client, self.application_id, interaction_token)
    }

    /// Edit a followup message, by an interaction token.
    pub const fn update_followup_message(
        &'a self,
        interaction_token: &'a str,
        message_id: MessageId,
    ) -> UpdateFollowupMessage<'a> {
        UpdateFollowupMessage::new(
            self.client,
            self.application_id,
            interaction_token,
            message_id,
        )
    }

    /// Delete a followup message by interaction token and the message's ID.
    pub const fn delete_followup_message(
        &'a self,
        interaction_token: &'a str,
        message_id: MessageId,
    ) -> DeleteFollowupMessage<'a> {
        DeleteFollowupMessage::new(
            self.client,
            self.application_id,
            interaction_token,
            message_id,
        )
    }

    /// Create a new command in a guild.
    ///
    /// The name must be between 1 and 32 characters in length. Creating a
    /// guild command with the same name as an already-existing guild command in
    /// the same guild will overwrite the old command. See [the discord docs]
    /// for more information.
    ///
    /// Returns an [`InteractionErrorType::CommandNameValidationFailed`]
    /// error type if the command name is not between 1 and 32 characters.
    ///
    ///
    /// [`InteractionErrorType::CommandNameValidationFailed`]: crate::request::application::InteractionErrorType::CommandNameValidationFailed
    /// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#create-guild-application-command
    pub fn create_guild_command(
        &'a self,
        guild_id: GuildId,
        name: &'a str,
    ) -> Result<CreateGuildCommand<'a>, InteractionError> {
        CreateGuildCommand::new(self.client, self.application_id, guild_id, name)
    }

    /// Fetch a guild command for your application.
    pub const fn get_guild_command(
        &self,
        guild_id: GuildId,
        command_id: CommandId,
    ) -> GetGuildCommand<'_> {
        GetGuildCommand::new(self.client, self.application_id, guild_id, command_id)
    }

    /// Fetch all commands for a guild, by ID.
    pub const fn get_guild_commands(&self, guild_id: GuildId) -> GetGuildCommands<'_> {
        GetGuildCommands::new(self.client, self.application_id, guild_id)
    }

    /// Edit a command in a guild, by ID.
    ///
    /// You must specify a name and description. See [the discord docs] for more
    /// information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#edit-guild-application-command
    pub const fn update_guild_command(
        &self,
        guild_id: GuildId,
        command_id: CommandId,
    ) -> UpdateGuildCommand<'_> {
        UpdateGuildCommand::new(self.client, self.application_id, guild_id, command_id)
    }

    /// Delete a command in a guild, by ID.
    pub const fn delete_guild_command(
        &self,
        guild_id: GuildId,
        command_id: CommandId,
    ) -> DeleteGuildCommand<'_> {
        DeleteGuildCommand::new(self.client, self.application_id, guild_id, command_id)
    }

    /// Set a guild's commands.
    ///
    /// This method is idempotent: it can be used on every start, without being
    /// ratelimited if there aren't changes to the commands.
    ///
    /// The [`Command`] struct has an [associated builder] in the
    /// [`twilight-util`] crate.
    ///
    /// [`twilight-util`]: https://docs.rs/twilight_util/index.html
    /// [associated builder]: https://docs.rs/twilight-util/latest/builder/command/struct.CommandBuilder.html
    pub const fn set_guild_commands(
        &'a self,
        guild_id: GuildId,
        commands: &'a [Command],
    ) -> SetGuildCommands<'a> {
        SetGuildCommands::new(self.client, self.application_id, guild_id, commands)
    }

    /// Create a new global command.
    ///
    /// The name must be between 1 and 32 characters in length. Creating a
    /// command with the same name as an already-existing global command will
    /// overwrite the old command. See [the discord docs] for more information.
    ///
    /// Returns an [`InteractionErrorType::CommandNameValidationFailed`]
    /// error type if the command name is not between 1 and 32 characters.
    ///
    /// [`InteractionErrorType::CommandNameValidationFailed`]: crate::request::application::InteractionErrorType::CommandNameValidationFailed
    /// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#create-global-application-command
    pub fn create_global_command(
        &'a self,
        name: &'a str,
    ) -> Result<CreateGlobalCommand<'a>, InteractionError> {
        CreateGlobalCommand::new(self.client, self.application_id, name)
    }

    /// Fetch a global command for your application.
    pub const fn get_global_command(&self, command_id: CommandId) -> GetGlobalCommand<'_> {
        GetGlobalCommand::new(self.client, self.application_id, command_id)
    }

    /// Fetch all global commands for your application.
    pub const fn get_global_commands(&self) -> GetGlobalCommands<'_> {
        GetGlobalCommands::new(self.client, self.application_id)
    }

    /// Edit a global command, by ID.
    ///
    /// You must specify a name and description. See [the discord docs] for more
    /// information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/interactions/application-commands#edit-global-application-command
    pub const fn update_global_command(&self, command_id: CommandId) -> UpdateGlobalCommand<'_> {
        UpdateGlobalCommand::new(self.client, self.application_id, command_id)
    }

    /// Delete a global command, by ID.
    pub const fn delete_global_command(&self, command_id: CommandId) -> DeleteGlobalCommand<'_> {
        DeleteGlobalCommand::new(self.client, self.application_id, command_id)
    }

    /// Set global commands.
    ///
    /// This method is idempotent: it can be used on every start, without being
    /// ratelimited if there aren't changes to the commands.
    ///
    /// The [`Command`] struct has an [associated builder] in the
    /// [`twilight-util`] crate.
    ///
    /// [`twilight-util`]: https://docs.rs/twilight-util/latest/index.html
    /// [associated builder]: https://docs.rs/twilight-util/latest/builder/command/struct.CommandBuilder.html
    pub const fn set_global_commands(&'a self, commands: &'a [Command]) -> SetGlobalCommands<'a> {
        SetGlobalCommands::new(self.client, self.application_id, commands)
    }

    /// Fetch command permissions for a command from the current application
    /// in a guild.
    pub const fn get_command_permissions(
        &self,
        guild_id: GuildId,
        command_id: CommandId,
    ) -> GetCommandPermissions<'_> {
        GetCommandPermissions::new(self.client, self.application_id, guild_id, command_id)
    }

    /// Fetch command permissions for all commands from the current
    /// application in a guild.
    pub const fn get_guild_command_permissions(
        &self,
        guild_id: GuildId,
    ) -> GetGuildCommandPermissions<'_> {
        GetGuildCommandPermissions::new(self.client, self.application_id, guild_id)
    }

    /// Update command permissions for a single command in a guild.
    ///
    /// This overwrites the command permissions so the full set of permissions
    /// have to be sent every time.
    pub const fn update_command_permissions(
        &'a self,
        guild_id: GuildId,
        command_id: CommandId,
        permissions: &'a [CommandPermissions],
    ) -> Result<UpdateCommandPermissions<'a>, InteractionError> {
        UpdateCommandPermissions::new(
            self.client,
            self.application_id,
            guild_id,
            command_id,
            permissions,
        )
    }

    /// Update command permissions for all commands in a guild.
    ///
    /// This overwrites the command permissions so the full set of permissions
    /// have to be sent every time.
    ///
    /// Returns an [`InteractionErrorType::TooManyCommands`] error type if too
    /// many commands have been provided. The maximum amount is defined by
    /// [`InteractionError::GUILD_COMMAND_LIMIT`].
    ///
    /// [`InteractionErrorType::TooManyCommands`]: crate::request::application::InteractionErrorType::TooManyCommands
    /// [`InteractionError::GUILD_COMMAND_LIMIT`]: crate::request::application::InteractionError::GUILD_COMMAND_LIMIT
    pub fn set_command_permissions(
        &'a self,
        guild_id: GuildId,
        permissions: &'a [(CommandId, CommandPermissions)],
    ) -> Result<SetCommandPermissions<'a>, InteractionError> {
        SetCommandPermissions::new(self.client, self.application_id, guild_id, permissions)
    }
}

#[cfg(test)]
mod tests {
    use super::InteractionClient;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(InteractionClient<'_>: Debug, Send, Sync);
}

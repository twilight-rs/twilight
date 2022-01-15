use crate::{
    request::application::{
        command::{
            CreateGlobalCommand, CreateGuildCommand, DeleteGlobalCommand, DeleteGuildCommand,
            GetCommandPermissions, GetGlobalCommand, GetGlobalCommands, GetGuildCommand,
            GetGuildCommandPermissions, GetGuildCommands, SetCommandPermissions, SetGlobalCommands,
            SetGuildCommands, UpdateCommandPermissions, UpdateGlobalCommand, UpdateGuildCommand,
        },
        interaction::{
            CreateFollowupMessage, DeleteFollowupMessage, DeleteOriginalResponse,
            GetFollowupMessage, GetOriginalResponse, InteractionCallback, UpdateFollowupMessage,
            UpdateOriginalResponse,
        },
    },
    Client,
};
use twilight_model::{
    application::{
        callback::InteractionResponse,
        command::{permissions::CommandPermissions, Command},
    },
    id::{
        marker::{ApplicationMarker, CommandMarker, GuildMarker, InteractionMarker, MessageMarker},
        Id,
    },
};
use twilight_validate::command::CommandValidationError;

/// Client interface for using interactions.
///
/// # Examples
///
/// Retrieve the application ID and then use an interaction request:
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::env;
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let application_id = Id::new(123);
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
    application_id: Id<ApplicationMarker>,
    client: &'a Client,
}

impl<'a> InteractionClient<'a> {
    /// Create a new interface for using interactions.
    pub(super) const fn new(client: &'a Client, application_id: Id<ApplicationMarker>) -> Self {
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
        interaction_id: Id<InteractionMarker>,
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
        message_id: Id<MessageMarker>,
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

    /// Create a followup message to an interaction.
    ///
    /// The message must include at least one of [`attachments`], [`content`],
    /// or [`embeds`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use std::env;
    /// use twilight_http::Client;
    /// use twilight_model::id::Id;
    ///
    /// let client = Client::new(env::var("DISCORD_TOKEN")?);
    /// let application_id = Id::new(1);
    ///
    /// client
    ///     .interaction(application_id)
    ///     .create_followup_message("webhook token")
    ///     .content("Pinkie...")?
    ///     .exec()
    ///     .await?;
    /// # Ok(()) }
    /// ```
    ///
    /// [`attachments`]: CreateFollowupMessage::attachments
    /// [`content`]: CreateFollowupMessage::content
    /// [`embeds`]: CreateFollowupMessage::embeds
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
        message_id: Id<MessageMarker>,
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
        message_id: Id<MessageMarker>,
    ) -> DeleteFollowupMessage<'a> {
        DeleteFollowupMessage::new(
            self.client,
            self.application_id,
            interaction_token,
            message_id,
        )
    }

    /// Create a new command in a guild.
    pub const fn create_guild_command(
        &'a self,
        guild_id: Id<GuildMarker>,
    ) -> CreateGuildCommand<'a> {
        CreateGuildCommand::new(self.client, self.application_id, guild_id)
    }

    /// Fetch a guild command for your application.
    pub const fn get_guild_command(
        &self,
        guild_id: Id<GuildMarker>,
        command_id: Id<CommandMarker>,
    ) -> GetGuildCommand<'_> {
        GetGuildCommand::new(self.client, self.application_id, guild_id, command_id)
    }

    /// Fetch all commands for a guild, by ID.
    pub const fn get_guild_commands(&self, guild_id: Id<GuildMarker>) -> GetGuildCommands<'_> {
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
        guild_id: Id<GuildMarker>,
        command_id: Id<CommandMarker>,
    ) -> UpdateGuildCommand<'_> {
        UpdateGuildCommand::new(self.client, self.application_id, guild_id, command_id)
    }

    /// Delete a command in a guild, by ID.
    pub const fn delete_guild_command(
        &self,
        guild_id: Id<GuildMarker>,
        command_id: Id<CommandMarker>,
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
        guild_id: Id<GuildMarker>,
        commands: &'a [Command],
    ) -> SetGuildCommands<'a> {
        SetGuildCommands::new(self.client, self.application_id, guild_id, commands)
    }

    /// Create a new global command.
    pub const fn create_global_command(&'a self) -> CreateGlobalCommand<'a> {
        CreateGlobalCommand::new(self.client, self.application_id)
    }

    /// Fetch a global command for your application.
    pub const fn get_global_command(&self, command_id: Id<CommandMarker>) -> GetGlobalCommand<'_> {
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
    pub const fn update_global_command(
        &self,
        command_id: Id<CommandMarker>,
    ) -> UpdateGlobalCommand<'_> {
        UpdateGlobalCommand::new(self.client, self.application_id, command_id)
    }

    /// Delete a global command, by ID.
    pub const fn delete_global_command(
        &self,
        command_id: Id<CommandMarker>,
    ) -> DeleteGlobalCommand<'_> {
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
        guild_id: Id<GuildMarker>,
        command_id: Id<CommandMarker>,
    ) -> GetCommandPermissions<'_> {
        GetCommandPermissions::new(self.client, self.application_id, guild_id, command_id)
    }

    /// Fetch command permissions for all commands from the current
    /// application in a guild.
    pub const fn get_guild_command_permissions(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> GetGuildCommandPermissions<'_> {
        GetGuildCommandPermissions::new(self.client, self.application_id, guild_id)
    }

    /// Update command permissions for a single command in a guild.
    ///
    /// This overwrites the command permissions so the full set of permissions
    /// have to be sent every time.
    pub fn update_command_permissions(
        &'a self,
        guild_id: Id<GuildMarker>,
        command_id: Id<CommandMarker>,
        permissions: &'a [CommandPermissions],
    ) -> Result<UpdateCommandPermissions<'a>, CommandValidationError> {
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
    /// Returns an error of type [`CountInvalid`] if too many commands have been
    /// provided. The maximum amount is defined by [`GUILD_COMMAND_LIMIT`].
    ///
    /// [`CountInvalid`]: twilight_validate::command::CommandValidationErrorType::CountInvalid
    /// [`GUILD_COMMAND_LIMIT`]: twilight_validate::command::GUILD_COMMAND_LIMIT
    pub fn set_command_permissions(
        &'a self,
        guild_id: Id<GuildMarker>,
        permissions: &'a [(Id<CommandMarker>, CommandPermissions)],
    ) -> Result<SetCommandPermissions<'a>, CommandValidationError> {
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

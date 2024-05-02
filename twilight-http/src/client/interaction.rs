use crate::{
    request::application::{
        command::{
            CreateGlobalCommand, CreateGuildCommand, DeleteGlobalCommand, DeleteGuildCommand,
            GetCommandPermissions, GetGlobalCommand, GetGlobalCommands, GetGuildCommand,
            GetGuildCommandPermissions, GetGuildCommands, SetGlobalCommands, SetGuildCommands,
            UpdateCommandPermissions, UpdateGlobalCommand, UpdateGuildCommand,
        },
        interaction::{
            CreateFollowup, CreateResponse, DeleteFollowup, DeleteResponse, GetFollowup,
            GetResponse, UpdateFollowup, UpdateResponse,
        },
    },
    Client,
};
use twilight_model::{
    application::command::{permissions::CommandPermission, Command},
    http::interaction::InteractionResponse,
    id::{
        marker::{ApplicationMarker, CommandMarker, GuildMarker, InteractionMarker, MessageMarker},
        Id,
    },
};

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
/// let commands = interaction_client.global_commands().await?.models().await?;
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

    /// Respond to an interaction, by its ID and token.
    ///
    /// For variants of [`InteractionResponse`] that contain
    /// [`InteractionResponseData`], there is an [associated builder] in the
    /// [`twilight-util`] crate.
    ///
    /// This endpoint is not bound to the application's global rate limit.
    ///
    /// [`InteractionResponseData`]: twilight_model::http::interaction::InteractionResponseData
    /// [`twilight-util`]: https://docs.rs/twilight-util/latest/index.html
    /// [associated builder]: https://docs.rs/twilight-util/latest/twilight_util/builder/struct.InteractionResponseDataBuilder.html
    pub const fn create_response(
        &'a self,
        interaction_id: Id<InteractionMarker>,
        interaction_token: &'a str,
        response: &'a InteractionResponse,
    ) -> CreateResponse<'a> {
        CreateResponse::new(self.client, interaction_id, interaction_token, response)
    }

    /// Delete the original message, by its token.
    ///
    /// This endpoint is not bound to the application's global rate limit.
    pub const fn delete_response(&'a self, interaction_token: &'a str) -> DeleteResponse<'a> {
        DeleteResponse::new(self.client, self.application_id, interaction_token)
    }

    /// Get the original message, by its token.
    ///
    /// This endpoint is not bound to the application's global rate limit.
    pub const fn response(&'a self, interaction_token: &'a str) -> GetResponse<'a> {
        GetResponse::new(self.client, self.application_id, interaction_token)
    }

    /// Edit the original message, by its token.
    ///
    /// The update must include at least one of [`attachments`], [`components`],
    /// [`content`] or [`embeds`].
    ///
    /// This endpoint is not bound to the application's global rate limit.
    ///
    /// [`attachments`]: CreateFollowup::attachments
    /// [`components`]: CreateFollowup::components
    /// [`content`]: CreateFollowup::content
    /// [`embeds`]: CreateFollowup::embeds
    pub const fn update_response(&'a self, interaction_token: &'a str) -> UpdateResponse<'a> {
        UpdateResponse::new(self.client, self.application_id, interaction_token)
    }

    /// Create a followup message to an interaction, by its token.
    ///
    /// The message must include at least one of [`attachments`], [`components`]
    /// [`content`] or [`embeds`].
    ///
    /// This endpoint is not bound to the application's global rate limit.
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
    ///     .create_followup("webhook token")
    ///     .content("Pinkie...")
    ///     .await?;
    /// # Ok(()) }
    /// ```
    ///
    /// [`attachments`]: CreateFollowup::attachments
    /// [`components`]: CreateFollowup::components
    /// [`content`]: CreateFollowup::content
    /// [`embeds`]: CreateFollowup::embeds
    pub const fn create_followup(&'a self, interaction_token: &'a str) -> CreateFollowup<'a> {
        CreateFollowup::new(self.client, self.application_id, interaction_token)
    }

    /// Delete a followup message to an interaction, by its token and message
    /// ID.
    ///
    /// This endpoint is not bound to the application's global rate limit.
    pub const fn delete_followup(
        &'a self,
        interaction_token: &'a str,
        message_id: Id<MessageMarker>,
    ) -> DeleteFollowup<'a> {
        DeleteFollowup::new(
            self.client,
            self.application_id,
            interaction_token,
            message_id,
        )
    }

    /// Get a followup message of an interaction, by its token and the message
    /// ID.
    ///
    /// This endpoint is not bound to the application's global rate limit.
    pub const fn followup(
        &'a self,
        interaction_token: &'a str,
        message_id: Id<MessageMarker>,
    ) -> GetFollowup<'a> {
        GetFollowup::new(
            self.client,
            self.application_id,
            interaction_token,
            message_id,
        )
    }

    /// Edit a followup message of an interaction, by its token and the message
    /// ID.
    ///
    /// This endpoint is not bound to the application's global rate limit.
    pub const fn update_followup(
        &'a self,
        interaction_token: &'a str,
        message_id: Id<MessageMarker>,
    ) -> UpdateFollowup<'a> {
        UpdateFollowup::new(
            self.client,
            self.application_id,
            interaction_token,
            message_id,
        )
    }

    /// Create a new global command.
    pub const fn create_global_command(&'a self) -> CreateGlobalCommand<'a> {
        CreateGlobalCommand::new(self.client, self.application_id)
    }

    /// Delete a global command, by ID.
    pub const fn delete_global_command(
        &self,
        command_id: Id<CommandMarker>,
    ) -> DeleteGlobalCommand<'_> {
        DeleteGlobalCommand::new(self.client, self.application_id, command_id)
    }

    /// Fetch a global command for your application.
    pub const fn global_command(&self, command_id: Id<CommandMarker>) -> GetGlobalCommand<'_> {
        GetGlobalCommand::new(self.client, self.application_id, command_id)
    }

    /// Fetch all global commands for your application.
    pub const fn global_commands(&self) -> GetGlobalCommands<'_> {
        GetGlobalCommands::new(self.client, self.application_id)
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
    /// [associated builder]: https://docs.rs/twilight-util/latest/twilight_util/builder/command/struct.CommandBuilder.html
    pub const fn set_global_commands(&'a self, commands: &'a [Command]) -> SetGlobalCommands<'a> {
        SetGlobalCommands::new(self.client, self.application_id, commands)
    }

    /// Edit a global command, by ID.
    ///
    /// You must specify a name and description. See
    /// [Discord Docs/Edit Global Application Command].
    ///
    /// [Discord Docs/Edit Global Application Command]: https://discord.com/developers/docs/interactions/application-commands#edit-global-application-command
    pub const fn update_global_command(
        &self,
        command_id: Id<CommandMarker>,
    ) -> UpdateGlobalCommand<'_> {
        UpdateGlobalCommand::new(self.client, self.application_id, command_id)
    }

    /// Create a new command in a guild.
    pub const fn create_guild_command(
        &'a self,
        guild_id: Id<GuildMarker>,
    ) -> CreateGuildCommand<'a> {
        CreateGuildCommand::new(self.client, self.application_id, guild_id)
    }

    /// Delete a command in a guild, by ID.
    pub const fn delete_guild_command(
        &self,
        guild_id: Id<GuildMarker>,
        command_id: Id<CommandMarker>,
    ) -> DeleteGuildCommand<'_> {
        DeleteGuildCommand::new(self.client, self.application_id, guild_id, command_id)
    }

    /// Fetch a guild command for your application.
    pub const fn guild_command(
        &self,
        guild_id: Id<GuildMarker>,
        command_id: Id<CommandMarker>,
    ) -> GetGuildCommand<'_> {
        GetGuildCommand::new(self.client, self.application_id, guild_id, command_id)
    }

    /// Fetch all commands for a guild, by ID.
    pub const fn guild_commands(&self, guild_id: Id<GuildMarker>) -> GetGuildCommands<'_> {
        GetGuildCommands::new(self.client, self.application_id, guild_id)
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
    /// [associated builder]: https://docs.rs/twilight-util/latest/twilight_util/builder/command/struct.CommandBuilder.html
    pub const fn set_guild_commands(
        &'a self,
        guild_id: Id<GuildMarker>,
        commands: &'a [Command],
    ) -> SetGuildCommands<'a> {
        SetGuildCommands::new(self.client, self.application_id, guild_id, commands)
    }

    /// Edit a command in a guild, by ID.
    ///
    /// You must specify a name and description. See
    /// [Discord Docs/Edit Guild Application Command].
    ///
    /// [Discord Docs/Edit Guild Application Command]: https://discord.com/developers/docs/interactions/application-commands#edit-guild-application-command
    pub const fn update_guild_command(
        &self,
        guild_id: Id<GuildMarker>,
        command_id: Id<CommandMarker>,
    ) -> UpdateGuildCommand<'_> {
        UpdateGuildCommand::new(self.client, self.application_id, guild_id, command_id)
    }

    /// Fetch command permissions for a command from the current application
    /// in a guild.
    pub const fn command_permissions(
        &self,
        guild_id: Id<GuildMarker>,
        command_id: Id<CommandMarker>,
    ) -> GetCommandPermissions<'_> {
        GetCommandPermissions::new(self.client, self.application_id, guild_id, command_id)
    }

    /// Fetch command permissions for all commands from the current
    /// application in a guild.
    pub const fn guild_command_permissions(
        &self,
        guild_id: Id<GuildMarker>,
    ) -> GetGuildCommandPermissions<'_> {
        GetGuildCommandPermissions::new(self.client, self.application_id, guild_id)
    }

    /// Update command permissions for a single command in a guild.
    ///
    /// This overwrites the command permissions so the full set of permissions
    /// have to be sent every time.
    ///
    /// This request requires that the client was configured with an OAuth2 Bearer
    /// token.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`PermissionsCountInvalid`] if the permissions
    /// are invalid.
    ///
    /// [`PermissionsCountInvalid`]: twilight_validate::command::CommandValidationErrorType::PermissionsCountInvalid
    pub fn update_command_permissions(
        &'a self,
        guild_id: Id<GuildMarker>,
        command_id: Id<CommandMarker>,
        permissions: &'a [CommandPermission],
    ) -> UpdateCommandPermissions<'a> {
        UpdateCommandPermissions::new(
            self.client,
            self.application_id,
            guild_id,
            command_id,
            permissions,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::InteractionClient;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(InteractionClient<'_>: Debug, Send, Sync);
}

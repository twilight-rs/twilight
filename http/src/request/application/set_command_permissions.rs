use crate::{
    client::Client,
    error::Error,
    request::{Pending, Request},
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    application::command::permissions::CommandPermissions,
    id::{ApplicationId, CommandId, GuildId},
};

#[derive(Serialize)]
struct PartialGuildCommandPermissions {
    id: CommandId,
    permissions: CommandPermissions,
}

/// Update command permissions for all commands in a guild.
///
/// This overwrites the command permissions so the full set of permissions
/// have to be sent every time.
pub struct SetCommandPermissions<'a> {
    application_id: ApplicationId,
    guild_id: GuildId,
    fields: Vec<PartialGuildCommandPermissions>,
    fut: Option<Pending<'a, CommandPermissions>>,
    http: &'a Client,
}

impl<'a> SetCommandPermissions<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: ApplicationId,
        guild_id: GuildId,
        permissions: impl Iterator<Item = (CommandId, CommandPermissions)>,
    ) -> Self {
        let fields = permissions
            .map(
                |(command_id, command_permissions)| PartialGuildCommandPermissions {
                    id: command_id,
                    permissions: command_permissions,
                },
            )
            .collect();
        Self {
            application_id,
            guild_id,
            fields,
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::builder(Route::SetCommandPermissions {
            application_id: self.application_id.0,
            guild_id: self.guild_id.0,
        })
        .json(&self.fields)?;

        self.fut
            .replace(Box::pin(self.http.request(request.build())));

        Ok(())
    }
}

poll_req!(SetCommandPermissions<'_>, CommandPermissions);

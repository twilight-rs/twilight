use crate::{
    client::Client,
    error::Error,
    request::{
        application::{InteractionError, InteractionErrorType},
        validate, Pending, Request,
    },
    routing::Route,
};
use serde::Serialize;
use std::collections::HashMap;
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
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
}

impl<'a> SetCommandPermissions<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: ApplicationId,
        guild_id: GuildId,
        permissions: impl Iterator<Item = (CommandId, CommandPermissions)>,
    ) -> Result<Self, InteractionError> {
        let fields = permissions
            .map(
                |(command_id, command_permissions)| PartialGuildCommandPermissions {
                    id: command_id,
                    permissions: command_permissions,
                },
            )
            .collect::<Vec<PartialGuildCommandPermissions>>();

        if !fields
            .iter()
            .fold(HashMap::new(), |mut acc, permission| {
                acc.entry(permission.id)
                    .and_modify(|p| *p += 1)
                    .or_insert(1_usize);

                acc
            })
            .iter()
            .all(|permission| validate::command_permissions(*permission.1))
        {
            return Err(InteractionError {
                kind: InteractionErrorType::TooManyCommandPermissions,
            });
        }

        Ok(Self {
            application_id,
            guild_id,
            fields,
            fut: None,
            http,
        })
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::builder(Route::SetCommandPermissions {
            application_id: self.application_id.0,
            guild_id: self.guild_id.0,
        })
        .json(&self.fields)?;

        self.fut
            .replace(Box::pin(self.http.verify(request.build())));

        Ok(())
    }
}

poll_req!(SetCommandPermissions<'_>, ());

#[cfg(test)]
mod tests {
    use super::SetCommandPermissions;
    use crate::Client;
    use twilight_model::{
        application::command::permissions::{CommandPermissions, CommandPermissionsType},
        id::{ApplicationId, CommandId, GuildId, RoleId},
    };

    #[test]
    fn test_validation() {
        let http = Client::new("token");

        let permissions = vec![
            (
                CommandId(3),
                CommandPermissions {
                    id: CommandPermissionsType::Role(RoleId(4)),
                    permission: true,
                },
            ),
            (
                CommandId(3),
                CommandPermissions {
                    id: CommandPermissionsType::Role(RoleId(4)),
                    permission: true,
                },
            ),
            (
                CommandId(3),
                CommandPermissions {
                    id: CommandPermissionsType::Role(RoleId(4)),
                    permission: true,
                },
            ),
            (
                CommandId(3),
                CommandPermissions {
                    id: CommandPermissionsType::Role(RoleId(4)),
                    permission: true,
                },
            ),
            (
                CommandId(3),
                CommandPermissions {
                    id: CommandPermissionsType::Role(RoleId(4)),
                    permission: true,
                },
            ),
            (
                CommandId(3),
                CommandPermissions {
                    id: CommandPermissionsType::Role(RoleId(4)),
                    permission: true,
                },
            ),
            (
                CommandId(3),
                CommandPermissions {
                    id: CommandPermissionsType::Role(RoleId(4)),
                    permission: true,
                },
            ),
            (
                CommandId(3),
                CommandPermissions {
                    id: CommandPermissionsType::Role(RoleId(4)),
                    permission: true,
                },
            ),
            (
                CommandId(3),
                CommandPermissions {
                    id: CommandPermissionsType::Role(RoleId(4)),
                    permission: true,
                },
            ),
            (
                CommandId(3),
                CommandPermissions {
                    id: CommandPermissionsType::Role(RoleId(4)),
                    permission: true,
                },
            ),
            (
                CommandId(3),
                CommandPermissions {
                    id: CommandPermissionsType::Role(RoleId(4)),
                    permission: true,
                },
            ),
            (
                CommandId(3),
                CommandPermissions {
                    id: CommandPermissionsType::Role(RoleId(4)),
                    permission: true,
                },
            ),
        ];

        let request = SetCommandPermissions::new(
            &http,
            ApplicationId(1),
            GuildId(2),
            permissions.into_iter(),
        );

        assert!(request.is_err());
    }
}

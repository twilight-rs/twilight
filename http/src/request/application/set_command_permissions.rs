use crate::{
    client::Client,
    error::Error,
    request::{
        application::{InteractionError, InteractionErrorType},
        validate, Request, RequestBuilder,
    },
    response::ResponseFuture,
    routing::Route,
};
use serde::{ser::SerializeSeq, Serialize, Serializer};
use twilight_model::{
    application::command::permissions::CommandPermissions,
    id::{ApplicationId, CommandId, GuildId},
};

#[derive(Clone, Copy, Debug, Default)]
struct OptionalCommandPermissions<'a>(
    [Option<&'a CommandPermissions>; InteractionError::GUILD_COMMAND_PERMISSION_LIMIT],
);

impl OptionalCommandPermissions<'_> {
    /// Determine the number of elements present.
    ///
    /// If all elements are present then
    /// [`InteractionError::GUILD_COMMAND_PERMISSION_LIMIT`] is returned.
    ///
    /// If no elements are present then 0 is returned.
    fn amount_present(&self) -> usize {
        // Iterate over the elements until we find one that is None. If we don't,
        // then the maximum number are present.
        self.0
            .iter()
            .position(Option::is_none)
            .unwrap_or(InteractionError::GUILD_COMMAND_PERMISSION_LIMIT)
    }
}

impl Serialize for OptionalCommandPermissions<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.amount_present()))?;

        for maybe_value in self.0 {
            if let Some(value) = maybe_value.as_ref() {
                seq.serialize_element(value)?;
            } else {
                // If an element isn't present then any trailing elements aren't
                // either.
                break;
            }
        }

        seq.end()
    }
}
#[derive(Clone, Copy, Debug, Serialize)]
struct SortedCommand<'a> {
    #[serde(skip_serializing)]
    count: u8,
    id: CommandId,
    permissions: OptionalCommandPermissions<'a>,
}

impl SortedCommand<'_> {
    // Retrieve the current count as a usize for indexing.
    const fn count(self) -> usize {
        self.count as usize
    }
}

impl Default for SortedCommand<'_> {
    fn default() -> Self {
        Self {
            count: 0,
            id: CommandId(u64::MAX),
            permissions: OptionalCommandPermissions::default(),
        }
    }
}

#[derive(Debug)]
struct SortedCommands<'a> {
    inner: [SortedCommand<'a>; InteractionError::GUILD_COMMAND_LIMIT],
}

impl<'a> SortedCommands<'a> {
    pub fn from_pairs(
        pairs: &'a [(CommandId, CommandPermissions)],
    ) -> Result<Self, InteractionError> {
        let mut sorted = [SortedCommand::default(); InteractionError::GUILD_COMMAND_LIMIT];

        'outer: for (command_id, permissions) in pairs {
            for mut sorted_command in &mut sorted {
                // If the sorted command ID is neither the currently iterated
                // provided command ID nor the maximum value, then we know this
                // isn't it and can't be used.
                if sorted_command.id != *command_id && sorted_command.id.0 != u64::MAX {
                    continue;
                }

                // We've got the right sorted command, but we first need to check
                // if we've already reached the maximum number of command
                // permissions allowed.
                if !validate::guild_command_permissions(sorted_command.count() + 1) {
                    return Err(InteractionError {
                        kind: InteractionErrorType::TooManyCommandPermissions,
                    });
                }

                // Set the sorted command's ID if it's currently the maximum
                // value.
                if sorted_command.id != *command_id {
                    sorted_command.id = *command_id;
                }

                // And now set the permissions and increment the number of
                // permissions set.
                sorted_command.permissions.0[sorted_command.count()] = Some(permissions);
                sorted_command.count += 1;

                continue 'outer;
            }

            // We've run out of space in the sorted permissions, which means the
            // user provided too many commands.
            return Err(InteractionError {
                kind: InteractionErrorType::TooManyCommands,
            });
        }

        Ok(Self { inner: sorted })
    }
}

impl Serialize for SortedCommands<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(self.inner.iter().filter(|item| item.id.0 != u64::MAX))
    }
}

/// Update command permissions for all commands in a guild.
///
/// This overwrites the command permissions so the full set of permissions
/// have to be sent every time.
#[derive(Debug)]
#[must_use = "requests must be configured and executed"]
pub struct SetCommandPermissions<'a> {
    application_id: ApplicationId,
    guild_id: GuildId,
    http: &'a Client,
    permissions: SortedCommands<'a>,
}

impl<'a> SetCommandPermissions<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: ApplicationId,
        guild_id: GuildId,
        permissions: &'a [(CommandId, CommandPermissions)],
    ) -> Result<Self, InteractionError> {
        let sorted_permissions = SortedCommands::from_pairs(permissions)?;

        Ok(Self {
            application_id,
            guild_id,
            http,
            permissions: sorted_permissions,
        })
    }

    fn request(&self) -> Result<Request, Error> {
        Request::builder(&Route::SetCommandPermissions {
            application_id: self.application_id.0,
            guild_id: self.guild_id.0,
        })
        .json(&self.permissions)
        .map(RequestBuilder::build)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<CommandPermissions> {
        match self.request() {
            Ok(request) => self.http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        super::{InteractionError, InteractionErrorType},
        SetCommandPermissions,
    };
    use crate::Client;
    use serde::Deserialize;
    use std::{error::Error, iter};
    use twilight_model::{
        application::command::permissions::{CommandPermissions, CommandPermissionsType},
        id::{ApplicationId, CommandId, GuildId, RoleId},
    };

    const APPLICATION_ID: ApplicationId = ApplicationId(1);
    const GUILD_ID: GuildId = GuildId(2);

    #[derive(Debug, Deserialize, Eq, PartialEq)]
    struct GuildCommandPermissionDeserializable {
        id: CommandId,
        permissions: Vec<CommandPermissions>,
    }

    fn command_permissions(id: CommandId) -> impl Iterator<Item = (CommandId, CommandPermissions)> {
        iter::repeat((
            id,
            CommandPermissions {
                id: CommandPermissionsType::Role(RoleId(4)),
                permission: true,
            },
        ))
    }

    #[allow(unused)]
    #[test]
    fn test_correct_validation() -> Result<(), Box<dyn Error>> {
        let http = Client::new("token".to_owned());
        let command_permissions = &[
            (
                CommandId(1),
                CommandPermissions {
                    id: CommandPermissionsType::Role(RoleId(3)),
                    permission: true,
                },
            ),
            (
                CommandId(1),
                CommandPermissions {
                    id: CommandPermissionsType::Role(RoleId(4)),
                    permission: true,
                },
            ),
            (
                CommandId(2),
                CommandPermissions {
                    id: CommandPermissionsType::Role(RoleId(5)),
                    permission: true,
                },
            ),
        ];

        let builder =
            SetCommandPermissions::new(&http, APPLICATION_ID, GUILD_ID, command_permissions)?;

        let request = builder.request()?;
        let body = request.body().expect("body must be present");
        let actual = serde_json::from_slice::<Vec<GuildCommandPermissionDeserializable>>(body)?;

        let expected = &[
            GuildCommandPermissionDeserializable {
                id: CommandId(1),
                permissions: Vec::from([
                    CommandPermissions {
                        id: CommandPermissionsType::Role(RoleId(3)),
                        permission: true,
                    },
                    CommandPermissions {
                        id: CommandPermissionsType::Role(RoleId(4)),
                        permission: true,
                    },
                ]),
            },
            GuildCommandPermissionDeserializable {
                id: CommandId(2),
                permissions: Vec::from([CommandPermissions {
                    id: CommandPermissionsType::Role(RoleId(5)),
                    permission: true,
                }]),
            },
        ];

        assert_eq!(expected, actual.as_slice());

        Ok(())
    }

    #[test]
    fn test_incorrect_validation() {
        let http = Client::new("token".to_owned());
        let command_permissions = command_permissions(CommandId(2))
            .take(InteractionError::GUILD_COMMAND_PERMISSION_LIMIT + 1)
            .collect::<Vec<_>>();

        let request =
            SetCommandPermissions::new(&http, APPLICATION_ID, GUILD_ID, &command_permissions);
        assert!(matches!(
            request.unwrap_err().kind(),
            InteractionErrorType::TooManyCommandPermissions
        ));
    }

    #[test]
    fn test_limits() {
        const SIZE: usize = InteractionError::GUILD_COMMAND_LIMIT;

        let http = Client::new("token".to_owned());
        let command_permissions = (1..=SIZE)
            .flat_map(|id| {
                command_permissions(CommandId(id as u64))
                    .take(InteractionError::GUILD_COMMAND_PERMISSION_LIMIT)
            })
            .collect::<Vec<_>>();

        assert!(
            SetCommandPermissions::new(&http, APPLICATION_ID, GUILD_ID, &command_permissions)
                .is_ok()
        );
    }

    #[test]
    fn test_command_count_over_limit() {
        const SIZE: usize = 101;

        let http = Client::new("token".to_owned());
        let command_permissions = (1..=SIZE)
            .flat_map(|id| command_permissions(CommandId(id as u64)).take(3))
            .collect::<Vec<_>>();

        let request =
            SetCommandPermissions::new(&http, APPLICATION_ID, GUILD_ID, &command_permissions);
        assert!(matches!(
            request.unwrap_err().kind(),
            InteractionErrorType::TooManyCommands
        ));
    }

    #[test]
    fn test_no_permissions() {
        let http = Client::new("token".to_owned());

        assert!(SetCommandPermissions::new(&http, APPLICATION_ID, GUILD_ID, &[]).is_ok());
    }
}

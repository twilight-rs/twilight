use crate::{
    client::Client,
    error::Error,
    request::{
        application::{InteractionError, InteractionErrorType},
        validate_inner, Request, RequestBuilder,
    },
    response::ResponseFuture,
    routing::Route,
};
use serde::{ser::SerializeSeq, Serialize, Serializer};
use twilight_model::{
    application::command::permissions::CommandPermissions,
    id::{ApplicationId, CommandId, GuildId},
};

#[derive(Clone, Copy, Debug)]
struct OptionalCommandPermissions<'a>(
    [Option<&'a CommandPermissions>; InteractionError::GUILD_COMMAND_PERMISSION_LIMIT],
);

impl OptionalCommandPermissions<'_> {
    /// Create a new list of command permissions with `None` elements.
    const fn new() -> Self {
        Self([None; InteractionError::GUILD_COMMAND_PERMISSION_LIMIT])
    }

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

        let mut iter = self.0.iter();

        // If an element isn't present while we haven't reached the end of the
        // iterator then any trailing elements aren't present either.
        while let Some(Some(value)) = iter.next() {
            seq.serialize_element(value)?;
        }

        seq.end()
    }
}

/// A sorted command's permissions.
///
/// Used in combination with [`SortedCommands`].
#[derive(Clone, Copy, Debug, Serialize)]
struct SortedCommand<'a> {
    #[serde(skip_serializing)]
    count: u8,
    id: CommandId,
    permissions: OptionalCommandPermissions<'a>,
}

impl SortedCommand<'_> {
    /// Create a new default sorted command with no configured permissions.
    ///
    /// The ID of the command is `u64::MAX`.
    fn new() -> Self {
        Self {
            count: 0,
            id: CommandId::new(u64::MAX).expect("non zero"),
            permissions: OptionalCommandPermissions::new(),
        }
    }

    // Retrieve the current count as a usize for indexing.
    const fn count(self) -> usize {
        self.count as usize
    }
}

/// Sorted list of commands and their permissions.
#[derive(Debug)]
struct SortedCommands<'a> {
    inner: [SortedCommand<'a>; InteractionError::GUILD_COMMAND_LIMIT],
}

impl<'a> SortedCommands<'a> {
    pub fn from_pairs(
        pairs: &'a [(CommandId, CommandPermissions)],
    ) -> Result<Self, InteractionError> {
        let mut sorted = [SortedCommand::new(); InteractionError::GUILD_COMMAND_LIMIT];
        let mut outer_idx = 0;

        'outer: while outer_idx < pairs.len() {
            let (command_id, permissions) = &pairs[outer_idx];
            let mut inner_idx = 0;

            while inner_idx < sorted.len() {
                // If the sorted command ID is neither the currently iterated
                // provided command ID nor the maximum value, then we know this
                // isn't it and can't be used.
                let sorted_id = sorted[inner_idx].id;

                if sorted_id.get() != command_id.get() && sorted_id.get() != u64::MAX {
                    inner_idx += 1;

                    continue;
                }

                // We've got the right sorted command, but we first need to check
                // if we've already reached the maximum number of command
                // permissions allowed.
                let sorted_count = sorted[inner_idx].count();

                if !validate_inner::guild_command_permissions(sorted_count + 1) {
                    return Err(InteractionError {
                        kind: InteractionErrorType::TooManyCommandPermissions,
                    });
                }

                // Set the sorted command's ID if it's currently the maximum
                // value.
                if sorted_id.get() != command_id.get() {
                    sorted[inner_idx].id = *command_id;
                }

                // And now set the permissions and increment the number of
                // permissions set.
                sorted[inner_idx].permissions.0[sorted_count] = Some(permissions);
                sorted[inner_idx].count += 1;

                outer_idx += 1;

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
        serializer.collect_seq(self.inner.iter().filter(|item| item.id.get() != u64::MAX))
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
        let sorted_permissions = match SortedCommands::from_pairs(permissions) {
            Ok(sorted_permissions) => sorted_permissions,
            Err(source) => return Err(source),
        };

        Ok(Self {
            application_id,
            guild_id,
            http,
            permissions: sorted_permissions,
        })
    }

    fn request(&self) -> Result<Request, Error> {
        Request::builder(&Route::SetCommandPermissions {
            application_id: self.application_id.get(),
            guild_id: self.guild_id.get(),
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
        super::super::{InteractionError, InteractionErrorType},
        SetCommandPermissions,
    };
    use crate::Client;
    use serde::Deserialize;
    use std::{error::Error, iter};
    use twilight_model::{
        application::command::permissions::{CommandPermissions, CommandPermissionsType},
        id::{ApplicationId, CommandId, GuildId, RoleId},
    };

    fn application_id() -> ApplicationId {
        ApplicationId::new(1).expect("non zero")
    }

    fn guild_id() -> GuildId {
        GuildId::new(2).expect("non zero")
    }

    #[derive(Debug, Deserialize, Eq, PartialEq)]
    struct GuildCommandPermissionDeserializable {
        id: CommandId,
        permissions: Vec<CommandPermissions>,
    }

    fn command_permissions(id: CommandId) -> impl Iterator<Item = (CommandId, CommandPermissions)> {
        iter::repeat((
            id,
            CommandPermissions {
                id: CommandPermissionsType::Role(RoleId::new(4).expect("non zero")),
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
                CommandId::new(1).expect("non zero"),
                CommandPermissions {
                    id: CommandPermissionsType::Role(RoleId::new(3).expect("non zero")),
                    permission: true,
                },
            ),
            (
                CommandId::new(1).expect("non zero"),
                CommandPermissions {
                    id: CommandPermissionsType::Role(RoleId::new(4).expect("non zero")),
                    permission: true,
                },
            ),
            (
                CommandId::new(2).expect("non zero"),
                CommandPermissions {
                    id: CommandPermissionsType::Role(RoleId::new(5).expect("non zero")),
                    permission: true,
                },
            ),
        ];

        let builder =
            SetCommandPermissions::new(&http, application_id(), guild_id(), command_permissions)?;

        let request = builder.request()?;
        let body = request.body().expect("body must be present");
        let actual = serde_json::from_slice::<Vec<GuildCommandPermissionDeserializable>>(body)?;

        let expected = &[
            GuildCommandPermissionDeserializable {
                id: CommandId::new(1).expect("non zero"),
                permissions: Vec::from([
                    CommandPermissions {
                        id: CommandPermissionsType::Role(RoleId::new(3).expect("non zero")),
                        permission: true,
                    },
                    CommandPermissions {
                        id: CommandPermissionsType::Role(RoleId::new(4).expect("non zero")),
                        permission: true,
                    },
                ]),
            },
            GuildCommandPermissionDeserializable {
                id: CommandId::new(2).expect("non zero"),
                permissions: Vec::from([CommandPermissions {
                    id: CommandPermissionsType::Role(RoleId::new(5).expect("non zero")),
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
        let command_permissions = command_permissions(CommandId::new(2).expect("non zero"))
            .take(InteractionError::GUILD_COMMAND_PERMISSION_LIMIT + 1)
            .collect::<Vec<_>>();

        let request =
            SetCommandPermissions::new(&http, application_id(), guild_id(), &command_permissions);
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
                command_permissions(CommandId::new(id as u64).expect("non zero"))
                    .take(InteractionError::GUILD_COMMAND_PERMISSION_LIMIT)
            })
            .collect::<Vec<_>>();

        assert!(SetCommandPermissions::new(
            &http,
            application_id(),
            guild_id(),
            &command_permissions
        )
        .is_ok());
    }

    #[test]
    fn test_command_count_over_limit() {
        const SIZE: usize = 101;

        let http = Client::new("token".to_owned());
        let command_permissions = (1..=SIZE)
            .flat_map(|id| {
                command_permissions(CommandId::new(id as u64).expect("non zero")).take(3)
            })
            .collect::<Vec<_>>();

        let request =
            SetCommandPermissions::new(&http, application_id(), guild_id(), &command_permissions);
        assert!(matches!(
            request.unwrap_err().kind(),
            InteractionErrorType::TooManyCommands
        ));
    }

    #[test]
    fn test_no_permissions() {
        let http = Client::new("token".to_owned());

        assert!(SetCommandPermissions::new(&http, application_id(), guild_id(), &[]).is_ok());
    }
}

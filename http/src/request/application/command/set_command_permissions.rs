use crate::{
    client::Client,
    error::Error,
    request::{Request, RequestBuilder, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use serde::{ser::SerializeSeq, Serialize, Serializer};
use twilight_model::{
    application::command::permissions::CommandPermissions,
    id::{
        marker::{ApplicationMarker, CommandMarker, GuildMarker},
        Id,
    },
};
use twilight_validate::command::{
    guild_permissions as validate_guild_permissions, CommandValidationError, GUILD_COMMAND_LIMIT,
    GUILD_COMMAND_PERMISSION_LIMIT,
};

#[derive(Clone, Copy, Debug)]
struct OptionalCommandPermissions<'a>(
    [Option<&'a CommandPermissions>; GUILD_COMMAND_PERMISSION_LIMIT],
);

impl OptionalCommandPermissions<'_> {
    /// Create a new list of command permissions with `None` elements.
    const fn new() -> Self {
        Self([None; GUILD_COMMAND_PERMISSION_LIMIT])
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
            .unwrap_or(GUILD_COMMAND_PERMISSION_LIMIT)
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
    id: Id<CommandMarker>,
    permissions: OptionalCommandPermissions<'a>,
}

impl SortedCommand<'_> {
    /// Create a new default sorted command with no configured permissions.
    ///
    /// The ID of the command is `u64::MAX`.
    const fn new() -> Self {
        Self {
            count: 0,
            id: Id::new(u64::MAX),
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
    inner: [SortedCommand<'a>; GUILD_COMMAND_LIMIT],
}

impl<'a> SortedCommands<'a> {
    pub fn from_pairs(
        pairs: &'a [(Id<CommandMarker>, CommandPermissions)],
    ) -> Result<Self, CommandValidationError> {
        let mut sorted = [SortedCommand::new(); GUILD_COMMAND_LIMIT];
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
                validate_guild_permissions(sorted_count + 1)?;

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
            return Err(CommandValidationError::COMMAND_COUNT_INVALID);
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
    application_id: Id<ApplicationMarker>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    permissions: SortedCommands<'a>,
}

impl<'a> SetCommandPermissions<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        guild_id: Id<GuildMarker>,
        permissions: &'a [(Id<CommandMarker>, CommandPermissions)],
    ) -> Result<Self, CommandValidationError> {
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

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<CommandPermissions> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for SetCommandPermissions<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::SetCommandPermissions {
            application_id: self.application_id.get(),
            guild_id: self.guild_id.get(),
        })
        .json(&self.permissions)
        .map(RequestBuilder::build)
    }
}

#[cfg(test)]
mod tests {
    use super::SetCommandPermissions;
    use crate::{request::TryIntoRequest, Client};
    use serde::Deserialize;
    use std::{error::Error, iter};
    use twilight_model::{
        application::command::permissions::{CommandPermissions, CommandPermissionsType},
        id::{
            marker::{ApplicationMarker, CommandMarker, GuildMarker},
            Id,
        },
    };
    use twilight_validate::command::{
        CommandValidationErrorType, GUILD_COMMAND_LIMIT, GUILD_COMMAND_PERMISSION_LIMIT,
    };

    const GUILD_ID: Id<GuildMarker> = Id::new(2);
    const APPLICATION_ID: Id<ApplicationMarker> = Id::new(1);

    #[derive(Debug, Deserialize, Eq, PartialEq)]
    struct GuildCommandPermissionDeserializable {
        id: Id<CommandMarker>,
        permissions: Vec<CommandPermissions>,
    }

    fn command_permissions(
        id: Id<CommandMarker>,
    ) -> impl Iterator<Item = (Id<CommandMarker>, CommandPermissions)> {
        iter::repeat((
            id,
            CommandPermissions {
                id: CommandPermissionsType::Role(Id::new(4)),
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
                Id::new(1),
                CommandPermissions {
                    id: CommandPermissionsType::Role(Id::new(3)),
                    permission: true,
                },
            ),
            (
                Id::new(1),
                CommandPermissions {
                    id: CommandPermissionsType::Role(Id::new(4)),
                    permission: true,
                },
            ),
            (
                Id::new(2),
                CommandPermissions {
                    id: CommandPermissionsType::Role(Id::new(5)),
                    permission: true,
                },
            ),
        ];

        let builder =
            SetCommandPermissions::new(&http, APPLICATION_ID, GUILD_ID, command_permissions)?;

        let request = builder.try_into_request()?;
        let body = request.body().expect("body must be present");
        let actual = serde_json::from_slice::<Vec<GuildCommandPermissionDeserializable>>(body)?;

        let expected = &[
            GuildCommandPermissionDeserializable {
                id: Id::new(1),
                permissions: Vec::from([
                    CommandPermissions {
                        id: CommandPermissionsType::Role(Id::new(3)),
                        permission: true,
                    },
                    CommandPermissions {
                        id: CommandPermissionsType::Role(Id::new(4)),
                        permission: true,
                    },
                ]),
            },
            GuildCommandPermissionDeserializable {
                id: Id::new(2),
                permissions: Vec::from([CommandPermissions {
                    id: CommandPermissionsType::Role(Id::new(5)),
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
        let command_permissions = command_permissions(Id::new(2))
            .take(GUILD_COMMAND_PERMISSION_LIMIT + 1)
            .collect::<Vec<_>>();

        let request =
            SetCommandPermissions::new(&http, APPLICATION_ID, GUILD_ID, &command_permissions);
        assert!(matches!(
            request.unwrap_err().kind(),
            CommandValidationErrorType::PermissionsCountInvalid
        ));
    }

    #[test]
    fn test_limits() {
        const SIZE: usize = GUILD_COMMAND_LIMIT;

        let http = Client::new("token".to_owned());
        let command_permissions = (1..=SIZE)
            .flat_map(|id| {
                command_permissions(Id::new(id as u64)).take(GUILD_COMMAND_PERMISSION_LIMIT)
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
            .flat_map(|id| command_permissions(Id::new(id as u64)).take(3))
            .collect::<Vec<_>>();

        let request =
            SetCommandPermissions::new(&http, APPLICATION_ID, GUILD_ID, &command_permissions);
        assert!(matches!(
            request.unwrap_err().kind(),
            CommandValidationErrorType::CountInvalid
        ));
    }

    #[test]
    fn test_no_permissions() {
        let http = Client::new("token".to_owned());

        assert!(SetCommandPermissions::new(&http, APPLICATION_ID, GUILD_ID, &[]).is_ok());
    }
}

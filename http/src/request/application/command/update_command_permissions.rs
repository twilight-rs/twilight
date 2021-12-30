use crate::{
    client::Client,
    error::Error,
    request::{Request, RequestBuilder, TryIntoRequest},
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    application::command::permissions::CommandPermissions,
    id::{
        marker::{ApplicationMarker, CommandMarker, GuildMarker},
        Id,
    },
};
use twilight_validate::command::{
    guild_permissions as validate_guild_permissions, CommandValidationError,
};

#[derive(Serialize)]
struct UpdateCommandPermissionsFields<'a> {
    pub permissions: &'a [CommandPermissions],
}

/// Update command permissions for a single command in a guild.
///
/// # Note:
///
/// This overwrites the command permissions so the full set of permissions
/// have to be sent every time.
#[must_use = "requests must be configured and executed"]
pub struct UpdateCommandPermissions<'a> {
    application_id: Id<ApplicationMarker>,
    command_id: Id<CommandMarker>,
    guild_id: Id<GuildMarker>,
    fields: UpdateCommandPermissionsFields<'a>,
    http: &'a Client,
}

impl<'a> UpdateCommandPermissions<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        guild_id: Id<GuildMarker>,
        command_id: Id<CommandMarker>,
        permissions: &'a [CommandPermissions],
    ) -> Result<Self, CommandValidationError> {
        validate_guild_permissions(permissions.len())?;

        Ok(Self {
            application_id,
            command_id,
            guild_id,
            fields: UpdateCommandPermissionsFields { permissions },
            http,
        })
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<ListBody<CommandPermissions>> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for UpdateCommandPermissions<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::UpdateCommandPermissions {
            application_id: self.application_id.get(),
            command_id: self.command_id.get(),
            guild_id: self.guild_id.get(),
        })
        .json(&self.fields)
        .map(RequestBuilder::build)
    }
}

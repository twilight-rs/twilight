use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture, marker::ListBody},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    application::command::permissions::CommandPermission,
    id::{
        Id,
        marker::{ApplicationMarker, CommandMarker, GuildMarker},
    },
};
use twilight_validate::command::{
    CommandValidationError, guild_permissions as validate_guild_permissions,
};

#[derive(Serialize)]
struct UpdateCommandPermissionsBody<'a> {
    pub permissions: &'a [CommandPermission],
}

#[expect(clippy::struct_field_names)]
struct UpdateCommandPermissionsPath {
    application_id: Id<ApplicationMarker>,
    command_id: Id<CommandMarker>,
    guild_id: Id<GuildMarker>,
}

/// Update command permissions for a single command in a guild.
///
/// Note that this overwrites the command permissions, so the full set of
/// permissions has to be sent every time.
///
/// This request requires that the client was configured with an OAuth2 Bearer
/// token.
#[must_use = "requests must be configured and executed"]
pub struct UpdateCommandPermissions<'a> {
    body: Result<UpdateCommandPermissionsBody<'a>, CommandValidationError>,
    http: &'a Client,
    path: UpdateCommandPermissionsPath,
}

impl<'a> UpdateCommandPermissions<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        guild_id: Id<GuildMarker>,
        command_id: Id<CommandMarker>,
        permissions: &'a [CommandPermission],
    ) -> Self {
        let body = Ok(UpdateCommandPermissionsBody { permissions }).and_then(|body| {
            validate_guild_permissions(permissions.len())?;

            Ok(body)
        });

        Self {
            body,
            http,
            path: UpdateCommandPermissionsPath {
                application_id,
                command_id,
                guild_id,
            },
        }
    }
}

impl IntoFuture for UpdateCommandPermissions<'_> {
    type Output = Result<Response<ListBody<CommandPermission>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<CommandPermission>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for UpdateCommandPermissions<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let body = self.body.map_err(Error::validation)?;

        Request::builder_from_route(&Route::UpdateCommandPermissions {
            application_id: self.path.application_id.get(),
            command_id: self.path.command_id.get(),
            guild_id: self.path.guild_id.get(),
        })
        .json(&body)
        .build()
    }
}

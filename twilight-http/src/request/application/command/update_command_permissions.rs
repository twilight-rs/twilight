use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    application::command::permissions::CommandPermission,
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
    pub permissions: &'a [CommandPermission],
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
    application_id: Id<ApplicationMarker>,
    command_id: Id<CommandMarker>,
    guild_id: Id<GuildMarker>,
    fields: Result<UpdateCommandPermissionsFields<'a>, CommandValidationError>,
    http: &'a Client,
}

impl<'a> UpdateCommandPermissions<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        guild_id: Id<GuildMarker>,
        command_id: Id<CommandMarker>,
        permissions: &'a [CommandPermission],
    ) -> Self {
        let fields = Ok(UpdateCommandPermissionsFields { permissions }).and_then(|fields| {
            validate_guild_permissions(permissions.len())?;

            Ok(fields)
        });

        Self {
            application_id,
            command_id,
            guild_id,
            fields,
            http,
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
        let fields = self.fields.map_err(Error::validation)?;

        Request::builder(&Route::UpdateCommandPermissions {
            application_id: self.application_id.get(),
            command_id: self.command_id.get(),
            guild_id: self.guild_id.get(),
        })
        .json(&fields)
        .build()
    }
}

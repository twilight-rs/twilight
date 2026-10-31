use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::ListBody},
};
use std::future::IntoFuture;
use twilight_model::{
    guild::invite::Invite,
    id::{Id, marker::GuildMarker},
};

struct GetGuildInvitesPath {
    guild_id: Id<GuildMarker>,
}

/// Get information about the invites of a guild.
///
/// Requires the [`MANAGE_GUILD`] permission.
///
/// [`MANAGE_GUILD`]: twilight_model::guild::Permissions::MANAGE_GUILD
#[must_use = "requests must be configured and executed"]
pub struct GetGuildInvites<'a> {
    path: GetGuildInvitesPath,
    http: &'a Client,
}

impl<'a> GetGuildInvites<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            path: GetGuildInvitesPath { guild_id },
            http,
        }
    }
}

impl IntoFuture for GetGuildInvites<'_> {
    type Output = Result<Response<ListBody<Invite>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<Invite>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildInvites<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetGuildInvitesPath { guild_id } = self.path;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .resource("invites")
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}

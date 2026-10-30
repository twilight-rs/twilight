use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::ListBody},
};
use std::future::IntoFuture;
use twilight_model::{
    guild::auto_moderation::AutoModerationRule,
    id::{Id, marker::GuildMarker},
};

struct GetGuildAutoModerationRulesPath {
    guild_id: Id<GuildMarker>,
}

/// Get an auto moderation rule in a guild.
///
/// Requires the [`MANAGE_GUILD`] permission.
///
/// [`MANAGE_GUILD`]: twilight_model::guild::Permissions::MANAGE_GUILD
#[must_use = "requests must be configured and executed"]
pub struct GetGuildAutoModerationRules<'a> {
    http: &'a Client,
    path: GetGuildAutoModerationRulesPath,
}

impl<'a> GetGuildAutoModerationRules<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            http,
            path: GetGuildAutoModerationRulesPath { guild_id },
        }
    }
}

impl IntoFuture for GetGuildAutoModerationRules<'_> {
    type Output = Result<Response<ListBody<AutoModerationRule>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<AutoModerationRule>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetGuildAutoModerationRules<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetGuildAutoModerationRulesPath { guild_id } = self.path;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .resource("auto-moderation")
            .subresource("rules")
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}

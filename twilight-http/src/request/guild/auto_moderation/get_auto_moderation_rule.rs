use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    guild::auto_moderation::AutoModerationRule,
    id::{
        Id,
        marker::{AutoModerationRuleMarker, GuildMarker},
    },
};

struct GetAutoModerationRulePath {
    auto_moderation_rule_id: Id<AutoModerationRuleMarker>,
    guild_id: Id<GuildMarker>,
}

/// Get an auto moderation rule in a guild.
///
/// Requires the [`MANAGE_GUILD`] permission.
///
/// [`MANAGE_GUILD`]: twilight_model::guild::Permissions::MANAGE_GUILD
#[must_use = "requests must be configured and executed"]
pub struct GetAutoModerationRule<'a> {
    http: &'a Client,
    path: GetAutoModerationRulePath,
}

impl<'a> GetAutoModerationRule<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        auto_moderation_rule_id: Id<AutoModerationRuleMarker>,
    ) -> Self {
        Self {
            http,
            path: GetAutoModerationRulePath {
                auto_moderation_rule_id,
                guild_id,
            },
        }
    }
}

impl IntoFuture for GetAutoModerationRule<'_> {
    type Output = Result<Response<AutoModerationRule>, Error>;

    type IntoFuture = ResponseFuture<AutoModerationRule>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetAutoModerationRule<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let GetAutoModerationRulePath {
            auto_moderation_rule_id,
            guild_id,
        } = self.path;

        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .resource("auto-moderation")
            .subresource("rules")
            .id(auto_moderation_rule_id)
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}

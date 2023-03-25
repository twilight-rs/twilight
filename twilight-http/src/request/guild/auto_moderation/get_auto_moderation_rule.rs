use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    guild::auto_moderation::AutoModerationRule,
    id::{
        marker::{AutoModerationRuleMarker, GuildMarker},
        Id,
    },
};

/// Get an auto moderation rule in a guild.
///
/// Requires the [`MANAGE_GUILD`] permission.
///
/// [`MANAGE_GUILD`]: twilight_model::guild::Permissions::MANAGE_GUILD
#[must_use = "requests must be configured and executed"]
pub struct GetAutoModerationRule<'a> {
    auto_moderation_rule_id: Id<AutoModerationRuleMarker>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> GetAutoModerationRule<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        auto_moderation_rule_id: Id<AutoModerationRuleMarker>,
    ) -> Self {
        Self {
            auto_moderation_rule_id,
            guild_id,
            http,
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
        Request::builder(&Route::GetAutoModerationRule {
            auto_moderation_rule_id: self.auto_moderation_rule_id.get(),
            guild_id: self.guild_id.get(),
        })
        .build()
    }
}

use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{marker::EmptyBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::id::{
    marker::{AutoModerationRuleMarker, GuildMarker},
    Id,
};
use twilight_validate::request::{audit_reason as validate_audit_reason, ValidationError};

/// Delete an auto moderation rule in a guild.
///
/// Requires the [`MANAGE_GUILD`] permission.
///
/// [`MANAGE_GUILD`]: twilight_model::guild::Permissions::MANAGE_GUILD
#[must_use = "requests must be configured and executed"]
pub struct DeleteAutoModerationRule<'a> {
    auto_moderation_rule_id: Id<AutoModerationRuleMarker>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    reason: Option<&'a str>,
}

impl<'a> DeleteAutoModerationRule<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        auto_moderation_rule_id: Id<AutoModerationRuleMarker>,
    ) -> Self {
        Self {
            auto_moderation_rule_id,
            guild_id,
            http,
            reason: None,
        }
    }
}

impl<'a> AuditLogReason<'a> for DeleteAutoModerationRule<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, ValidationError> {
        validate_audit_reason(reason)?;

        self.reason.replace(reason);

        Ok(self)
    }
}

impl IntoFuture for DeleteAutoModerationRule<'_> {
    type Output = Result<Response<EmptyBody>, Error>;

    type IntoFuture = ResponseFuture<EmptyBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for DeleteAutoModerationRule<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::DeleteAutoModerationRule {
            auto_moderation_rule_id: self.auto_moderation_rule_id.get(),
            guild_id: self.guild_id.get(),
        });

        if let Some(reason) = self.reason {
            let header = request::audit_header(reason)?;

            request = request.headers(header);
        }

        Ok(request.build())
    }
}

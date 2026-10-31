use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    guild::auto_moderation::{
        AutoModerationAction, AutoModerationEventType, AutoModerationRule,
        AutoModerationTriggerMetadata,
    },
    id::{
        Id,
        marker::{AutoModerationRuleMarker, ChannelMarker, GuildMarker, RoleMarker},
    },
};
use twilight_validate::request::{ValidationError, audit_reason as validate_audit_reason};

#[derive(Serialize)]
struct UpdateAutoModerationRuleBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    actions: Option<&'a [AutoModerationAction]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    event_type: Option<AutoModerationEventType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exempt_channels: Option<&'a [Id<ChannelMarker>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exempt_roles: Option<&'a [Id<RoleMarker>]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_metadata: Option<&'a AutoModerationTriggerMetadata>,
}

struct UpdateAutoModerationRulePath {
    auto_moderation_rule_id: Id<AutoModerationRuleMarker>,
    guild_id: Id<GuildMarker>,
}

/// Update an auto moderation rule in a guild.
///
/// Requires the [`MANAGE_GUILD`] permission.
///
/// [`MANAGE_GUILD`]: twilight_model::guild::Permissions::MANAGE_GUILD
#[must_use = "requests must be configured and executed"]
pub struct UpdateAutoModerationRule<'a> {
    body: UpdateAutoModerationRuleBody<'a>,
    http: &'a Client,
    path: UpdateAutoModerationRulePath,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> UpdateAutoModerationRule<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        auto_moderation_rule_id: Id<AutoModerationRuleMarker>,
    ) -> Self {
        Self {
            body: UpdateAutoModerationRuleBody {
                actions: None,
                enabled: None,
                event_type: None,
                exempt_channels: None,
                exempt_roles: None,
                name: None,
                trigger_metadata: None,
            },
            http,
            path: UpdateAutoModerationRulePath {
                auto_moderation_rule_id,
                guild_id,
            },
            reason: Ok(None),
        }
    }

    /// Set the list of actions.
    pub const fn actions(mut self, actions: &'a [AutoModerationAction]) -> Self {
        self.body.actions = Some(actions);

        self
    }

    /// Set whether the rule is enabled.
    pub const fn enabled(mut self, enabled: bool) -> Self {
        self.body.enabled = Some(enabled);

        self
    }

    /// Set the rule's event type.
    pub const fn event_type(mut self, event_type: AutoModerationEventType) -> Self {
        self.body.event_type = Some(event_type);

        self
    }

    /// Set the channels where the rule does not apply.
    pub const fn exempt_channels(mut self, exempt_channels: &'a [Id<ChannelMarker>]) -> Self {
        self.body.exempt_channels = Some(exempt_channels);

        self
    }

    /// Set the roles to which the rule does not apply.
    pub const fn exempt_roles(mut self, exempt_roles: &'a [Id<RoleMarker>]) -> Self {
        self.body.exempt_roles = Some(exempt_roles);

        self
    }

    /// Set the rule's name.
    pub const fn name(mut self, name: &'a str) -> Self {
        self.body.name = Some(name);

        self
    }

    /// Set the trigger metadata.
    ///
    /// Care must be taken to set the correct metadata based on the rule's type.
    pub const fn trigger_metadata(
        mut self,
        trigger_metadata: &'a AutoModerationTriggerMetadata,
    ) -> Self {
        self.body.trigger_metadata = Some(trigger_metadata);

        self
    }
}

impl<'a> AuditLogReason<'a> for UpdateAutoModerationRule<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for UpdateAutoModerationRule<'_> {
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

impl TryIntoRequest for UpdateAutoModerationRule<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder_from_route(&Route::UpdateAutoModerationRule {
            auto_moderation_rule_id: self.path.auto_moderation_rule_id.get(),
            guild_id: self.path.guild_id.get(),
        })
        .json(&self.body);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}

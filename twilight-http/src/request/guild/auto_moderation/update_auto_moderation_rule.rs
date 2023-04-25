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
        marker::{AutoModerationRuleMarker, ChannelMarker, GuildMarker, RoleMarker},
        Id,
    },
};
use twilight_validate::request::{audit_reason as validate_audit_reason, ValidationError};

#[derive(Serialize)]
struct UpdateAutoModerationRuleFields<'a> {
    actions: Option<&'a [AutoModerationAction]>,
    enabled: Option<bool>,
    event_type: Option<AutoModerationEventType>,
    exempt_channels: Option<&'a [Id<ChannelMarker>]>,
    exempt_roles: Option<&'a [Id<RoleMarker>]>,
    name: Option<&'a str>,
    trigger_metadata: Option<&'a AutoModerationTriggerMetadata>,
}

/// Update an auto moderation rule in a guild.
///
/// Requires the [`MANAGE_GUILD`] permission.
///
/// [`MANAGE_GUILD`]: twilight_model::guild::Permissions::MANAGE_GUILD
#[must_use = "requests must be configured and executed"]
pub struct UpdateAutoModerationRule<'a> {
    auto_moderation_rule_id: Id<AutoModerationRuleMarker>,
    fields: UpdateAutoModerationRuleFields<'a>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> UpdateAutoModerationRule<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        auto_moderation_rule_id: Id<AutoModerationRuleMarker>,
    ) -> Self {
        Self {
            auto_moderation_rule_id,
            fields: UpdateAutoModerationRuleFields {
                actions: None,
                enabled: None,
                event_type: None,
                exempt_channels: None,
                exempt_roles: None,
                name: None,
                trigger_metadata: None,
            },
            guild_id,
            http,
            reason: Ok(None),
        }
    }

    /// Set the list of actions.
    pub const fn actions(mut self, actions: &'a [AutoModerationAction]) -> Self {
        self.fields.actions = Some(actions);

        self
    }

    /// Set whether the rule is enabled.
    pub const fn enabled(mut self, enabled: bool) -> Self {
        self.fields.enabled = Some(enabled);

        self
    }

    /// Set the rule's event type.
    pub const fn event_type(mut self, event_type: AutoModerationEventType) -> Self {
        self.fields.event_type = Some(event_type);

        self
    }

    /// Set the channels where the rule does not apply.
    pub const fn exempt_channels(mut self, exempt_channels: &'a [Id<ChannelMarker>]) -> Self {
        self.fields.exempt_channels = Some(exempt_channels);

        self
    }

    /// Set the roles to which the rule does not apply.
    pub const fn exempt_roles(mut self, exempt_roles: &'a [Id<RoleMarker>]) -> Self {
        self.fields.exempt_roles = Some(exempt_roles);

        self
    }

    /// Set the rule's name.
    pub const fn name(mut self, name: &'a str) -> Self {
        self.fields.name = Some(name);

        self
    }

    /// Set the trigger metadata.
    ///
    /// Care must be taken to set the correct metadata based on the rule's type.
    pub const fn trigger_metadata(
        mut self,
        trigger_metadata: &'a AutoModerationTriggerMetadata,
    ) -> Self {
        self.fields.trigger_metadata = Some(trigger_metadata);

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
        let mut request = Request::builder(&Route::UpdateAutoModerationRule {
            auto_moderation_rule_id: self.auto_moderation_rule_id.get(),
            guild_id: self.guild_id.get(),
        })
        .json(&self.fields);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}

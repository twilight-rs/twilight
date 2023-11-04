use std::future::IntoFuture;

use twilight_model::{
    guild::onboarding::{Onboarding, OnboardingMode, OnboardingPromptEmoji, OnboardingPromptType},
    id::{
        marker::{
            ChannelMarker, GuildMarker, OnboardingPromptMarker, OnboardingPromptOptionMarker,
            RoleMarker,
        },
        Id,
    },
};

use serde::{Deserialize, Serialize};

use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};

use twilight_validate::request::{audit_reason as validate_audit_reason, ValidationError};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct UpdateOnboardingPromptOption {
    pub channel_ids: Vec<Id<ChannelMarker>>,
    pub description: Option<String>,
    pub emoji: OnboardingPromptEmoji,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id<OnboardingPromptOptionMarker>>,
    pub role_ids: Vec<Id<RoleMarker>>,
    pub title: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct UpdateGuildOnboardingPrompt {
    pub id: Id<OnboardingPromptMarker>,
    pub in_onboarding: bool,
    #[serde(rename = "type")]
    pub kind: OnboardingPromptType,
    pub options: Vec<UpdateOnboardingPromptOption>,
    pub required: bool,
    pub single_select: bool,
    pub title: String,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct UpdateGuildOnboardingFields {
    pub default_channel_ids: Vec<Id<ChannelMarker>>,
    pub prompts: Vec<UpdateGuildOnboardingPrompt>,
    pub enabled: bool,
    pub mode: OnboardingMode,
}

/// Update the guild's onboarding flow
#[must_use = "requests must be configured and executed"]
pub struct UpdateGuildOnboarding<'a> {
    fields: UpdateGuildOnboardingFields,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> UpdateGuildOnboarding<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        fields: UpdateGuildOnboardingFields,
    ) -> Self {
        Self {
            fields,
            guild_id,
            http,
            reason: Ok(None),
        }
    }
}

impl<'a> AuditLogReason<'a> for UpdateGuildOnboarding<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for UpdateGuildOnboarding<'_> {
    type Output = Result<Response<Onboarding>, Error>;

    type IntoFuture = ResponseFuture<Onboarding>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for UpdateGuildOnboarding<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::UpdateGuildOnboarding {
            guild_id: self.guild_id.get(),
        });

        request = request.json(&self.fields);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            let header = request::audit_header(reason)?;

            request = request.headers(header);
        }

        request.build()
    }
}

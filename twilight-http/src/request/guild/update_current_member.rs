use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Nullable, Request, TryIntoRequest},
    response::{marker::EmptyBody, Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::id::{marker::GuildMarker, Id};
use twilight_validate::request::{
    audit_reason as validate_audit_reason, nickname as validate_nickname, ValidationError,
};

#[derive(Serialize)]
struct UpdateCurrentMemberFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    nick: Option<Nullable<&'a str>>,
}

/// Update the user's member in a guild.
#[must_use = "requests must be configured and executed"]
pub struct UpdateCurrentMember<'a> {
    fields: Result<UpdateCurrentMemberFields<'a>, ValidationError>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> UpdateCurrentMember<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            fields: Ok(UpdateCurrentMemberFields { nick: None }),
            guild_id,
            http,
            reason: Ok(None),
        }
    }

    /// Set the current user's nickname.
    ///
    /// Set to [`None`] to clear the nickname.
    ///
    /// The minimum length is 1 UTF-16 character and the maximum is 32 UTF-16 characters.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`Nickname`] if the nickname length is too
    /// short or too long.
    ///
    /// [`Nickname`]: twilight_validate::request::ValidationErrorType::Nickname
    pub fn nick(mut self, nick: Option<&'a str>) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            if let Some(nick) = nick {
                validate_nickname(nick)?;
            }

            fields.nick = Some(Nullable(nick));

            Ok(fields)
        });

        self
    }
}

impl<'a> AuditLogReason<'a> for UpdateCurrentMember<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for UpdateCurrentMember<'_> {
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

impl TryIntoRequest for UpdateCurrentMember<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;
        let mut request = Request::builder(&Route::UpdateCurrentMember {
            guild_id: self.guild_id.get(),
        })
        .json(&fields);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}

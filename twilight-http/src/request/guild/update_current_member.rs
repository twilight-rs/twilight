use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Method, Nullable, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::EmptyBody},
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::id::{Id, marker::GuildMarker};
use twilight_validate::request::{
    ValidationError, audit_reason as validate_audit_reason, bio as validate_bio,
    nickname as validate_nickname,
};

#[derive(Serialize)]
struct UpdateCurrentMemberBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    nick: Option<Nullable<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    banner: Option<Nullable<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    avatar: Option<Nullable<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bio: Option<Nullable<&'a str>>,
}

struct UpdateCurrentMemberPath {
    guild_id: Id<GuildMarker>,
}

/// Update the user's member in a guild.
#[must_use = "requests must be configured and executed"]
pub struct UpdateCurrentMember<'a> {
    body: Result<UpdateCurrentMemberBody<'a>, ValidationError>,
    path: UpdateCurrentMemberPath,
    http: &'a Client,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> UpdateCurrentMember<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            body: Ok(UpdateCurrentMemberBody {
                nick: None,
                banner: None,
                avatar: None,
                bio: None,
            }),
            path: UpdateCurrentMemberPath { guild_id },
            http,
            reason: Ok(None),
        }
    }

    /// Set the current member's nickname.
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
        self.body = self.body.and_then(|mut body| {
            if let Some(nick) = nick {
                validate_nickname(nick)?;
            }

            body.nick = Some(Nullable(nick));

            Ok(body)
        });

        self
    }

    /// Set the current member's banner.
    ///
    /// Set to [`None`] to clear the banner.
    ///
    /// Uses the [Image Data URI Scheme](https://discord.com/developers/docs/reference#image-data) for image data.
    pub const fn banner(mut self, banner: Option<&'a str>) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.banner = Some(Nullable(banner));
        }

        self
    }

    /// Set the current member's avatar.
    ///
    /// Set to [`None`] to clear the avatar.
    ///
    /// Uses the [Image Data URI Scheme](https://discord.com/developers/docs/reference#image-data) for image data.
    pub const fn avatar(mut self, avatar: Option<&'a str>) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.avatar = Some(Nullable(avatar));
        }

        self
    }

    /// Set the current member's bio.
    ///
    /// Set to [`None`] to clear the bio.
    ///
    /// The minimum length is 1 codepoint character and the maximum is 400 codepoint characters.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`Bio`] if the bio length is too
    /// short or too long.
    ///
    /// [`Bio`]: twilight_validate::request::ValidationErrorType::Bio
    pub fn bio(mut self, bio: Option<&'a str>) -> Self {
        self.body = self.body.and_then(|mut body| {
            if let Some(bio) = bio {
                validate_bio(bio)?;
            }

            body.bio = Some(Nullable(bio));

            Ok(body)
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
        let UpdateCurrentMemberPath { guild_id } = self.path;

        let body = self.body.map_err(Error::validation)?;
        let path = Path::builder()
            .resource("guilds")
            .id(guild_id)
            .resource("members")
            .me()
            .build()
            .map_err(Error::path)?;

        let mut request = Request::builder(Method::Patch, path).json(&body);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}

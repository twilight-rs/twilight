use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    guild::PartialMember,
    id::{
        Id,
        marker::{GuildMarker, RoleMarker, UserMarker},
    },
};
use twilight_validate::request::{ValidationError, nickname as validate_nickname};

#[derive(Serialize)]
struct AddGuildMemberBody<'a> {
    pub access_token: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deaf: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<&'a [Id<RoleMarker>]>,
}

struct AddGuildMemberPath {
    guild_id: Id<GuildMarker>,
    user_id: Id<UserMarker>,
}

#[must_use = "requests must be configured and executed"]
pub struct AddGuildMember<'a> {
    body: Result<AddGuildMemberBody<'a>, ValidationError>,
    http: &'a Client,
    path: AddGuildMemberPath,
}

/// Add a user to a guild.
///
/// An access token for the user with `guilds.join` scope is required. All other
/// fields are optional. See [Discord Docs/Add Guild Member].
///
/// [Discord Docs/Add Guild Member]: https://discord.com/developers/docs/resources/guild#add-guild-member
impl<'a> AddGuildMember<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
        access_token: &'a str,
    ) -> Self {
        Self {
            body: Ok(AddGuildMemberBody {
                access_token,
                deaf: None,
                mute: None,
                nick: None,
                roles: None,
            }),
            http,
            path: AddGuildMemberPath { guild_id, user_id },
        }
    }

    /// Whether the new member will be unable to hear audio when connected to a
    /// voice channel.
    pub const fn deaf(mut self, deaf: bool) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.deaf = Some(deaf);
        }

        self
    }

    /// Whether the new member will be unable to speak in voice channels.
    pub const fn mute(mut self, mute: bool) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.mute = Some(mute);
        }

        self
    }

    /// Set the user's initial nickname.
    ///
    /// The minimum length is 1 UTF-16 character and the maximum is 32 UTF-16
    /// characters.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`Nickname`] if the nickname length is too
    /// short or too long.
    ///
    /// [`Nickname`]: twilight_validate::request::ValidationErrorType::Nickname
    pub fn nick(mut self, nick: &'a str) -> Self {
        self.body = self.body.and_then(|mut body| {
            validate_nickname(nick)?;

            body.nick.replace(nick);

            Ok(body)
        });

        self
    }

    /// List of roles to assign the new member.
    pub const fn roles(mut self, roles: &'a [Id<RoleMarker>]) -> Self {
        if let Ok(body) = self.body.as_mut() {
            body.roles = Some(roles);
        }

        self
    }
}

impl IntoFuture for AddGuildMember<'_> {
    type Output = Result<Response<PartialMember>, Error>;

    type IntoFuture = ResponseFuture<PartialMember>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for AddGuildMember<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let body = self.body.map_err(Error::validation)?;

        Request::builder_from_route(&Route::AddGuildMember {
            guild_id: self.path.guild_id.get(),
            user_id: self.path.user_id.get(),
        })
        .json(&body)
        .build()
    }
}

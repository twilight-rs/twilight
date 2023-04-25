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
        marker::{GuildMarker, RoleMarker, UserMarker},
        Id,
    },
};
use twilight_validate::request::{nickname as validate_nickname, ValidationError};

#[derive(Serialize)]
struct AddGuildMemberFields<'a> {
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

#[must_use = "requests must be configured and executed"]
pub struct AddGuildMember<'a> {
    fields: AddGuildMemberFields<'a>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    user_id: Id<UserMarker>,
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
            fields: AddGuildMemberFields {
                access_token,
                deaf: None,
                mute: None,
                nick: None,
                roles: None,
            },
            guild_id,
            http,
            user_id,
        }
    }

    /// Whether the new member will be unable to hear audio when connected to a
    /// voice channel.
    pub const fn deaf(mut self, deaf: bool) -> Self {
        self.fields.deaf = Some(deaf);

        self
    }

    /// Whether the new member will be unable to speak in voice channels.
    pub const fn mute(mut self, mute: bool) -> Self {
        self.fields.mute = Some(mute);

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
    pub fn nick(mut self, nick: &'a str) -> Result<Self, ValidationError> {
        validate_nickname(nick)?;

        self.fields.nick.replace(nick);

        Ok(self)
    }

    /// List of roles to assign the new member.
    pub const fn roles(mut self, roles: &'a [Id<RoleMarker>]) -> Self {
        self.fields.roles = Some(roles);

        self
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    #[deprecated(since = "0.14.0", note = "use `.await` or `into_future` instead")]
    pub fn exec(self) -> ResponseFuture<PartialMember> {
        self.into_future()
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
        let mut request = Request::builder(&Route::AddGuildMember {
            guild_id: self.guild_id.get(),
            user_id: self.user_id.get(),
        });

        request = request.json(&self.fields)?;

        Ok(request.build())
    }
}

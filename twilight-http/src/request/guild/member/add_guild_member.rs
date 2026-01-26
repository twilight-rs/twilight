#[cfg(not(target_os = "wasi"))]
use crate::response::{Response, ResponseFuture};
use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
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
    fields: Result<AddGuildMemberFields<'a>, ValidationError>,
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
            fields: Ok(AddGuildMemberFields {
                access_token,
                deaf: None,
                mute: None,
                nick: None,
                roles: None,
            }),
            guild_id,
            http,
            user_id,
        }
    }

    /// Whether the new member will be unable to hear audio when connected to a
    /// voice channel.
    pub const fn deaf(mut self, deaf: bool) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.deaf = Some(deaf);
        }

        self
    }

    /// Whether the new member will be unable to speak in voice channels.
    pub const fn mute(mut self, mute: bool) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.mute = Some(mute);
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
        self.fields = self.fields.and_then(|mut fields| {
            validate_nickname(nick)?;

            fields.nick.replace(nick);

            Ok(fields)
        });

        self
    }

    /// List of roles to assign the new member.
    pub const fn roles(mut self, roles: &'a [Id<RoleMarker>]) -> Self {
        if let Ok(fields) = self.fields.as_mut() {
            fields.roles = Some(roles);
        }

        self
    }
}

#[cfg(not(target_os = "wasi"))]
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
        let fields = self.fields.map_err(Error::validation)?;

        Request::builder(&Route::AddGuildMember {
            guild_id: self.guild_id.get(),
            user_id: self.user_id.get(),
        })
        .json(&fields)
        .build()
    }
}

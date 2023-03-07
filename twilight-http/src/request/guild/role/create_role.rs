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
    guild::{Permissions, Role},
    id::{marker::GuildMarker, Id},
};
use twilight_validate::request::{audit_reason as validate_audit_reason, ValidationError};

#[derive(Serialize)]
struct CreateRoleFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hoist: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<&'a [u8]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mentionable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unicode_emoji: Option<&'a str>,
}

/// Create a role in a guild.
///
/// # Examples
///
/// ```no_run
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
/// let guild_id = Id::new(234);
///
/// client
///     .create_role(guild_id)
///     .color(0xd90083)
///     .name("Bright Pink")
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct CreateRole<'a> {
    fields: CreateRoleFields<'a>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> CreateRole<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            fields: CreateRoleFields {
                color: None,
                hoist: None,
                icon: None,
                mentionable: None,
                name: None,
                permissions: None,
                unicode_emoji: None,
            },
            guild_id,
            http,
            reason: Ok(None),
        }
    }

    /// Set the role color.
    ///
    /// This must be a valid hexadecimal RGB value. `0x000000` is ignored and
    /// doesn't count towards the final computed color in the user list. Refer
    /// to [`COLOR_MAXIMUM`] for the maximum acceptable value.
    ///
    /// [`COLOR_MAXIMUM`]: twilight_validate::embed::COLOR_MAXIMUM
    pub const fn color(mut self, color: u32) -> Self {
        self.fields.color = Some(color);

        self
    }

    /// If true, display the role in the members list.
    pub const fn hoist(mut self, hoist: bool) -> Self {
        self.fields.hoist = Some(hoist);

        self
    }

    /// Set the icon of the role.
    ///
    /// Only works if the guild has the `ROLE_ICONS` feature.
    ///
    /// See [Discord Docs/Image Data].
    ///
    /// [Discord Docs/Image Data]: https://discord.com/developers/docs/reference#image-data
    pub const fn icon(mut self, icon: &'a [u8]) -> Self {
        self.fields.icon = Some(icon);

        self
    }

    /// If true, the role can be @mentioned (pinged) in chat.
    pub const fn mentionable(mut self, mentionable: bool) -> Self {
        self.fields.mentionable = Some(mentionable);

        self
    }

    /// Set the name of the role.
    ///
    /// If none is specified, Discord sets this to `New Role`.
    pub const fn name(mut self, name: &'a str) -> Self {
        self.fields.name = Some(name);

        self
    }

    /// Set the allowed permissions of this role.
    pub const fn permissions(mut self, permissions: Permissions) -> Self {
        self.fields.permissions = Some(permissions);

        self
    }

    /// Set the unicode emoji of a role.
    pub const fn unicode_emoji(mut self, unicode_emoji: &'a str) -> Self {
        self.fields.unicode_emoji = Some(unicode_emoji);

        self
    }
}

impl<'a> AuditLogReason<'a> for CreateRole<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for CreateRole<'_> {
    type Output = Result<Response<Role>, Error>;

    type IntoFuture = ResponseFuture<Role>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for CreateRole<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::CreateRole {
            guild_id: self.guild_id.get(),
        });

        request = request.json(&self.fields);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}

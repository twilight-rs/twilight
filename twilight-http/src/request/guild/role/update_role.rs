use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Nullable, Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    guild::{Permissions, Role},
    id::{
        marker::{GuildMarker, RoleMarker},
        Id,
    },
};
use twilight_validate::request::{audit_reason as validate_audit_reason, ValidationError};

#[derive(Serialize)]
struct UpdateRoleFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Nullable<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hoist: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<Nullable<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mentionable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Nullable<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unicode_emoji: Option<Nullable<&'a str>>,
}

/// Update a role by guild id and its id.
#[must_use = "requests must be configured and executed"]
pub struct UpdateRole<'a> {
    fields: UpdateRoleFields<'a>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    role_id: Id<RoleMarker>,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> UpdateRole<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        role_id: Id<RoleMarker>,
    ) -> Self {
        Self {
            fields: UpdateRoleFields {
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
            role_id,
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
    pub const fn color(mut self, color: Option<u32>) -> Self {
        self.fields.color = Some(Nullable(color));

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
    ///
    /// # Editing
    ///
    /// Pass [`None`] to clear the existing icon.
    ///
    /// **Warning**: If the existing unicode emoji isn't cleared when setting the icon, it might
    /// cause incorrect behavior.
    ///
    /// # Examples
    ///
    /// Sets a role icon. The unicode emoji should always be cleared to ensure the icon can be
    /// set correctly.
    ///
    /// ```no_run
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_http::Client;
    /// use twilight_model::id::Id;
    ///
    /// let client = Client::new("token".to_owned());
    /// let guild_id = Id::new(1);
    /// let role_id = Id::new(1);
    /// let icon = "data:image/png;base64,BASE64_ENCODED_PNG_IMAGE_DATA";
    ///
    /// client
    ///     .update_role(guild_id, role_id)
    ///     .icon(Some(icon))
    ///     .unicode_emoji(None)
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub const fn icon(mut self, icon: Option<&'a str>) -> Self {
        self.fields.icon = Some(Nullable(icon));

        self
    }

    /// If true, the role can be @mentioned (pinged) in chat.
    pub const fn mentionable(mut self, mentionable: bool) -> Self {
        self.fields.mentionable = Some(mentionable);

        self
    }

    /// Set the name of the role.
    pub const fn name(mut self, name: Option<&'a str>) -> Self {
        self.fields.name = Some(Nullable(name));

        self
    }

    /// Set the allowed permissions of this role.
    pub const fn permissions(mut self, permissions: Permissions) -> Self {
        self.fields.permissions = Some(permissions);

        self
    }

    /// Set the unicode emoji of a role.
    ///
    /// Only works if the guild has the `ROLE_ICONS` feature.
    ///
    /// # Editing
    ///
    /// Pass [`None`] to clear the existing unicode emoji.
    ///
    /// **Warning**: If the existing icon isn't cleared when setting the unicode emoji, it might
    /// cause incorrect behavior.
    ///
    /// # Examples
    ///
    /// Sets a role unicode emoji. The icon should always be cleared to ensure the unicode emoji
    /// can be set correctly.
    ///
    /// ```no_run
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_http::Client;
    /// use twilight_model::id::Id;
    ///
    /// let client = Client::new("token".to_owned());
    /// let guild_id = Id::new(1);
    /// let role_id = Id::new(1);
    ///
    /// client
    ///     .update_role(guild_id, role_id)
    ///     .icon(None)
    ///     .unicode_emoji(Some("🦀"))
    ///     .await?;
    /// # Ok(()) }
    /// ```
    pub const fn unicode_emoji(mut self, unicode_emoji: Option<&'a str>) -> Self {
        self.fields.unicode_emoji = Some(Nullable(unicode_emoji));

        self
    }
}

impl<'a> AuditLogReason<'a> for UpdateRole<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for UpdateRole<'_> {
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

impl TryIntoRequest for UpdateRole<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::UpdateRole {
            guild_id: self.guild_id.get(),
            role_id: self.role_id.get(),
        });

        request = request.json(&self.fields);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}

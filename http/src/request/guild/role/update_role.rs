use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, AuditLogReasonError, NullableField, Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    guild::{Permissions, Role},
    id::{
        marker::{GuildMarker, RoleMarker},
        Id,
    },
};

#[derive(Serialize)]
struct UpdateRoleFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<NullableField<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hoist: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    icon: Option<&'a [u8]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mentionable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<NullableField<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    unicode_emoji: Option<&'a str>,
}

/// Update a role by guild id and its id.
#[must_use = "requests must be configured and executed"]
pub struct UpdateRole<'a> {
    fields: UpdateRoleFields<'a>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    role_id: Id<RoleMarker>,
    reason: Option<&'a str>,
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
            reason: None,
        }
    }

    /// Set the color of the role.
    pub const fn color(mut self, color: Option<u32>) -> Self {
        self.fields.color = Some(NullableField(color));

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
    pub const fn name(mut self, name: Option<&'a str>) -> Self {
        self.fields.name = Some(NullableField(name));

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

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Role> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl<'a> AuditLogReason<'a> for UpdateRole<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, AuditLogReasonError> {
        self.reason.replace(AuditLogReasonError::validate(reason)?);

        Ok(self)
    }
}

impl TryIntoRequest for UpdateRole<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::UpdateRole {
            guild_id: self.guild_id.get(),
            role_id: self.role_id.get(),
        });

        request = request.json(&self.fields)?;

        if let Some(reason) = &self.reason {
            let header = request::audit_header(reason)?;

            request = request.headers(header);
        }

        Ok(request.build())
    }
}

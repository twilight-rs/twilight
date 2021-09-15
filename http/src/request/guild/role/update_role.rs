use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, AuditLogReasonError, IntoRequest, NullableField, Request},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    guild::{Permissions, Role},
    id::{GuildId, RoleId},
};

#[derive(Serialize)]
struct UpdateRoleFields<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<NullableField<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hoist: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mentionable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<NullableField<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<Permissions>,
}

/// Update a role by guild id and its id.
#[must_use = "requests must be configured and executed"]
pub struct UpdateRole<'a> {
    fields: UpdateRoleFields<'a>,
    guild_id: GuildId,
    http: &'a Client,
    role_id: RoleId,
    reason: Option<&'a str>,
}

impl<'a> UpdateRole<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: GuildId, role_id: RoleId) -> Self {
        Self {
            fields: UpdateRoleFields {
                color: None,
                hoist: None,
                mentionable: None,
                name: None,
                permissions: None,
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

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Role> {
        let http = self.http;

        match self.into_request() {
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

impl IntoRequest for UpdateRole<'_> {
    fn into_request(self) -> Result<Request, Error> {
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

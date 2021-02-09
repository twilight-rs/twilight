use crate::request::prelude::*;
use twilight_model::{
    guild::{Permissions, Role},
    id::{GuildId, RoleId},
};

#[derive(Default, Serialize)]
struct UpdateRoleFields {
    #[allow(clippy::option_option)]
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Option<u32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    hoist: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mentionable: Option<bool>,
    #[allow(clippy::option_option)]
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    permissions: Option<Permissions>,
}

/// Update a role by guild id and its id.
pub struct UpdateRole<'a> {
    fields: UpdateRoleFields,
    fut: Option<Pending<'a, Role>>,
    guild_id: GuildId,
    http: &'a Client,
    role_id: RoleId,
    reason: Option<String>,
}

impl<'a> UpdateRole<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, role_id: RoleId) -> Self {
        Self {
            fields: UpdateRoleFields::default(),
            fut: None,
            guild_id,
            http,
            role_id,
            reason: None,
        }
    }

    /// Set the color of the role.
    pub fn color(mut self, color: impl Into<Option<u32>>) -> Self {
        self.fields.color.replace(color.into());

        self
    }

    /// If true, display the role in the members list.
    pub fn hoist(mut self, hoist: bool) -> Self {
        self.fields.hoist.replace(hoist);

        self
    }

    /// If true, the role can be @mentioned (pinged) in chat.
    pub fn mentionable(mut self, mentionable: bool) -> Self {
        self.fields.mentionable.replace(mentionable);

        self
    }

    /// Set the name of the role.
    pub fn name(mut self, name: impl Into<Option<String>>) -> Self {
        self.fields.name.replace(name.into());

        self
    }

    /// Set the allowed permissions of this role.
    pub fn permissions(mut self, permissions: Permissions) -> Self {
        self.fields.permissions.replace(permissions);

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                crate::json_to_vec(&self.fields).map_err(HttpError::json)?,
                headers,
                Route::UpdateRole {
                    guild_id: self.guild_id.0,
                    role_id: self.role_id.0,
                },
            ))
        } else {
            Request::from((
                crate::json_to_vec(&self.fields).map_err(HttpError::json)?,
                Route::UpdateRole {
                    guild_id: self.guild_id.0,
                    role_id: self.role_id.0,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

impl<'a> AuditLogReason for UpdateRole<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        self.reason
            .replace(AuditLogReasonError::validate(reason.into())?);

        Ok(self)
    }
}

poll_req!(UpdateRole<'_>, Role);

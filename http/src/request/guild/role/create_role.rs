use crate::json_to_vec;
use crate::request::prelude::*;
use twilight_model::{
    guild::{Permissions, Role},
    id::GuildId,
};

#[derive(Default, Serialize)]
struct CreateRoleFields {
    color: Option<u64>,
    hoist: Option<bool>,
    mentionable: Option<bool>,
    name: Option<String>,
    permissions: Option<Permissions>,
}

/// Create a role in a guild.
///
/// # Examples
///
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::id::GuildId;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token");
/// let guild_id = GuildId(234);
///
/// client.create_role(guild_id)
///     .color(0xd90083)
///     .name("Bright Pink")
///     .await?;
/// # Ok(()) }
/// ```
pub struct CreateRole<'a> {
    fields: CreateRoleFields,
    fut: Option<Pending<'a, Role>>,
    guild_id: GuildId,
    http: &'a Client,
    reason: Option<String>,
}

impl<'a> CreateRole<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fields: CreateRoleFields::default(),
            fut: None,
            guild_id,
            http,
            reason: None,
        }
    }

    /// Set the color of the role.
    pub fn color(mut self, color: u64) -> Self {
        self.fields.color.replace(color);

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
    ///
    /// If none is specified, Discord sets this to `New Role`.
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.fields.name.replace(name.into());

        self
    }

    /// Set the allowed permissions of this role.
    pub fn permissions(mut self, permissions: Permissions) -> Self {
        self.fields.permissions.replace(permissions);

        self
    }

    /// Attach an audit log reason to this request.
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason.replace(reason.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                json_to_vec(&self.fields)?,
                headers,
                Route::CreateRole {
                    guild_id: self.guild_id.0,
                },
            ))
        } else {
            Request::from((
                json_to_vec(&self.fields)?,
                Route::CreateRole {
                    guild_id: self.guild_id.0,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(CreateRole<'_>, Role);

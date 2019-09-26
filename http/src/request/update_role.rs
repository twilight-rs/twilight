use dawn_model::{
    guild::{Permissions, Role},
    id::{GuildId, RoleId},
};
use super::prelude::*;

#[derive(Serialize)]
pub struct UpdateRole<'a> {
    color: Option<u64>,
    hoist: Option<bool>,
    mentionable: Option<bool>,
    name: Option<String>,
    permissions: Option<Permissions>,
    #[serde(skip)]
    fut: Option<Pin<Box<dyn Future<Output = Result<Role>> + Send + 'a>>>,
    #[serde(skip)]
    guild_id: GuildId,
    #[serde(skip)]
    http: &'a Client,
    #[serde(skip)]
    role_id: RoleId,
}

impl<'a> UpdateRole<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: impl Into<GuildId>,
        role_id: impl Into<RoleId>,
    ) -> Self {
        Self {
            color: None,
            fut: None,
            guild_id: guild_id.into(),
            http,
            hoist: None,
            mentionable: None,
            name: None,
            permissions: None,
            role_id: role_id.into(),
        }
    }

    pub fn color(mut self, color: u64) -> Self {
        self.color.replace(color);

        self
    }

    pub fn hoist(mut self, hoist: bool) -> Self {
        self.hoist.replace(hoist);

        self
    }

    pub fn mentionable(mut self, mentionable: bool) -> Self {
        self.mentionable.replace(mentionable);

        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name.replace(name.into());

        self
    }

    pub fn permissions(mut self, permissions: Permissions) -> Self {
        self.permissions.replace(permissions);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(self)?,
            Route::UpdateRole {
                guild_id: self.guild_id.0,
                role_id: self.role_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(UpdateRole<'_>, Role);

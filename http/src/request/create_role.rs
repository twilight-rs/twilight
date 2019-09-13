use dawn_model::{
    guild::{Permissions, Role},
    id::GuildId,
};
use super::prelude::*;

#[derive(Serialize)]
pub struct CreateRole<'a> {
    color: Option<u64>,
    hoist: Option<bool>,
    mentionable: Option<bool>,
    name: Option<String>,
    permissions: Option<Permissions>,
    #[serde(skip)]
    fut: Option<PendingBody<'a, Role>>,
    #[serde(skip)]
    guild_id: GuildId,
    #[serde(skip)]
    http: &'a Client,
}

impl<'a> CreateRole<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: impl Into<GuildId>) -> Self {
        Self {
            color: None,
            fut: None,
            guild_id: guild_id.into(),
            http,
            hoist: None,
            mentionable: None,
            name: None,
            permissions: None,
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
        self.fut.replace(self.http.request(Request::from((
            serde_json::to_vec(self)?,
            Route::CreateRole {
                guild_id: self.guild_id.0,
            },
        )))?);

        Ok(())
    }
}

poll_req!(CreateRole<'_>, Role);

use dawn_model::{
    guild::Emoji,
    id::{GuildId, RoleId},
};
use serde::Serialize;
use super::prelude::*;

#[derive(Serialize)]
pub struct CreateEmoji<'a> {
    roles: Option<Vec<RoleId>>,
    #[serde(skip)]
    fut: Option<Pin<Box<dyn Future<Output = Result<Emoji>> + Send + 'a>>>,
    #[serde(skip)]
    guild_id: GuildId,
    #[serde(skip)]
    http: &'a Client,
    image: String,
    name: String,
}

impl<'a> CreateEmoji<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: impl Into<GuildId>,
        name: impl Into<String>,
        image: impl Into<String>,
    ) -> Self {
        Self {
            fut: None,
            guild_id: guild_id.into(),
            http,
            image: image.into(),
            name: name.into(),
            roles: None,
        }
    }

    pub fn roles(mut self, roles: Vec<RoleId>) -> Self {
        self.roles.replace(roles);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(self)?,
            Route::CreateEmoji {
                guild_id: self.guild_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(CreateEmoji<'_>, Emoji);

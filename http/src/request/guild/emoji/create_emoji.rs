use crate::request::prelude::*;
use twilight_model::{
    guild::Emoji,
    id::{GuildId, RoleId},
};

#[derive(Serialize)]
struct CreateEmojiFields {
    image: String,
    name: String,
    roles: Option<Vec<RoleId>>,
}

pub struct CreateEmoji<'a> {
    fut: Option<Pending<'a, Emoji>>,
    fields: CreateEmojiFields,
    guild_id: GuildId,
    http: &'a Client,
    reason: Option<String>,
}

impl<'a> CreateEmoji<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: GuildId,
        name: impl Into<String>,
        image: impl Into<String>,
    ) -> Self {
        Self {
            fields: CreateEmojiFields {
                image: image.into(),
                name: name.into(),
                roles: None,
            },
            fut: None,
            guild_id,
            http,
            reason: None,
        }
    }

    pub fn roles(mut self, roles: Vec<RoleId>) -> Self {
        self.fields.roles.replace(roles);

        self
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason.replace(reason.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                serde_json::to_vec(&self.fields)?,
                headers,
                Route::CreateEmoji {
                    guild_id: self.guild_id.0,
                },
            ))
        } else {
            Request::from((
                serde_json::to_vec(&self.fields)?,
                Route::CreateEmoji {
                    guild_id: self.guild_id.0,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(CreateEmoji<'_>, Emoji);

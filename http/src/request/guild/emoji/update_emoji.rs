use crate::json_to_vec;
use crate::request::prelude::*;
use std::borrow::Cow;
use twilight_model::{
    guild::Emoji,
    id::{EmojiId, GuildId, RoleId},
};

#[derive(Default, Serialize)]
struct UpdateEmojiFields<'a> {
    name: Option<Cow<'a, str>>,
    roles: Option<Cow<'a, [RoleId]>>,
}

/// Update an emoji in a guild, by id.
pub struct UpdateEmoji<'a> {
    emoji_id: EmojiId,
    fields: UpdateEmojiFields<'a>,
    fut: Option<Pending<'a, Emoji>>,
    guild_id: GuildId,
    http: &'a Client,
    reason: Option<Cow<'a, str>>,
}

impl<'a> UpdateEmoji<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId, emoji_id: EmojiId) -> Self {
        Self {
            fields: UpdateEmojiFields::default(),
            emoji_id,
            fut: None,
            guild_id,
            http,
            reason: None,
        }
    }

    /// Change the name of the emoji.
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self {
        self.fields.name.replace(name.into());

        self
    }

    /// Change the roles that the emoji is whitelisted to.
    pub fn roles(mut self, roles: impl Into<Cow<'a, [RoleId]>>) -> Self {
        self.fields.roles.replace(roles.into());

        self
    }

    /// Attach an audit log reason to this request.
    pub fn reason(mut self, reason: impl Into<Cow<'a, str>>) -> Self {
        self.reason.replace(reason.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                json_to_vec(&self.fields)?,
                headers,
                Route::UpdateEmoji {
                    emoji_id: self.emoji_id.0,
                    guild_id: self.guild_id.0,
                },
            ))
        } else {
            Request::from((
                json_to_vec(&self.fields)?,
                Route::UpdateEmoji {
                    emoji_id: self.emoji_id.0,
                    guild_id: self.guild_id.0,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(UpdateEmoji<'_>, Emoji);

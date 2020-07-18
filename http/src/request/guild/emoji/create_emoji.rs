use crate::json_to_vec;
use crate::request::prelude::*;
use std::borrow::Cow;
use twilight_model::{
    guild::Emoji,
    id::{GuildId, RoleId},
};

#[derive(Serialize)]
struct CreateEmojiFields<'a> {
    image: Cow<'a, str>,
    name: Cow<'a, str>,
    roles: Option<Cow<'a, [RoleId]>>,
}

/// Create an emoji in a guild.
///
/// The emoji must be a Data URI, in the form of `data:image/{type};base64,{data}` where `{type}`
/// is the image MIME type and `{data}` is the base64-encoded image.  Refer to [the discord docs]
/// for more information about image data.
///
/// [the discord docs]: https://discord.com/developers/docs/reference#image-data
pub struct CreateEmoji<'a> {
    fut: Option<Pending<'a, Emoji>>,
    fields: CreateEmojiFields<'a>,
    guild_id: GuildId,
    http: &'a Client,
    reason: Option<Cow<'a, str>>,
}

impl<'a> CreateEmoji<'a> {
    pub(crate) fn new(
        http: &'a Client,
        guild_id: GuildId,
        name: impl Into<Cow<'a, str>>,
        image: impl Into<Cow<'a, str>>,
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

    /// Whitelist roles for this emoji.
    ///
    /// Refer to [the discord docs] for more information.
    ///
    /// [the discord docs]: https://discord.com/developers/docs/resources/emoji
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
                Route::CreateEmoji {
                    guild_id: self.guild_id.0,
                },
            ))
        } else {
            Request::from((
                json_to_vec(&self.fields)?,
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

use crate::request::prelude::*;
use std::borrow::Cow;
use twilight_model::{
    channel::ReactionType,
    id::{ChannelId, MessageId},
};

/// Delete one reaction by a user on a message.
pub struct DeleteReaction<'a> {
    channel_id: ChannelId,
    emoji: String,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
    message_id: MessageId,
    target_user: Cow<'a, str>,
}

impl<'a> DeleteReaction<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: ChannelId,
        message_id: MessageId,
        emoji: ReactionType,
        target_user: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self {
            channel_id,
            emoji: super::format_emoji(emoji),
            fut: None,
            http,
            message_id,
            target_user: target_user.into(),
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.verify(Request::from(
            Route::DeleteReaction {
                channel_id: self.channel_id.0,
                emoji: Cow::Owned(self.emoji.clone()),
                message_id: self.message_id.0,
                user: self.target_user.clone(),
            },
        ))));

        Ok(())
    }
}

poll_req!(DeleteReaction<'_>, ());

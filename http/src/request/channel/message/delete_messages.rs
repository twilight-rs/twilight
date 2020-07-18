use crate::json_to_vec;
use crate::request::prelude::*;
use std::borrow::Cow;
use twilight_model::id::{ChannelId, MessageId};

#[derive(Serialize)]
struct DeleteMessagesFields<'a> {
    messages: Cow<'a, [MessageId]>,
}

/// Delete messgaes by [`ChannelId`] and Vec<[`MessageId`]>.
///
/// The vec count can be between 2 and 100. If the supplied [`MessageId`]s are invalid, they
/// still count towards the lower and upper limits. This method will not delete messages older
/// than two weeks. Refer to [the discord docs] for more information.
///
/// [`ChannelId`]: ../../../../twilight_model/id/struct.ChannelId.html
/// [`MessageId`]: ../../../../twilight_model/id/struct.MessageId.html
/// [the discord docs]: https://discord.com/developers/docs/resources/channel#bulk-delete-messages
pub struct DeleteMessages<'a> {
    channel_id: ChannelId,
    fields: DeleteMessagesFields<'a>,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
    reason: Option<Cow<'a, str>>,
}

impl<'a> DeleteMessages<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: ChannelId,
        message_ids: impl Into<Cow<'a, [MessageId]>>,
    ) -> Self {
        Self {
            channel_id,
            fields: DeleteMessagesFields {
                messages: message_ids.into(),
            },
            fut: None,
            http,
            reason: None,
        }
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
                Route::DeleteMessages {
                    channel_id: self.channel_id.0,
                },
            ))
        } else {
            Request::from((
                json_to_vec(&self.fields)?,
                Route::DeleteMessages {
                    channel_id: self.channel_id.0,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

poll_req!(DeleteMessages<'_>, ());

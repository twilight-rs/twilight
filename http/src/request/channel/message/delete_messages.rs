use crate::request::prelude::*;
use twilight_model::id::{ChannelId, MessageId};

#[derive(Serialize)]
struct DeleteMessagesFields {
    messages: Vec<MessageId>,
}

/// Delete messgaes by [`ChannelId`] and Vec<[`MessageId`]>.
///
/// The vec count can be between 2 and 100. If the supplied [`MessageId`]s are invalid, they
/// still count towards the lower and upper limits. This method will not delete messages older
/// than two weeks. Refer to [the discord docs] for more information.
///
/// [the discord docs]: https://discord.com/developers/docs/resources/channel#bulk-delete-messages
pub struct DeleteMessages<'a> {
    channel_id: ChannelId,
    fields: DeleteMessagesFields,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
    reason: Option<String>,
}

impl<'a> DeleteMessages<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: ChannelId,
        message_ids: impl Into<Vec<MessageId>>,
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

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                crate::json_to_vec(&self.fields).map_err(HttpError::json)?,
                headers,
                Route::DeleteMessages {
                    channel_id: self.channel_id.0,
                },
            ))
        } else {
            Request::from((
                crate::json_to_vec(&self.fields).map_err(HttpError::json)?,
                Route::DeleteMessages {
                    channel_id: self.channel_id.0,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

impl<'a> AuditLogReason for DeleteMessages<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        self.reason
            .replace(AuditLogReasonError::validate(reason.into())?);

        Ok(self)
    }
}

poll_req!(DeleteMessages<'_>, ());

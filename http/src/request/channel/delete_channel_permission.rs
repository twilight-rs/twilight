use crate::request::prelude::*;
use twilight_model::id::ChannelId;

/// Clear the permissions for a target ID in a channel.
///
/// The `target_id` is a `u64`, but it should point to a `RoleId` or a `UserId`.
pub struct DeleteChannelPermission<'a> {
    channel_id: ChannelId,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
    target_id: u64,
    reason: Option<String>,
}

impl<'a> DeleteChannelPermission<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId, target_id: u64) -> Self {
        Self {
            channel_id,
            fut: None,
            http,
            target_id,
            reason: None,
        }
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
                headers,
                Route::DeletePermissionOverwrite {
                    channel_id: self.channel_id.0,
                    target_id: self.target_id,
                },
            ))
        } else {
            Request::from(Route::DeletePermissionOverwrite {
                channel_id: self.channel_id.0,
                target_id: self.target_id,
            })
        };

        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

poll_req!(DeleteChannelPermission<'_>, ());

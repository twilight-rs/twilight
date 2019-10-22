use crate::request::prelude::*;
use dawn_model::id::ChannelId;

pub struct DeleteChannelPermission<'a> {
    channel_id: ChannelId,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
    target_id: u64,
}

impl<'a> DeleteChannelPermission<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId, target_id: u64) -> Self {
        Self {
            channel_id,
            fut: None,
            http,
            target_id,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::DeletePermissionOverwrite {
                channel_id: self.channel_id.0,
                target_id: self.target_id,
            },
        ))));

        Ok(())
    }
}

poll_req!(DeleteChannelPermission<'_>, ());

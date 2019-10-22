use crate::request::prelude::*;
use dawn_model::{guild::Permissions, id::ChannelId};

#[derive(Serialize)]
struct UpdateChannelPermissionConfiguredFields {
    allow: Permissions,
    deny: Permissions,
    kind: String,
}

pub struct UpdateChannelPermissionConfigured<'a> {
    channel_id: ChannelId,
    fields: UpdateChannelPermissionConfiguredFields,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
    target_id: u64,
}

impl<'a> UpdateChannelPermissionConfigured<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: ChannelId,
        allow: Permissions,
        deny: Permissions,
        kind: impl Into<String>,
        target_id: u64,
    ) -> Self {
        Self {
            channel_id,
            fields: UpdateChannelPermissionConfiguredFields {
                allow,
                deny,
                kind: kind.into(),
            },
            fut: None,
            http,
            target_id,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.verify(Request::from((
            serde_json::to_vec(&self.fields)?,
            Route::UpdatePermissionOverwrite {
                channel_id: self.channel_id.0,
                target_id: self.target_id,
            },
        )))));

        Ok(())
    }
}

poll_req!(UpdateChannelPermissionConfigured<'_>, ());

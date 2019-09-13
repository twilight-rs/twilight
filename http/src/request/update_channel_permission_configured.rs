use dawn_model::{
    guild::Permissions,
    id::ChannelId,
};
use super::prelude::*;

#[derive(Serialize)]
pub struct UpdateChannelPermissionConfigured<'a> {
    allow: Permissions,
    #[serde(skip)]
    channel_id: ChannelId,
    deny: Permissions,
    #[serde(skip)]
    fut: Option<Pending<'a>>,
    #[serde(skip)]
    http: &'a Client,
    #[serde(rename = "type")]
    kind: String,
    #[serde(skip)]
    target_id: u64,
}

impl<'a> UpdateChannelPermissionConfigured<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: impl Into<ChannelId>,
        allow: Permissions,
        deny: Permissions,
        kind: impl Into<String>,
        target_id: u64,
    ) -> Self {
        Self {
            allow,
            channel_id: channel_id.into(),
            deny,
            fut: None,
            http,
            kind: kind.into(),
            target_id,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(self.http.verify(Request::from((
            serde_json::to_vec(self)?,
            Route::UpdatePermissionOverwrite {
                channel_id: self.channel_id.0,
                target_id: self.target_id,
            },
        )))?);

        Ok(())
    }
}

poll_req!(UpdateChannelPermissionConfigured<'_>, ());

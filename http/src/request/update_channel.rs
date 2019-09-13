use dawn_model::{
    channel::{
        permission_overwrite::PermissionOverwrite,
        Channel,
    },
    id::ChannelId,
};
use super::prelude::*;

#[derive(Serialize)]
pub struct UpdateChannel<'a> {
    bitrate: Option<u64>,
    name: Option<String>,
    nsfw: Option<bool>,
    parent_id: Option<ChannelId>,
    permission_overwrites: Option<Vec<PermissionOverwrite>>,
    position: Option<u64>,
    rate_limit_per_user: Option<u64>,
    topic: Option<String>,
    user_limit: Option<u64>,
    #[serde(skip)]
    channel_id: ChannelId,
    #[serde(skip)]
    fut: Option<PendingBody<'a, Channel>>,
    #[serde(skip)]
    http: &'a Client,
}

impl<'a> UpdateChannel<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: impl Into<ChannelId>,
    ) -> Self {
        Self {
            bitrate: None,
            name: None,
            nsfw: None,
            parent_id: None,
            permission_overwrites: None,
            position: None,
            rate_limit_per_user: None,
            topic: None,
            user_limit: None,
            channel_id: channel_id.into(),
            fut: None,
            http,
        }
    }

    pub fn bitrate(mut self, bitrate: u64) -> Self {
        self.bitrate.replace(bitrate);

        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name.replace(name.into());

        self
    }

    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.nsfw.replace(nsfw);

        self
    }

    pub fn parent_id(mut self, parent_id: ChannelId) -> Self {
        self.parent_id.replace(parent_id);

        self
    }

    pub fn permission_overwrites(
        mut self,
        permission_overwrites: Vec<PermissionOverwrite>,
    ) -> Self {
        self.permission_overwrites.replace(permission_overwrites);

        self
    }

    pub fn position(mut self, position: u64) -> Self {
        self.position.replace(position);

        self
    }

    pub fn rate_limit_per_user(mut self, rate_limit_per_user: u64) -> Self {
        self.rate_limit_per_user.replace(rate_limit_per_user);

        self
    }

    pub fn topic(mut self, topic: impl Into<String>) -> Self {
        self.topic.replace(topic.into());

        self
    }

    pub fn user_limit(mut self, user_limit: u64) -> Self {
        self.user_limit.replace(user_limit);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(self.http.request(Request::from((
            serde_json::to_vec(self)?,
            Route::UpdateChannel {
                channel_id: self.channel_id.0,
            },
        )))?);

        Ok(())
    }
}

poll_req!(UpdateChannel<'_>, Channel);

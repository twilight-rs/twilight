use crate::request::prelude::*;
use dawn_model::{
    channel::{permission_overwrite::PermissionOverwrite, Channel},
    id::ChannelId,
};

#[derive(Default, Serialize)]
struct UpdateChannelFields {
    bitrate: Option<u64>,
    name: Option<String>,
    nsfw: Option<bool>,
    parent_id: Option<ChannelId>,
    permission_overwrites: Option<Vec<PermissionOverwrite>>,
    position: Option<u64>,
    rate_limit_per_user: Option<u64>,
    topic: Option<String>,
    user_limit: Option<u64>,
}

pub struct UpdateChannel<'a> {
    channel_id: ChannelId,
    fields: UpdateChannelFields,
    fut: Option<Pending<'a, Channel>>,
    http: &'a Client,
    reason: Option<String>,
}

impl<'a> UpdateChannel<'a> {
    pub(crate) fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            fields: UpdateChannelFields::default(),
            fut: None,
            http,
            reason: None,
        }
    }

    pub fn bitrate(mut self, bitrate: u64) -> Self {
        self.fields.bitrate.replace(bitrate);

        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.fields.name.replace(name.into());

        self
    }

    pub fn nsfw(mut self, nsfw: bool) -> Self {
        self.fields.nsfw.replace(nsfw);

        self
    }

    pub fn parent_id(mut self, parent_id: ChannelId) -> Self {
        self.fields.parent_id.replace(parent_id);

        self
    }

    pub fn permission_overwrites(
        mut self,
        permission_overwrites: Vec<PermissionOverwrite>,
    ) -> Self {
        self.fields
            .permission_overwrites
            .replace(permission_overwrites);

        self
    }

    pub fn position(mut self, position: u64) -> Self {
        self.fields.position.replace(position);

        self
    }

    pub fn rate_limit_per_user(mut self, rate_limit_per_user: u64) -> Self {
        self.fields.rate_limit_per_user.replace(rate_limit_per_user);

        self
    }

    pub fn topic(mut self, topic: impl Into<String>) -> Self {
        self.fields.topic.replace(topic.into());

        self
    }

    pub fn user_limit(mut self, user_limit: u64) -> Self {
        self.fields.user_limit.replace(user_limit);

        self
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason.replace(reason.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        let request = if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                serde_json::to_vec(&self.fields)?,
                headers,
                Route::UpdateChannel {
                    channel_id: self.channel_id.0,
                },
            ))
        } else {
            Request::from((
                serde_json::to_vec(&self.fields)?,
                Route::UpdateChannel {
                    channel_id: self.channel_id.0,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(UpdateChannel<'_>, Channel);

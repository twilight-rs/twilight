use crate::request::prelude::*;
use dawn_model::{id::ChannelId, invite::Invite};

#[derive(Default, Serialize)]
struct CreateInviteFields {
    max_age: Option<u64>,
    max_uses: Option<u64>,
    temporary: Option<bool>,
    unique: Option<bool>,
}

pub struct CreateInvite<'a> {
    channel_id: ChannelId,
    fields: CreateInviteFields,
    fut: Option<Pending<'a, Invite>>,
    http: &'a Client,
}

impl<'a> CreateInvite<'a> {
    pub fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            fields: CreateInviteFields::default(),
            fut: None,
            http,
        }
    }

    pub fn max_age(mut self, max_age: u64) -> Self {
        self.fields.max_age.replace(max_age);

        self
    }

    pub fn max_uses(mut self, max_uses: u64) -> Self {
        self.fields.max_uses.replace(max_uses);

        self
    }

    pub fn temporary(mut self, temporary: bool) -> Self {
        self.fields.temporary.replace(temporary);

        self
    }

    pub fn unique(mut self, unique: bool) -> Self {
        self.fields.unique.replace(unique);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(&self.fields)?,
            Route::CreateInvite {
                channel_id: self.channel_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(CreateInvite<'_>, Invite);

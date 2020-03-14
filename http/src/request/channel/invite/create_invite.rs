use crate::request::prelude::*;
use dawn_model::{
    id::{ChannelId, UserId},
    invite::{Invite, TargetUserType},
};

#[derive(Default, Serialize)]
struct CreateInviteFields {
    max_age: Option<u64>,
    max_uses: Option<u64>,
    temporary: Option<bool>,
    unique: Option<bool>,
    target_user: Option<String>,
    target_user_type: Option<TargetUserType>,
}

pub struct CreateInvite<'a> {
    channel_id: ChannelId,
    fields: CreateInviteFields,
    fut: Option<Pending<'a, Invite>>,
    http: &'a Client,
    reason: Option<String>,
}

impl<'a> CreateInvite<'a> {
    pub fn new(http: &'a Client, channel_id: ChannelId) -> Self {
        Self {
            channel_id,
            fields: CreateInviteFields::default(),
            fut: None,
            http,
            reason: None,
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

    pub fn target_user(mut self, target_user: UserId) -> Self {
        self.fields.target_user.replace(target_user.0.to_string());

        self
    }

    pub fn target_user_type(mut self, target_user_type: TargetUserType) -> Self {
        self.fields.target_user_type.replace(target_user_type);

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
                Route::CreateInvite {
                    channel_id: self.channel_id.0,
                },
            ))
        } else {
            Request::from((
                serde_json::to_vec(&self.fields)?,
                Route::CreateInvite {
                    channel_id: self.channel_id.0,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(CreateInvite<'_>, Invite);

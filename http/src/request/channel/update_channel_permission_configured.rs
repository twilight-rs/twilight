use crate::request::prelude::*;
use twilight_model::{guild::Permissions, id::ChannelId};

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
    reason: Option<String>,
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
            reason: None,
        }
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
                Route::UpdatePermissionOverwrite {
                    channel_id: self.channel_id.0,
                    target_id: self.target_id,
                },
            ))
        } else {
            Request::from((
                serde_json::to_vec(&self.fields)?,
                Route::UpdatePermissionOverwrite {
                    channel_id: self.channel_id.0,
                    target_id: self.target_id,
                },
            ))
        };

        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

poll_req!(UpdateChannelPermissionConfigured<'_>, ());

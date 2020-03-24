use crate::request::prelude::*;
use twilight_model::{
    guild::audit_log::{AuditLog, AuditLogEvent},
    id::{GuildId, UserId},
};

#[derive(Default)]
struct GetAuditLogFields {
    action_type: Option<AuditLogEvent>,
    before: Option<u64>,
    limit: Option<u64>,
    user_id: Option<UserId>,
}

pub struct GetAuditLog<'a> {
    fields: GetAuditLogFields,
    fut: Option<Pending<'a, Option<AuditLog>>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetAuditLog<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fields: GetAuditLogFields::default(),
            fut: None,
            guild_id,
            http,
        }
    }

    pub fn action_type(mut self, action_type: AuditLogEvent) -> Self {
        self.fields.action_type.replace(action_type);

        self
    }

    pub fn before(mut self, before: u64) -> Self {
        self.fields.before.replace(before);

        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.fields.limit.replace(limit);

        self
    }

    pub fn user_id(mut self, user_id: UserId) -> Self {
        self.fields.user_id.replace(user_id);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetAuditLogs {
                action_type: self.fields.action_type.map(|x| x as u64),
                before: self.fields.before,
                guild_id: self.guild_id.0,
                limit: self.fields.limit,
                user_id: self.fields.user_id.map(|x| x.0),
            },
        ))));

        Ok(())
    }
}

poll_req!(GetAuditLog<'_>, Option<AuditLog>);

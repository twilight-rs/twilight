use dawn_model::{
    guild::audit_log::{AuditLog, AuditLogEvent},
    id::{GuildId, UserId},
};
use super::prelude::*;

pub struct GetAuditLog<'a> {
    action_type: Option<AuditLogEvent>,
    before: Option<u64>,
    limit: Option<u64>,
    user_id: Option<UserId>,
    fut: Option<Pin<Box<dyn Future<Output = Result<Option<AuditLog>>> + Send + 'a>>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetAuditLog<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: impl Into<GuildId>) -> Self {
        Self {
            action_type: None,
            before: None,
            fut: None,
            guild_id: guild_id.into(),
            http,
            limit: None,
            user_id: None,
        }
    }

    pub fn action_type(mut self, action_type: AuditLogEvent) -> Self {
        self.action_type.replace(action_type);

        self
    }

    pub fn before(mut self, before: u64) -> Self {
        self.before.replace(before);

        self
    }

    pub fn limit(mut self, limit: u64) -> Self {
        self.limit.replace(limit);

        self
    }

    pub fn user_id(mut self, user_id: UserId) -> Self {
        self.user_id.replace(user_id);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(Route::GetAuditLogs {
            action_type: self.action_type.map(|x| x as u64),
            before: self.before,
            guild_id: self.guild_id.0,
            limit: self.limit,
            user_id: self.user_id.map(|x| x.0),
        }))));

        Ok(())
    }
}

poll_req!(GetAuditLog<'_>, Option<AuditLog>);

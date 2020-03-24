use crate::request::prelude::*;
use twilight_model::{guild::GuildPrune, id::GuildId};

#[derive(Default)]
struct CreateGuildPruneFields {
    compute_prune_count: Option<bool>,
    days: Option<u64>,
}

pub struct CreateGuildPrune<'a> {
    fields: CreateGuildPruneFields,
    guild_id: GuildId,
    fut: Option<Pending<'a, Option<GuildPrune>>>,
    http: &'a Client,
    reason: Option<String>,
}

impl<'a> CreateGuildPrune<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fields: CreateGuildPruneFields::default(),
            fut: None,
            guild_id,
            http,
            reason: None,
        }
    }

    pub fn compute_prune_count(mut self, compute_prune_count: bool) -> Self {
        self.fields.compute_prune_count.replace(compute_prune_count);

        self
    }

    pub fn days(mut self, days: u64) -> Self {
        self.fields.days.replace(days);

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
                headers,
                Route::CreateGuildPrune {
                    compute_prune_count: self.fields.compute_prune_count,
                    days: self.fields.days,
                    guild_id: self.guild_id.0,
                },
            ))
        } else {
            Request::from(Route::CreateGuildPrune {
                compute_prune_count: self.fields.compute_prune_count,
                days: self.fields.days,
                guild_id: self.guild_id.0,
            })
        };

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(CreateGuildPrune<'_>, Option<GuildPrune>);

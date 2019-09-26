use dawn_model::{
    guild::GuildPrune,
    id::GuildId,
};
use super::prelude::*;

#[derive(Serialize)]
pub struct CreateGuildPrune<'a> {
    compute_prune_count: Option<bool>,
    days: Option<u64>,
    #[serde(skip)]
    guild_id: GuildId,
    #[serde(skip)]
    fut: Option<Pin<Box<dyn Future<Output = Result<Option<GuildPrune>>> + Send + 'a>>>,
    #[serde(skip)]
    http: &'a Client,
}

impl<'a> CreateGuildPrune<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: impl Into<GuildId>) -> Self {
        Self {
            compute_prune_count: None,
            days: None,
            fut: None,
            guild_id: guild_id.into(),
            http,
        }
    }

    pub fn compute_prune_count(mut self, compute_prune_count: bool) -> Self {
        self.compute_prune_count.replace(compute_prune_count);

        self
    }

    pub fn days(mut self, days: u64) -> Self {
        self.days.replace(days);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from(Route::CreateGuildPrune {
            compute_prune_count: self.compute_prune_count,
            days: self.days,
            guild_id: self.guild_id.0,
        }))));

        Ok(())
    }
}

poll_req!(CreateGuildPrune<'_>, Option<GuildPrune>);

use dawn_model::{
    id::ChannelId,
    invite::Invite,
};
use super::prelude::*;

#[derive(Serialize)]
pub struct CreateInvite<'a> {
    pub max_age: Option<u64>,
    pub max_uses: Option<u64>,
    pub temporary: Option<bool>,
    pub unique: Option<bool>,
    #[serde(skip)]
    channel_id: ChannelId,
    #[serde(skip)]
    fut: Option<Pin<Box<dyn Future<Output = Result<Invite>> + Send + 'a>>>,
    #[serde(skip)]
    http: &'a Client,
}

impl<'a> CreateInvite<'a> {
    pub fn new(http: &'a Client, channel_id: impl Into<ChannelId>) -> Self {
        Self {
            channel_id: channel_id.into(),
            fut: None,
            http,
            max_age: None,
            max_uses: None,
            temporary: None,
            unique: None,
        }
    }

    pub fn max_age(mut self, max_age: u64) -> Self {
        self.max_age.replace(max_age.into());

        self
    }

    pub fn max_uses(mut self, max_uses: u64) -> Self {
        self.max_uses.replace(max_uses.into());

        self
    }

    pub fn temporary(mut self, temporary: bool) -> Self {
        self.temporary.replace(temporary.into());

        self
    }

    pub fn unique(mut self, unique: bool) -> Self {
        self.unique.replace(unique.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(self)?,
            Route::CreateInvite {
                channel_id: self.channel_id.0,
            },
        )))));

        Ok(())
    }
}

poll_req!(CreateInvite<'_>, Invite);

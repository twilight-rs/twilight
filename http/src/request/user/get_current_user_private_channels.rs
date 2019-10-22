use crate::request::prelude::*;
use dawn_model::channel::PrivateChannel;

pub struct GetCurrentUserPrivateChannels<'a> {
    fut: Option<Pending<'a, Vec<PrivateChannel>>>,
    http: &'a Client,
}

impl<'a> GetCurrentUserPrivateChannels<'a> {
    pub(crate) fn new(http: &'a Client) -> Self {
        Self {
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(
            self.http
                .request(Request::from(Route::GetUserPrivateChannels)),
        ));

        Ok(())
    }
}

poll_req!(GetCurrentUserPrivateChannels<'_>, Vec<PrivateChannel>);

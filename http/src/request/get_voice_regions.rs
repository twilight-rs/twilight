use crate::request::prelude::*;
use twilight_model::voice::VoiceRegion;

/// Get a list of voice regions that can be used when creating a guild.
pub struct GetVoiceRegions<'a> {
    fut: Option<Pending<'a, Vec<VoiceRegion>>>,
    http: &'a Client,
}

impl<'a> GetVoiceRegions<'a> {
    pub(crate) fn new(http: &'a Client) -> Self {
        Self { fut: None, http }
    }

    fn start(&mut self) -> Result<()> {
        let request = Request::from_route(Route::GetVoiceRegions);

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetVoiceRegions<'_>, Vec<VoiceRegion>);

use crate::request::prelude::*;
use twilight_model::voice::VoiceRegion;

pub struct GetVoiceRegions<'a> {
    fut: Option<Pending<'a, Vec<VoiceRegion>>>,
    http: &'a Client,
}

impl<'a> GetVoiceRegions<'a> {
    pub(crate) fn new(http: &'a Client) -> Self {
        Self { fut: None, http }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(
            self.http.request(Request::from(Route::GetVoiceRegions)),
        ));

        Ok(())
    }
}

poll_req!(GetVoiceRegions<'_>, Vec<VoiceRegion>);

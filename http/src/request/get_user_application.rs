use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    routing::Route,
};
use twilight_model::oauth::CurrentApplicationInfo;

pub struct GetUserApplicationInfo<'a> {
    fut: Option<PendingResponse<'a, CurrentApplicationInfo>>,
    http: &'a Client,
}

impl<'a> GetUserApplicationInfo<'a> {
    pub(crate) fn new(http: &'a Client) -> Self {
        Self { fut: None, http }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::GetCurrentUserApplicationInfo);

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetUserApplicationInfo<'_>, CurrentApplicationInfo);

use crate::request::prelude::*;
use twilight_model::oauth::CurrentApplicationInfo;

pub struct GetUserApplicationInfo<'a> {
    fut: Option<Pending<'a, CurrentApplicationInfo>>,
    http: &'a Client,
}

impl<'a> GetUserApplicationInfo<'a> {
    pub(crate) fn new(http: &'a Client) -> Self {
        Self { fut: None, http }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(
            self.http
                .request(Request::from(Route::GetCurrentUserApplicationInfo)),
        ));

        Ok(())
    }
}

poll_req!(GetUserApplicationInfo<'_>, CurrentApplicationInfo);

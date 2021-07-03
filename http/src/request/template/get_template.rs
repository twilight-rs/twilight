use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    routing::Route,
};
use twilight_model::template::Template;

/// Get a template by its code.
pub struct GetTemplate<'a> {
    fut: Option<PendingResponse<'a, Template>>,
    http: &'a Client,
    template_code: String,
}

impl<'a> GetTemplate<'a> {
    pub(crate) fn new(http: &'a Client, template_code: impl Into<String>) -> Self {
        Self::_new(http, template_code.into())
    }

    fn _new(http: &'a Client, template_code: String) -> Self {
        Self {
            fut: None,
            http,
            template_code,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::GetTemplate {
            template_code: self.template_code.clone(),
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetTemplate<'_>, Template);

use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::guild::template::Template;

/// Get a template by its code.
#[must_use = "requests must be configured and executed"]
pub struct GetTemplate<'a> {
    http: &'a Client,
    template_code: &'a str,
}

impl<'a> GetTemplate<'a> {
    pub(crate) const fn new(http: &'a Client, template_code: &'a str) -> Self {
        Self {
            http,
            template_code,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Template> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetTemplate<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetTemplate {
            template_code: self.template_code,
        }))
    }
}

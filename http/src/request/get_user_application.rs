use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
use twilight_model::oauth::CurrentApplicationInfo;

#[must_use = "requests must be configured and executed"]
pub struct GetUserApplicationInfo<'a> {
    http: &'a Client,
}

impl<'a> GetUserApplicationInfo<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self { http }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<CurrentApplicationInfo> {
        let request = Request::from_route(&Route::GetCurrentUserApplicationInfo);

        self.http.request(request)
    }
}

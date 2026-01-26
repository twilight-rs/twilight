#[cfg(not(target_os = "wasi"))]
use crate::response::{Response, ResponseFuture};
use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    id::{Id, marker::UserMarker},
    user::User,
};

/// Get a user's information by id.
#[must_use = "requests must be configured and executed"]
pub struct GetUser<'a> {
    http: &'a Client,
    user_id: Id<UserMarker>,
}

impl<'a> GetUser<'a> {
    pub(crate) const fn new(http: &'a Client, user_id: Id<UserMarker>) -> Self {
        Self { http, user_id }
    }
}

#[cfg(not(target_os = "wasi"))]
impl IntoFuture for GetUser<'_> {
    type Output = Result<Response<User>, Error>;

    type IntoFuture = ResponseFuture<User>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetUser<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetUser {
            user_id: self.user_id.get(),
        }))
    }
}

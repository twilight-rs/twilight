use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    id::{Id, marker::UserMarker},
    user::User,
};

pub struct GetUserPath {
    user_id: Id<UserMarker>,
}

/// Get a user's information by id.
#[must_use = "requests must be configured and executed"]
pub struct GetUser<'a> {
    http: &'a Client,
    path: GetUserPath,
}

impl<'a> GetUser<'a> {
    pub(crate) const fn new(http: &'a Client, user_id: Id<UserMarker>) -> Self {
        Self {
            http,
            path: GetUserPath { user_id },
        }
    }
}

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
        let path = Path::builder()
            .resource("users")
            .id(self.path.user_id)
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}

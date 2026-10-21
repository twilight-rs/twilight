use crate::{
    client::Client,
    error::Error,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::{Response, ResponseFuture, marker::ListBody},
};
use std::future::IntoFuture;
use twilight_model::user::Connection;

/// Get the current user's connections.
///
/// Requires the `connections` `OAuth2` scope.
#[must_use = "requests must be configured and executed"]
pub struct GetCurrentUserConnections<'a> {
    http: &'a Client,
}

impl<'a> GetCurrentUserConnections<'a> {
    pub(crate) const fn new(http: &'a Client) -> Self {
        Self { http }
    }
}

impl IntoFuture for GetCurrentUserConnections<'_> {
    type Output = Result<Response<ListBody<Connection>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<Connection>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl Route for GetCurrentUserConnections<'_> {
    type Fields = ();

    const METHOD: Method = Method::Get;

    fn path(_: Self::Fields) -> Path {
        Path::builder()
            .resource("users")
            .me()
            .resource("connections")
            .build()
    }
}

impl TryIntoRequest for GetCurrentUserConnections<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route_new::<Self>(()))
    }
}

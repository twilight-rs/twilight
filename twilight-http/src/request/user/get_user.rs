use crate::{
    client::Client,
    error::Error,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    id::{Id, marker::UserMarker},
    user::User,
};

pub struct GetUserFields {
    user_id: Id<UserMarker>,
}

/// Get a user's information by id.
#[must_use = "requests must be configured and executed"]
pub struct GetUser<'a> {
    fields: GetUserFields,
    http: &'a Client,
}

impl<'a> GetUser<'a> {
    pub(crate) const fn new(http: &'a Client, user_id: Id<UserMarker>) -> Self {
        Self {
            fields: GetUserFields { user_id },
            http,
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

impl Route for GetUser<'_> {
    type Fields = GetUserFields;

    const METHOD: Method = Method::Get;

    fn path(fields: Self::Fields) -> Path {
        Path::builder().resource("users").id(fields.user_id).build()
    }
}

impl TryIntoRequest for GetUser<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route_new::<Self>(self.fields))
    }
}

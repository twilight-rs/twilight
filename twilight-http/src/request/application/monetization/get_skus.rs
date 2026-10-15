use std::future::IntoFuture;

use twilight_model::{
    application::monetization::Sku,
    id::{Id, marker::ApplicationMarker},
};

use crate::{
    Client, Error, Response,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::{ResponseFuture, marker::ListBody},
};

pub(in crate::request) struct GetSKUsPath {
    application_id: Id<ApplicationMarker>,
}

pub struct GetSKUs<'a> {
    http: &'a Client,
    path: GetSKUsPath,
}

impl<'a> GetSKUs<'a> {
    pub(crate) const fn new(http: &'a Client, application_id: Id<ApplicationMarker>) -> Self {
        Self {
            http,
            path: GetSKUsPath { application_id },
        }
    }
}

impl IntoFuture for GetSKUs<'_> {
    type Output = Result<Response<ListBody<Sku>>, Error>;
    type IntoFuture = ResponseFuture<ListBody<Sku>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl Route for GetSKUs<'_> {
    type Fields = GetSKUsPath;

    const METHOD: Method = Method::Get;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("applications")
            .id(fields.application_id)
            .resource("skus")
            .build()
    }
}

impl TryIntoRequest for GetSKUs<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route_new::<Self>(self.path))
    }
}

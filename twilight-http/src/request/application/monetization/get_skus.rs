use std::future::IntoFuture;

use twilight_model::{
    application::monetization::Sku,
    id::{marker::ApplicationMarker, Id},
};

use crate::{
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, ResponseFuture},
    routing::Route,
    Client, Error, Response,
};

pub struct GetSKUs<'a> {
    application_id: Id<ApplicationMarker>,
    http: &'a Client,
}

impl<'a> GetSKUs<'a> {
    pub(crate) const fn new(http: &'a Client, application_id: Id<ApplicationMarker>) -> Self {
        Self {
            application_id,
            http,
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

impl TryIntoRequest for GetSKUs<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetSKUs {
            application_id: self.application_id.get(),
        }))
    }
}

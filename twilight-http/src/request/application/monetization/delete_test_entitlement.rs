use std::future::IntoFuture;

use twilight_model::id::{
    marker::{ApplicationMarker, EntitlementMarker},
    Id,
};

use crate::{
    request::{Request, TryIntoRequest},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
    Client, Error, Response,
};

pub struct DeleteTestEntitlement<'a> {
    application_id: Id<ApplicationMarker>,
    entitlement_id: Id<EntitlementMarker>,
    http: &'a Client,
}

impl<'a> DeleteTestEntitlement<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        entitlement_id: Id<EntitlementMarker>,
    ) -> Self {
        Self {
            application_id,
            entitlement_id,
            http,
        }
    }
}

impl IntoFuture for DeleteTestEntitlement<'_> {
    type Output = Result<Response<EmptyBody>, Error>;

    type IntoFuture = ResponseFuture<EmptyBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for DeleteTestEntitlement<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::DeleteTestEntitlement {
            application_id: self.application_id.get(),
            entitlement_id: self.entitlement_id.get(),
        })
        .build()
    }
}

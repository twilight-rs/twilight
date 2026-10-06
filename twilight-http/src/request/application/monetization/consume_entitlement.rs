use crate::{
    Client, Error, Response,
    request::{Request, TryIntoRequest},
    response::{ResponseFuture, marker::EmptyBody},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::id::{
    Id,
    marker::{ApplicationMarker, EntitlementMarker},
};

pub struct ConsumeEntitlement<'a> {
    application_id: Id<ApplicationMarker>,
    entitlement_id: Id<EntitlementMarker>,
    http: &'a Client,
}

impl<'a> ConsumeEntitlement<'a> {
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

impl IntoFuture for ConsumeEntitlement<'_> {
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

impl TryIntoRequest for ConsumeEntitlement<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::ConsumeEntitlement {
            application_id: self.application_id.get(),
            entitlement_id: self.entitlement_id.get(),
        }))
    }
}

use crate::{
    Client, Error, Response,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    application::monetization::Entitlement,
    id::{
        Id,
        marker::{ApplicationMarker, EntitlementMarker},
    },
};

/// Get an entitlement.
#[must_use = "requests must be configured and executed"]
pub struct GetEntitlement<'a> {
    application_id: Id<ApplicationMarker>,
    entitlement_id: Id<EntitlementMarker>,
    http: &'a Client,
}

impl<'a> GetEntitlement<'a> {
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

impl IntoFuture for GetEntitlement<'_> {
    type Output = Result<Response<Entitlement>, Error>;

    type IntoFuture = ResponseFuture<Entitlement>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetEntitlement<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::from_route(&Route::GetEntitlement {
            application_id: self.application_id.get(),
            entitlement_id: self.entitlement_id.get(),
        }))
    }
}

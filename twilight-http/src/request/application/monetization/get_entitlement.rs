use crate::{
    Client, Error, Response,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::ResponseFuture,
};
use std::future::IntoFuture;
use twilight_model::{
    application::monetization::Entitlement,
    id::{
        Id,
        marker::{ApplicationMarker, EntitlementMarker},
    },
};

struct GetEntitlementPath {
    application_id: Id<ApplicationMarker>,
    entitlement_id: Id<EntitlementMarker>,
}

/// Get an entitlement.
#[must_use = "requests must be configured and executed"]
pub struct GetEntitlement<'a> {
    http: &'a Client,
    path: GetEntitlementPath,
}

impl<'a> GetEntitlement<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        entitlement_id: Id<EntitlementMarker>,
    ) -> Self {
        Self {
            http,
            path: GetEntitlementPath {
                application_id,
                entitlement_id,
            },
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
        let GetEntitlementPath {
            application_id,
            entitlement_id,
        } = self.path;

        let path = Path::builder()
            .resource("applications")
            .id(application_id)
            .resource("entitlements")
            .id(entitlement_id)
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Get, path))
    }
}

use std::future::IntoFuture;

use twilight_model::id::{
    Id,
    marker::{ApplicationMarker, EntitlementMarker},
};

use crate::{
    Client, Error, Response,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{ResponseFuture, marker::EmptyBody},
};

pub struct DeleteTestEntitlementPath {
    application_id: Id<ApplicationMarker>,
    entitlement_id: Id<EntitlementMarker>,
}

pub struct DeleteTestEntitlement<'a> {
    http: &'a Client,
    path: DeleteTestEntitlementPath,
}

impl<'a> DeleteTestEntitlement<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        entitlement_id: Id<EntitlementMarker>,
    ) -> Self {
        Self {
            http,
            path: DeleteTestEntitlementPath {
                application_id,
                entitlement_id,
            },
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
        let path = Path::builder()
            .resource("applications")
            .id(self.path.application_id)
            .resource("entitlements")
            .id(self.path.entitlement_id)
            .build()
            .map_err(Error::path)?;

        Request::builder(Method::Delete, path).build()
    }
}

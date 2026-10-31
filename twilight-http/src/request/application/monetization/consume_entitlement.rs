use crate::{
    Client, Error, Response,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{ResponseFuture, marker::EmptyBody},
};
use std::future::IntoFuture;
use twilight_model::id::{
    Id,
    marker::{ApplicationMarker, EntitlementMarker},
};

struct ConsumeEntitlementPath {
    application_id: Id<ApplicationMarker>,
    entitlement_id: Id<EntitlementMarker>,
}

/// For One-Time Purchase consumable SKUs, marks a given entitlement for the
/// user as consumed.
///
/// The entitlement will have `consumed` as true when listing entitlements.
#[must_use = "requests must be configured and executed"]
pub struct ConsumeEntitlement<'a> {
    http: &'a Client,
    path: ConsumeEntitlementPath,
}

impl<'a> ConsumeEntitlement<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        entitlement_id: Id<EntitlementMarker>,
    ) -> Self {
        Self {
            http,
            path: ConsumeEntitlementPath {
                application_id,
                entitlement_id,
            },
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
        let ConsumeEntitlementPath {
            application_id,
            entitlement_id,
        } = self.path;

        let path = Path::builder()
            .resource("applications")
            .id(application_id)
            .resource("entitlements")
            .id(entitlement_id)
            .action("consume")
            .build()
            .map_err(Error::path)?;

        Ok(Request::new(Method::Post, path))
    }
}

use std::future::IntoFuture;

use twilight_model::id::{
    Id,
    marker::{ApplicationMarker, EntitlementMarker},
};

use crate::{
    Client, Error, Response,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::{ResponseFuture, marker::EmptyBody},
};

pub(crate) struct DeleteTestEntitlementFields {
    application_id: Id<ApplicationMarker>,
    entitlement_id: Id<EntitlementMarker>,
}

pub struct DeleteTestEntitlement<'a> {
    fields: DeleteTestEntitlementFields,
    http: &'a Client,
}

impl<'a> DeleteTestEntitlement<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        entitlement_id: Id<EntitlementMarker>,
    ) -> Self {
        Self {
            fields: DeleteTestEntitlementFields {
                application_id,
                entitlement_id,
            },
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

impl Route for DeleteTestEntitlement<'_> {
    type Fields = DeleteTestEntitlementFields;

    const METHOD: Method = Method::Delete;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("applications")
            .id(fields.application_id)
            .resource("entitlements")
            .id(fields.entitlement_id)
            .build()
    }
}

impl TryIntoRequest for DeleteTestEntitlement<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder_new::<Self>(self.fields).build()
    }
}

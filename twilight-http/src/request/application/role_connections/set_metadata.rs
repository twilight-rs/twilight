use crate::{
    client::Client,
    error::Error,
    request::{Request, RequestBuilder, TryIntoRequest},
    response::{marker::ListBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    application::role_connection::Metadata,
    id::{marker::ApplicationMarker, Id},
};

/// Set a user's linked roles metadata for the given application.
#[must_use = "requests must be configured and executed"]
pub struct SetMetadata<'a> {
    application_id: Id<ApplicationMarker>,
    http: &'a Client,
    records: &'a [Metadata],
}

impl<'a> SetMetadata<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        records: &'a [Metadata],
    ) -> Self {
        Self {
            application_id,
            http,
            records,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    #[deprecated(since = "0.14.0", note = "use `.await` or `into_future` instead")]
    pub fn exec(self) -> ResponseFuture<ListBody<Metadata>> {
        self.into_future()
    }
}

impl IntoFuture for SetMetadata<'_> {
    type Output = Result<Response<ListBody<Metadata>>, Error>;

    type IntoFuture = ResponseFuture<ListBody<Metadata>>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for SetMetadata<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::SetApplicationRoleConnectionMetadataRecords {
            application_id: self.application_id.get(),
        })
        .json(&self.records)
        .map(RequestBuilder::build)
    }
}

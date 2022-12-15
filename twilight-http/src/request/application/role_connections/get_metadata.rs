use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::ListBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    application::role_connection::Metadata,
    id::{marker::ApplicationMarker, Id},
};

/// Get Application Role Connection Metadata Records.
#[must_use = "requests must be configured and executed"]
pub struct GetMetadata<'a> {
    application_id: Id<ApplicationMarker>,
    http: &'a Client,
}

impl<'a> GetMetadata<'a> {
    pub(crate) const fn new(http: &'a Client, application_id: Id<ApplicationMarker>) -> Self {
        Self {
            application_id,
            http,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    #[deprecated(since = "0.14.0", note = "use `.await` or `into_future` instead")]
    pub fn exec(self) -> ResponseFuture<ListBody<Metadata>> {
        self.into_future()
    }
}

impl IntoFuture for GetMetadata<'_> {
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

impl TryIntoRequest for GetMetadata<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(
            Request::builder(&Route::GetApplicationRoleConnectionMetadataRecords {
                application_id: self.application_id.get(),
            })
            .build(),
        )
    }
}

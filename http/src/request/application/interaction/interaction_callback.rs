use crate::{
    client::Client,
    error::Error,
    request::{IntoRequest, Request},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{application::callback::InteractionResponse, id::InteractionId};

/// Respond to an interaction, by ID and token.
#[must_use = "requests must be configured and executed"]
pub struct InteractionCallback<'a> {
    interaction_id: InteractionId,
    interaction_token: &'a str,
    response: &'a InteractionResponse,
    http: &'a Client,
}

impl<'a> InteractionCallback<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        interaction_id: InteractionId,
        interaction_token: &'a str,
        response: &'a InteractionResponse,
    ) -> Self {
        Self {
            interaction_id,
            interaction_token,
            response,
            http,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        let http = self.http;

        match self.into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl IntoRequest for InteractionCallback<'_> {
    fn into_request(self) -> Result<Request, Error> {
        let request = Request::builder(&Route::InteractionCallback {
            interaction_id: self.interaction_id.get(),
            interaction_token: self.interaction_token,
        })
        .json(self.response)?
        .build();

        Ok(request)
    }
}

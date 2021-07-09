use crate::{
    client::Client,
    error::Error,
    request::Request,
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{application::callback::InteractionResponse, id::InteractionId};

/// Respond to an interaction, by ID and token.
pub struct InteractionCallback<'a> {
    interaction_id: InteractionId,
    interaction_token: String,
    response: InteractionResponse,
    http: &'a Client,
}

impl<'a> InteractionCallback<'a> {
    pub(crate) fn new(
        http: &'a Client,
        interaction_id: InteractionId,
        interaction_token: impl Into<String>,
        response: InteractionResponse,
    ) -> Self {
        Self {
            interaction_id,
            interaction_token: interaction_token.into(),
            response,
            http,
        }
    }

    // `self` needs to be consumed and the client returned due to parameters
    // being consumed in request construction.
    fn request(self) -> Result<(Request, &'a Client), Error> {
        let request = Request::builder(Route::InteractionCallback {
            interaction_id: self.interaction_id.0,
            interaction_token: self.interaction_token,
        })
        .json(&self.response)?
        .build();

        Ok((request, self.http))
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        match self.request() {
            Ok((request, client)) => client.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::{
    application::callback::InteractionResponse,
    id::{marker::InteractionMarker, Id},
};

/// Respond to an interaction, by its ID and token.
#[must_use = "requests must be configured and executed"]
pub struct CreateResponse<'a> {
    interaction_id: Id<InteractionMarker>,
    interaction_token: &'a str,
    response: &'a InteractionResponse,
    http: &'a Client,
}

impl<'a> CreateResponse<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        interaction_id: Id<InteractionMarker>,
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

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for CreateResponse<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let request = Request::builder(&Route::InteractionCallback {
            interaction_id: self.interaction_id.get(),
            interaction_token: self.interaction_token,
        })
        .json(self.response)?
        .use_authorization_token(false)
        .build();

        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use crate::{client::Client, request::TryIntoRequest};
    use std::error::Error;
    use twilight_http_ratelimiting::Path;
    use twilight_model::{application::callback::InteractionResponse, id::Id};

    #[test]
    fn test_interaction_callback() -> Result<(), Box<dyn Error>> {
        let application_id = Id::new(1);
        let interaction_id = Id::new(2);
        let token = "foo".to_owned().into_boxed_str();

        let client = Client::new(String::new());

        let sent_response = InteractionResponse::DeferredUpdateMessage;
        let req = client
            .interaction(application_id)
            .create_response(interaction_id, &token, &sent_response)
            .try_into_request()?;

        assert!(!req.use_authorization_token());
        assert_eq!(
            &Path::InteractionCallback(interaction_id.get()),
            req.ratelimit_path()
        );

        Ok(())
    }
}

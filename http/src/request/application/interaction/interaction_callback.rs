use crate::{
    client::Client,
    error::Error,
    request::Request,
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

    // `self` needs to be consumed and the client returned due to parameters
    // being consumed in request construction.
    fn request(&self) -> Result<Request, Error> {
        let request = Request::builder(&Route::InteractionCallback {
            interaction_id: self.interaction_id.get(),
            interaction_token: self.interaction_token,
        })
        .json(self.response)?
        .use_authorization_token(false)
        .build();

        Ok(request)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        match self.request() {
            Ok(request) => self.http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::client::Client;
    use std::error::Error;
    use twilight_http_ratelimiting::Path;
    use twilight_model::{
        application::callback::InteractionResponse,
        id::{ApplicationId, InteractionId},
    };

    #[test]
    fn test_interaction_callback() -> Result<(), Box<dyn Error>> {
        let application_id = ApplicationId::new(1).expect("non zero id");
        let interaction_id = InteractionId::new(2).expect("non zero id");
        let token = "foo".to_owned().into_boxed_str();

        let client = Client::new(String::new());

        let sent_response = InteractionResponse::DeferredUpdateMessage;
        let req = client
            .interaction(application_id)
            .interaction_callback(interaction_id, &token, &sent_response)
            .request()?;

        assert!(!req.use_authorization_token());
        assert_eq!(
            &Path::InteractionCallback(interaction_id.get()),
            req.ratelimit_path()
        );

        Ok(())
    }
}

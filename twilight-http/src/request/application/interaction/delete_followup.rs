use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{marker::EmptyBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::id::{
    marker::{ApplicationMarker, MessageMarker},
    Id,
};

/// Delete the original message, by its token.
///
/// This endpoint is not bound to the application's global rate limit.
///
/// # Examples
///
/// ```no_run
/// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::env;
/// use twilight_http::Client;
/// use twilight_model::id::Id;
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let application_id = Id::new(1);
///
/// client
///     .interaction(application_id)
///     .delete_followup("token here", Id::new(2))
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct DeleteFollowup<'a> {
    http: &'a Client,
    message_id: Id<MessageMarker>,
    token: &'a str,
    application_id: Id<ApplicationMarker>,
}

impl<'a> DeleteFollowup<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        token: &'a str,
        message_id: Id<MessageMarker>,
    ) -> Self {
        Self {
            http,
            message_id,
            token,
            application_id,
        }
    }
}

impl IntoFuture for DeleteFollowup<'_> {
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

impl TryIntoRequest for DeleteFollowup<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::DeleteWebhookMessage {
            message_id: self.message_id.get(),
            thread_id: None,
            token: self.token,
            webhook_id: self.application_id.get(),
        })
        .use_authorization_token(false)
        .build()
    }
}

#[cfg(test)]
mod tests {
    use super::DeleteFollowup;
    use crate::{
        client::Client,
        request::{Request, TryIntoRequest},
        routing::Route,
    };
    use std::error::Error;
    use twilight_model::id::Id;

    #[test]
    fn request() -> Result<(), Box<dyn Error>> {
        let client = Client::new("token".to_owned());

        let builder = DeleteFollowup::new(&client, Id::new(1), "token", Id::new(2));
        let actual = builder.try_into_request()?;

        let expected = Request::from_route(&Route::DeleteWebhookMessage {
            message_id: 2,
            thread_id: None,
            token: "token",
            webhook_id: 1,
        });

        assert_eq!(expected.path, actual.path);
        assert!(!actual.use_authorization_token());

        Ok(())
    }
}

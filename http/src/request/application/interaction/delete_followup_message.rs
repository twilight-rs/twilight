use crate::{
    client::Client,
    request::Request,
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use twilight_model::id::{
    marker::{ApplicationMarker, MessageMarker},
    Id,
};

/// Delete a followup message created from a interaction.
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
/// let application_id = Id::new(1).expect("non zero");
///
/// client
///     .interaction(application_id)
///     .delete_followup_message("token here", Id::new(2).expect("non zero"))
///     .exec()
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct DeleteFollowupMessage<'a> {
    http: &'a Client,
    message_id: Id<MessageMarker>,
    token: &'a str,
    application_id: Id<ApplicationMarker>,
}

impl<'a> DeleteFollowupMessage<'a> {
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

    fn request(self) -> Request {
        Request::builder(&Route::DeleteWebhookMessage {
            message_id: self.message_id.get(),
            thread_id: None,
            token: self.token,
            webhook_id: self.application_id.get(),
        })
        .use_authorization_token(false)
        .build()
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        self.http.request(self.request())
    }
}

#[cfg(test)]
mod tests {
    use super::DeleteFollowupMessage;
    use crate::{client::Client, request::Request, routing::Route};
    use twilight_model::id::Id;

    #[test]
    fn test_request() {
        let client = Client::new("token".to_owned());

        let builder = DeleteFollowupMessage::new(
            &client,
            Id::new(1).expect("non zero"),
            "token",
            Id::new(2).expect("non zero"),
        );
        let actual = builder.request();

        let expected = Request::from_route(&Route::DeleteWebhookMessage {
            message_id: 2,
            thread_id: None,
            token: "token",
            webhook_id: 1,
        });

        assert_eq!(expected.path, actual.path);
        assert!(!actual.use_authorization_token());
    }
}

use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
use twilight_model::{
    channel::Message,
    id::{ApplicationId, MessageId},
};

/// Get a followup message of an interaction.
///
/// # Examples
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::env;
/// use twilight_http::Client;
/// use twilight_http::request::AuditLogReason;
/// use twilight_model::id::{ApplicationId, MessageId};
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let application_id = ApplicationId::new(1).expect("non zero");
///
/// let response = client
///     .interaction(application_id)
///     .followup_message("token here", MessageId::new(2).expect("non zero"))
///     .exec()
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetFollowupMessage<'a> {
    application_id: ApplicationId,
    http: &'a Client,
    message_id: MessageId,
    interaction_token: &'a str,
}

impl<'a> GetFollowupMessage<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: ApplicationId,
        interaction_token: &'a str,
        message_id: MessageId,
    ) -> Self {
        Self {
            application_id,
            http,
            message_id,
            interaction_token,
        }
    }

    fn request(&self) -> Request {
        Request::builder(&Route::GetFollowupMessage {
            application_id: self.application_id.get(),
            interaction_token: self.interaction_token,
            thread_id: None,
            message_id: self.message_id.get(),
        })
        .use_authorization_token(false)
        .build()
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Message> {
        self.http.request(self.request())
    }
}

#[cfg(test)]
mod tests {
    use super::GetFollowupMessage;
    use crate::{client::Client, request::Request, routing::Route};
    use static_assertions::assert_impl_all;
    use twilight_model::id::{ApplicationId, MessageId};

    assert_impl_all!(GetFollowupMessage<'_>: Send, Sync);

    #[test]
    fn test_request() {
        const TOKEN: &str = "token";

        fn application_id() -> ApplicationId {
            ApplicationId::new(1).expect("non zero")
        }

        fn message_id() -> MessageId {
            MessageId::new(2).expect("non zero")
        }

        let client = Client::new("token".to_owned());

        let actual = client
            .interaction(application_id())
            .followup_message(TOKEN, message_id())
            .request();
        let expected = Request::builder(&Route::GetFollowupMessage {
            application_id: application_id().get(),
            interaction_token: TOKEN,
            thread_id: None,
            message_id: message_id().get(),
        })
        .use_authorization_token(false)
        .build();

        assert!(expected.body().is_none());
        assert_eq!(expected.path(), actual.path());
        assert_eq!(expected.ratelimit_path(), actual.ratelimit_path());
        assert_eq!(
            expected.use_authorization_token(),
            actual.use_authorization_token()
        );
    }
}

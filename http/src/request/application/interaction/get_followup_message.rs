use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use twilight_model::{
    channel::Message,
    id::{marker, Id},
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
/// use twilight_model::id::Id;
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let application_id = Id::new(1).expect("non zero");
///
/// let response = client
///     .interaction(application_id)
///     .followup_message("token here", Id::new(2).expect("non zero"))
///     .exec()
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetFollowupMessage<'a> {
    application_id: Id<marker::Application>,
    http: &'a Client,
    message_id: Id<marker::Message>,
    interaction_token: &'a str,
}

impl<'a> GetFollowupMessage<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<marker::Application>,
        interaction_token: &'a str,
        message_id: Id<marker::Message>,
    ) -> Self {
        Self {
            application_id,
            http,
            message_id,
            interaction_token,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Message> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetFollowupMessage<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Ok(Request::builder(&Route::GetFollowupMessage {
            application_id: self.application_id.get(),
            interaction_token: self.interaction_token,
            thread_id: None,
            message_id: self.message_id.get(),
        })
        .use_authorization_token(false)
        .build())
    }
}

#[cfg(test)]
mod tests {
    use super::GetFollowupMessage;
    use crate::{
        client::Client,
        request::{Request, TryIntoRequest},
        routing::Route,
    };
    use static_assertions::assert_impl_all;
    use std::error::Error;
    use twilight_model::id::{marker, Id};

    assert_impl_all!(GetFollowupMessage<'_>: Send, Sync);

    #[test]
    fn test_request() -> Result<(), Box<dyn Error>> {
        const TOKEN: &str = "token";

        fn application_id() -> Id<marker::Application> {
            Id::new(1).expect("non zero")
        }

        fn message_id() -> Id<marker::Message> {
            Id::new(2).expect("non zero")
        }

        let client = Client::new("token".to_owned());

        let actual = client
            .interaction(application_id())
            .followup_message(TOKEN, message_id())
            .try_into_request()?;
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

        Ok(())
    }
}

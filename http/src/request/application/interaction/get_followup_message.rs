use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
use twilight_model::{
    channel::Message,
    id::{
        marker::{ApplicationMarker, MessageMarker},
        Id,
    },
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
/// client.set_application_id(Id::new(1).expect("non zero"));
///
/// let response = client
///     .followup_message("token here", Id::new(2).expect("non zero"))?
///     .exec()
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetFollowupMessage<'a> {
    application_id: Id<ApplicationMarker>,
    http: &'a Client,
    message_id: Id<MessageMarker>,
    interaction_token: &'a str,
}

impl<'a> GetFollowupMessage<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        interaction_token: &'a str,
        message_id: Id<MessageMarker>,
    ) -> Self {
        Self {
            application_id,
            http,
            message_id,
            interaction_token,
        }
    }

    fn request(&self) -> Request {
        Request::from_route(&Route::GetFollowupMessage {
            application_id: self.application_id.get(),
            interaction_token: self.interaction_token,
            message_id: self.message_id.get(),
        })
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
    use std::error::Error;
    use twilight_model::id::{
        marker::{ApplicationMarker, MessageMarker},
        Id,
    };

    assert_impl_all!(GetFollowupMessage<'_>: Send, Sync);

    #[test]
    fn test_request() -> Result<(), Box<dyn Error>> {
        const TOKEN: &str = "token";

        fn application_id() -> Id<ApplicationMarker> {
            Id::new(1).expect("non zero")
        }

        fn message_id() -> Id<MessageMarker> {
            Id::new(2).expect("non zero")
        }

        let client = Client::new("token".to_owned());
        client.set_application_id(application_id());

        let actual = client.followup_message(TOKEN, message_id())?.request();
        let expected = Request::from_route(&Route::GetFollowupMessage {
            application_id: application_id().get(),
            interaction_token: TOKEN,
            message_id: message_id().get(),
        });

        assert!(expected.body().is_none());
        assert_eq!(expected.path(), actual.path());
        assert_eq!(expected.ratelimit_path(), actual.ratelimit_path());

        Ok(())
    }
}

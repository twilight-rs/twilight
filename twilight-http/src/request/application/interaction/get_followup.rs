use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::{
    channel::Message,
    id::{
        marker::{ApplicationMarker, MessageMarker},
        Id,
    },
};

/// Get a followup message of an interaction, by its token and the message ID.
///
/// This endpoint is not bound to the application's global rate limit.
///
/// # Examples
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::env;
/// use twilight_http::{request::AuditLogReason, Client};
/// use twilight_model::id::Id;
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// let application_id = Id::new(1);
///
/// let response = client
///     .interaction(application_id)
///     .followup("token here", Id::new(2))
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetFollowup<'a> {
    application_id: Id<ApplicationMarker>,
    http: &'a Client,
    message_id: Id<MessageMarker>,
    interaction_token: &'a str,
}

impl<'a> GetFollowup<'a> {
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
}

impl IntoFuture for GetFollowup<'_> {
    type Output = Result<Response<Message>, Error>;

    type IntoFuture = ResponseFuture<Message>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for GetFollowup<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder(&Route::GetFollowupMessage {
            application_id: self.application_id.get(),
            interaction_token: self.interaction_token,
            thread_id: None,
            message_id: self.message_id.get(),
        })
        .use_authorization_token(false)
        .build()
    }
}

#[cfg(test)]
mod tests {
    use super::GetFollowup;
    use crate::{
        client::Client,
        request::{Request, TryIntoRequest},
        routing::Route,
    };
    use static_assertions::assert_impl_all;
    use std::error::Error;
    use twilight_model::id::{
        marker::{ApplicationMarker, MessageMarker},
        Id,
    };

    assert_impl_all!(GetFollowup<'_>: Send, Sync);

    #[test]
    fn request() -> Result<(), Box<dyn Error>> {
        const APPLICATION_ID: Id<ApplicationMarker> = Id::new(1);
        const MESSAGE_ID: Id<MessageMarker> = Id::new(2);
        const TOKEN: &str = "token";

        let client = Client::new("token".to_owned());

        let actual = client
            .interaction(APPLICATION_ID)
            .followup(TOKEN, MESSAGE_ID)
            .try_into_request()?;
        let expected = Request::builder(&Route::GetFollowupMessage {
            application_id: APPLICATION_ID.get(),
            interaction_token: TOKEN,
            thread_id: None,
            message_id: MESSAGE_ID.get(),
        })
        .use_authorization_token(false)
        .build()?;

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

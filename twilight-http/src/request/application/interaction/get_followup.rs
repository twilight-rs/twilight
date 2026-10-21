use crate::{
    client::Client,
    error::Error,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    channel::Message,
    id::{
        Id,
        marker::{ApplicationMarker, MessageMarker},
    },
};

pub(in crate::request) struct GetFollowupPath<'a> {
    application_id: Id<ApplicationMarker>,
    interaction_token: &'a str,
    message_id: Id<MessageMarker>,
}

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
/// use twilight_http::{Client, request::AuditLogReason};
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
    http: &'a Client,
    path: GetFollowupPath<'a>,
}

impl<'a> GetFollowup<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        interaction_token: &'a str,
        message_id: Id<MessageMarker>,
    ) -> Self {
        Self {
            http,
            path: GetFollowupPath {
                application_id,
                interaction_token,
                message_id,
            },
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

impl<'a> Route for GetFollowup<'a> {
    type Fields = GetFollowupPath<'a>;

    const METHOD: Method = Method::Get;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("webhooks")
            .id(fields.application_id)
            .no_resource()
            .string_id(fields.interaction_token)
            .resource("messages")
            .id(fields.message_id)
            .build()
    }
}

impl TryIntoRequest for GetFollowup<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder_new::<Self>(self.path)
            .use_authorization_token(false)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::{GetFollowup, GetFollowupPath};
    use crate::{
        client::Client,
        request::{Request, TryIntoRequest},
    };
    use static_assertions::assert_impl_all;
    use std::error::Error;
    use twilight_model::id::{
        Id,
        marker::{ApplicationMarker, MessageMarker},
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
        let expected = Request::builder_new::<GetFollowup>(GetFollowupPath {
            application_id: APPLICATION_ID,
            interaction_token: TOKEN,
            message_id: MESSAGE_ID,
        })
        .use_authorization_token(false)
        .build()?;

        assert!(expected.body().is_none());
        assert_eq!(expected.path(), actual.path());
        assert_eq!(
            expected.use_authorization_token(),
            actual.use_authorization_token()
        );

        Ok(())
    }
}

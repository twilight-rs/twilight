use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
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

pub struct GetFollowupPath<'a> {
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

impl TryIntoRequest for GetFollowup<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let path = Path::builder()
            .resource("webhooks")
            .id(self.path.application_id)
            .no_resource()
            .id(self.path.interaction_token)
            .resource("messages")
            .id(self.path.message_id)
            .build()
            .map_err(Error::path)?;

        Request::builder(Method::Get, path)
            .use_authorization_token(false)
            .build()
    }
}

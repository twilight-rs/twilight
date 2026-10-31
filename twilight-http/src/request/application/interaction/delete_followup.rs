use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture, marker::EmptyBody},
};
use std::future::IntoFuture;
use twilight_model::id::{
    Id,
    marker::{ApplicationMarker, MessageMarker},
};

pub struct DeleteFollowupPath<'a> {
    application_id: Id<ApplicationMarker>,
    message_id: Id<MessageMarker>,
    token: &'a str,
}

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
    path: DeleteFollowupPath<'a>,
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
            path: DeleteFollowupPath {
                application_id,
                message_id,
                token,
            },
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
        let path = Path::builder()
            .resource("webhooks")
            .id(self.path.application_id)
            .no_resource()
            .id(self.path.token)
            .resource("messages")
            .id(self.path.message_id)
            .build()
            .map_err(Error::path)?;

        Request::builder(Method::Delete, path)
            .use_authorization_token(false)
            .build()
    }
}

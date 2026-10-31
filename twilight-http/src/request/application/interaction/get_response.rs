use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, path::Path},
    response::{Response, ResponseFuture},
};
use std::future::IntoFuture;
use twilight_model::{
    channel::Message,
    id::{Id, marker::ApplicationMarker},
};

pub struct GetResponsePath<'a> {
    application_id: Id<ApplicationMarker>,
    token: &'a str,
}

/// Get the original message, by its token.
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
/// client
///     .interaction(application_id)
///     .response("token here")
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetResponse<'a> {
    http: &'a Client,
    path: GetResponsePath<'a>,
}

impl<'a> GetResponse<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        interaction_token: &'a str,
    ) -> Self {
        Self {
            http,
            path: GetResponsePath {
                application_id,
                token: interaction_token,
            },
        }
    }
}

impl IntoFuture for GetResponse<'_> {
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

impl TryIntoRequest for GetResponse<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let path = Path::builder()
            .resource("webhooks")
            .id(self.path.application_id)
            .no_resource()
            .id(self.path.token)
            .resource("messages")
            .id("@original")
            .build()
            .map_err(Error::path)?;

        Request::builder(Method::Get, path)
            .use_authorization_token(false)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use crate::{client::Client, request::TryIntoRequest};
    use std::error::Error;
    use twilight_model::id::Id;

    #[test]
    fn delete_followup_message() -> Result<(), Box<dyn Error>> {
        let application_id = Id::new(1);
        let token = "foo".to_owned();

        let client = Client::new(String::new());
        let req = client
            .interaction(application_id)
            .response(&token)
            .try_into_request()?;

        assert!(!req.use_authorization_token());

        Ok(())
    }
}

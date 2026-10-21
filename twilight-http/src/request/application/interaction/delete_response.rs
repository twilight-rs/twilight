use crate::{
    client::Client,
    error::Error,
    request::{Method, Path, Request, Route, TryIntoRequest},
    response::{Response, ResponseFuture, marker::EmptyBody},
};
use std::future::IntoFuture;
use twilight_model::id::{Id, marker::ApplicationMarker};

pub(in crate::request) struct DeleteResponsePath<'a> {
    application_id: Id<ApplicationMarker>,
    token: &'a str,
}

/// Delete a followup message to an interaction, by its token and message ID.
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
///     .delete_response("token here")
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct DeleteResponse<'a> {
    http: &'a Client,
    path: DeleteResponsePath<'a>,
}

impl<'a> DeleteResponse<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: Id<ApplicationMarker>,
        token: &'a str,
    ) -> Self {
        Self {
            http,
            path: DeleteResponsePath {
                application_id,
                token,
            },
        }
    }
}

impl IntoFuture for DeleteResponse<'_> {
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

impl<'a> Route for DeleteResponse<'a> {
    type Fields = DeleteResponsePath<'a>;

    const METHOD: Method = Method::Delete;

    fn path(fields: Self::Fields) -> Path {
        Path::builder()
            .resource("webhooks")
            .id(fields.application_id)
            .no_resource()
            .string_id(fields.token)
            .resource("messages")
            .string_id("@original")
            .build()
    }
}

impl TryIntoRequest for DeleteResponse<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        Request::builder_new::<Self>(self.path)
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
            .delete_response(&token)
            .try_into_request()?;

        assert!(!req.use_authorization_token());

        Ok(())
    }
}

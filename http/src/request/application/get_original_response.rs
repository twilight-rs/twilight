use crate::{
    client::Client,
    error::Error,
    request::{PendingOption, Request},
    routing::Route,
};
use twilight_model::{channel::Message, id::ApplicationId};

/// Get the original message, by its token.
///
/// # Examples
///
/// ```no_run
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::env;
/// use twilight_http::Client;
/// use twilight_http::request::AuditLogReason;
/// use twilight_model::id::ApplicationId;
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// client.set_application_id(ApplicationId(1));
///
/// let message = client
///     .get_interaction_original("token here")?
///     .await?;
/// # Ok(()) }
/// ```
pub struct GetOriginalResponse<'a> {
    application_id: ApplicationId,
    fut: Option<PendingOption<'a>>,
    http: &'a Client,
    token: String,
}

impl<'a> GetOriginalResponse<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: ApplicationId,
        token: impl Into<String>,
    ) -> Self {
        Self {
            application_id,
            fut: None,
            http,
            token: token.into(),
        }
    }

    fn request(&self) -> Result<Request, Error> {
        let request = Request::from_route(Route::GetInteractionOriginal {
            application_id: self.application_id.0,
            interaction_token: self.token.clone(),
        });

        Ok(request)
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = self.request()?;
        self.fut.replace(Box::pin(self.http.request_bytes(request)));

        Ok(())
    }
}

poll_req!(opt, GetOriginalResponse<'_>, Message);

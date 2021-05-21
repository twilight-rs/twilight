use crate::{
    client::Client,
    error::Result,
    request::{Pending, Request},
    routing::Route,
};
use twilight_model::id::ApplicationId;

/// Delete a original interaction response.
///
/// # Examples
///
/// ```no_run
/// # use twilight_http::Client;
/// use twilight_http::request::AuditLogReason;
/// use twilight_model::id::ApplicationId;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let client = Client::new("token");
/// # client.set_application_id(ApplicationId(1));
/// client
///     .delete_interaction_original("token here")?
///     .await?;
/// # Ok(()) }
/// ```
pub struct DeleteOriginalResponse<'a> {
    application_id: ApplicationId,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
    token: String,
}

impl<'a> DeleteOriginalResponse<'a> {
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

    fn request(&self) -> Result<Request> {
        let request = Request::from_route(Route::DeleteInteractionOriginal {
            application_id: self.application_id.0,
            interaction_token: self.token.clone(),
        });

        Ok(request)
    }

    fn start(&mut self) -> Result<()> {
        let request = self.request()?;
        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

poll_req!(DeleteOriginalResponse<'_>, ());

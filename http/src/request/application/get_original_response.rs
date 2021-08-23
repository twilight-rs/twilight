use crate::{client::Client, request::Request, response::ResponseFuture, routing::Route};
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
/// client.set_application_id(ApplicationId::new(1).expect("non zero"));
///
/// let message = client
///     .get_interaction_original("token here")?
///     .exec()
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct GetOriginalResponse<'a> {
    application_id: ApplicationId,
    http: &'a Client,
    token: &'a str,
}

impl<'a> GetOriginalResponse<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        application_id: ApplicationId,
        interaction_token: &'a str,
    ) -> Self {
        Self {
            application_id,
            http,
            token: interaction_token,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<Message> {
        let request = Request::from_route(&Route::GetInteractionOriginal {
            application_id: self.application_id.get(),
            interaction_token: self.token,
        });

        self.http.request(request)
    }
}

use crate::{
    client::Client,
    error::Result,
    request::{Pending, Request},
    routing::Route,
};
use twilight_model::id::{ApplicationId, MessageId};

/// Delete a followup message created from a interaction.
///
/// # Examples
///
/// ```no_run
/// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use std::env;
/// use twilight_http::Client;
/// use twilight_model::id::{MessageId, ApplicationId};
///
/// let client = Client::new(env::var("DISCORD_TOKEN")?);
/// client
///     .delete_followup_message("token here", MessageId(2))?
///     .await?;
/// # Ok(()) }
/// ```
pub struct DeleteFollowupMessage<'a> {
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
    message_id: MessageId,
    token: String,
    application_id: ApplicationId,
}

impl<'a> DeleteFollowupMessage<'a> {
    pub(crate) fn new(
        http: &'a Client,
        application_id: ApplicationId,
        token: impl Into<String>,
        message_id: MessageId,
    ) -> Self {
        Self {
            fut: None,
            http,
            message_id,
            token: token.into(),
            application_id,
        }
    }

    fn request(&self) -> Result<Request> {
        let request = Request::from_route(Route::DeleteWebhookMessage {
            message_id: self.message_id.0,
            token: self.token.clone(),
            webhook_id: self.application_id.0,
        });

        Ok(request)
    }

    fn start(&mut self) -> Result<()> {
        let request = self.request()?;
        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

poll_req!(DeleteFollowupMessage<'_>, ());

#[cfg(test)]
mod tests {
    use super::DeleteFollowupMessage;
    use crate::{client::Client, request::Request, routing::Route};
    use twilight_model::id::{ApplicationId, MessageId};

    #[test]
    fn test_request() {
        let client = Client::new("token");

        let builder = DeleteFollowupMessage::new(&client, ApplicationId(1), "token", MessageId(2));
        let actual = builder.request().expect("failed to create request");

        let expected = Request::from_route(Route::DeleteWebhookMessage {
            message_id: 2,
            token: "token".to_owned(),
            webhook_id: 1,
        });

        assert_eq!(expected.body, actual.body);
        assert_eq!(expected.path, actual.path);
    }
}

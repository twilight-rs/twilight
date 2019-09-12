use dawn_model::id::WebhookId;
use super::prelude::*;

#[derive(Serialize)]
pub struct DeleteWebhook<'a> {
    token: Option<String>,
    #[serde(skip)]
    fut: Option<Pending<'a>>,
    #[serde(skip)]
    http: &'a Client,
    id: WebhookId,
}

impl<'a> DeleteWebhook<'a> {
    pub(crate) fn new(http: &'a Client, id: impl Into<WebhookId>) -> Self {
        Self {
            fut: None,
            http,
            id: id.into(),
            token: None,
        }
    }

    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token.replace(token.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(self.http.verify(Request {
            route: Route::DeleteWebhook {
                webhook_id: self.id.0,
                token: self.token.as_ref().map(AsRef::as_ref),
            },
            ..Default::default()
        })?);

        Ok(())
    }
}

poll_req!(DeleteWebhook<'_>, ());

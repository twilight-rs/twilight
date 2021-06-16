use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    response::marker::EmptyBody,
    routing::Route,
};
use twilight_model::{application::callback::InteractionResponse, id::InteractionId};

/// Respond to an interaction, by ID and token.
pub struct InteractionCallback<'a> {
    interaction_id: InteractionId,
    interaction_token: String,
    response: InteractionResponse,
    fut: Option<PendingResponse<'a, EmptyBody>>,
    http: &'a Client,
}

impl<'a> InteractionCallback<'a> {
    pub(crate) fn new(
        http: &'a Client,
        interaction_id: InteractionId,
        interaction_token: impl Into<String>,
        response: InteractionResponse,
    ) -> Self {
        Self {
            interaction_id,
            interaction_token: interaction_token.into(),
            response,
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::builder(Route::InteractionCallback {
            interaction_id: self.interaction_id.0,
            interaction_token: self.interaction_token.clone(),
        })
        .json(&self.response)?;

        self.fut
            .replace(Box::pin(self.http.request(request.build())));

        Ok(())
    }
}

poll_req!(InteractionCallback<'_>, EmptyBody);

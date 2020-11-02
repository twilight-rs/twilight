use crate::request::prelude::*;
use twilight_model::applications::InteractionResponse;
use twilight_model::id::*;

pub struct InteractionCallback<'a> {
    interaction_id: InteractionId,
    interaction_token: String,
    response: InteractionResponse,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
}

impl<'a> InteractionCallback<'a> {
    pub(crate) fn new(
        http: &'a Client,
        interaction_id: InteractionId,
        interaction_token: String,
        response: InteractionResponse,
    ) -> Self {
        Self {
            interaction_id,
            interaction_token,
            response,
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        let req = Request::from((
            crate::json_to_vec(&self.response)?,
            Route::InteractionCallback {
                interaction_id: self.interaction_id.0,
                interaction_token: self.interaction_token.clone(),
            },
        ));
        self.fut.replace(Box::pin(self.http.verify(req)));

        Ok(())
    }
}

poll_req!(InteractionCallback<'_>, ());

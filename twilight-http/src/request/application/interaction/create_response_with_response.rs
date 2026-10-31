use crate::{
    client::Client,
    error::Error,
    request::{Method, Request, TryIntoRequest, attachment::AttachmentManager, path::Path},
    response::{Response, ResponseFuture},
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    application::interaction::callback::InteractionCallbackResponse,
    http::interaction::InteractionResponse,
    id::{Id, marker::InteractionMarker},
};

#[derive(Serialize)]
#[serde(transparent)]
struct CreateResponseWithResponseBody<'a>(&'a InteractionResponse);

pub struct CreateResponseWithResponsePath<'a> {
    interaction_id: Id<InteractionMarker>,
    interaction_token: &'a str,
}

/// Respond to an interaction, by its ID and token.
///
/// This endpoint is not bound to the application's global rate limit.
#[must_use = "requests must be configured and executed"]
pub struct CreateResponseWithResponse<'a> {
    body: CreateResponseWithResponseBody<'a>,
    http: &'a Client,
    path: CreateResponseWithResponsePath<'a>,
}

impl<'a> CreateResponseWithResponse<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        interaction_id: Id<InteractionMarker>,
        interaction_token: &'a str,
        response: &'a InteractionResponse,
    ) -> Self {
        Self {
            body: CreateResponseWithResponseBody(response),
            http,
            path: CreateResponseWithResponsePath {
                interaction_id,
                interaction_token,
            },
        }
    }
}

impl IntoFuture for CreateResponseWithResponse<'_> {
    type Output = Result<Response<InteractionCallbackResponse>, Error>;

    type IntoFuture = ResponseFuture<InteractionCallbackResponse>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for CreateResponseWithResponse<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let path = Path::builder()
            .resource("interactions")
            .id(self.path.interaction_id)
            .no_resource()
            .id(self.path.interaction_token)
            .action("callback")
            .parameter(("with_response", true))
            .build()
            .map_err(Error::path)?;
        let mut request = Request::builder(Method::Post, path);

        // Interaction executions don't need the authorization token, only the
        // interaction token.
        request = request.use_authorization_token(false);

        // Determine whether we need to use a multipart/form-data body or a JSON
        // body.
        if let Some(attachments) = self
            .body
            .0
            .data
            .as_ref()
            .and_then(|data| data.attachments.as_ref())
        {
            let body = crate::json::to_vec(&self.body).map_err(Error::json)?;

            let form = AttachmentManager::new()
                .set_files(attachments.iter().collect())
                .build_form(&body);

            request = request.form(form);
        } else {
            request = request.json(&self.body);
        }

        request.build()
    }
}

#[cfg(test)]
mod tests {
    use crate::{client::Client, request::TryIntoRequest};
    use std::error::Error;
    use twilight_model::{
        http::interaction::{InteractionResponse, InteractionResponseType},
        id::Id,
    };

    #[test]
    fn interaction_callback() -> Result<(), Box<dyn Error>> {
        let application_id = Id::new(1);
        let interaction_id = Id::new(2);
        let token = "foo".to_owned().into_boxed_str();

        let client = Client::new(String::new());

        let response = InteractionResponse {
            kind: InteractionResponseType::DeferredUpdateMessage,
            data: None,
        };

        let req = client
            .interaction(application_id)
            .create_response(interaction_id, &token, &response)
            .with_response()
            .try_into_request()?;

        assert!(!req.use_authorization_token());

        Ok(())
    }
}

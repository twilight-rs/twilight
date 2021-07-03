use crate::{
    client::Client,
    error::Error,
    request::{PendingResponse, Request},
    routing::Route,
};
use twilight_model::user::User;

/// Get a user's information by id.
pub struct GetUser<'a> {
    fut: Option<PendingResponse<'a, User>>,
    http: &'a Client,
    target_user: String,
}

impl<'a> GetUser<'a> {
    pub(crate) fn new(http: &'a Client, target_user: impl Into<String>) -> Self {
        Self {
            fut: None,
            http,
            target_user: target_user.into(),
        }
    }

    fn start(&mut self) -> Result<(), Error> {
        let request = Request::from_route(Route::GetUser {
            target_user: self.target_user.clone(),
        });

        self.fut.replace(Box::pin(self.http.request(request)));

        Ok(())
    }
}

poll_req!(GetUser<'_>, User);

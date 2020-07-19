use crate::request::prelude::*;
use twilight_model::user::User;

/// Get a user's information by id.
pub struct GetUser<'a> {
    fut: Option<PendingOption<'a>>,
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

    fn start(&mut self) -> Result<()> {
        self.fut
            .replace(Box::pin(self.http.request_bytes(Request::from(
                Route::GetUser {
                    target_user: self.target_user.clone(),
                },
            ))));

        Ok(())
    }
}

poll_req!(opt, GetUser<'_>, User);

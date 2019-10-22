use crate::request::prelude::*;
use dawn_model::user::User;

pub struct GetUser<'a> {
    fut: Option<Pending<'a, Option<User>>>,
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
            .replace(Box::pin(self.http.request(Request::from(Route::GetUser {
                target_user: self.target_user.clone(),
            }))));

        Ok(())
    }
}

poll_req!(GetUser<'_>, Option<User>);

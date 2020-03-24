use crate::request::prelude::*;
use twilight_model::user::CurrentUser;

pub struct GetCurrentUser<'a> {
    fut: Option<Pending<'a, CurrentUser>>,
    http: &'a Client,
}

impl<'a> GetCurrentUser<'a> {
    pub(crate) fn new(http: &'a Client) -> Self {
        Self {
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut
            .replace(Box::pin(self.http.request(Request::from(Route::GetUser {
                target_user: "@me".to_owned(),
            }))));

        Ok(())
    }
}

poll_req!(GetCurrentUser<'_>, CurrentUser);

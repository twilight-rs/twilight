use crate::request::prelude::*;
use dawn_model::user::User;

#[derive(Default, Serialize)]
struct UpdateCurrentUserFields {
    avatar: Option<String>,
    username: Option<String>,
}

pub struct UpdateCurrentUser<'a> {
    fields: UpdateCurrentUserFields,
    fut: Option<Pending<'a, User>>,
    http: &'a Client,
}

impl<'a> UpdateCurrentUser<'a> {
    pub(crate) fn new(http: &'a Client) -> Self {
        Self {
            fields: UpdateCurrentUserFields::default(),
            fut: None,
            http,
        }
    }

    pub fn avatar(mut self, avatar: impl Into<String>) -> Self {
        self.fields.avatar.replace(avatar.into());

        self
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.fields.username.replace(username.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(&self.fields)?,
            Route::UpdateCurrentUser,
        )))));

        Ok(())
    }
}

poll_req!(UpdateCurrentUser<'_>, User);

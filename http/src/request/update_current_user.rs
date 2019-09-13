use dawn_model::user::User;
use super::prelude::*;

#[derive(Serialize)]
pub struct UpdateCurrentUser<'a> {
    avatar: Option<String>,
    username: Option<String>,
    #[serde(skip)]
    fut: Option<PendingBody<'a, User>>,
    #[serde(skip)]
    http: &'a Client,
}

impl<'a> UpdateCurrentUser<'a> {
    pub(crate) fn new(http: &'a Client) -> Self {
        Self {
            avatar: None,
            fut: None,
            http,
            username: None,
        }
    }

    pub fn avatar(mut self, avatar: impl Into<String>) -> Self {
        self.avatar.replace(avatar.into());

        self
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username.replace(username.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(self.http.request(Request::from((
            serde_json::to_vec(&self)?,
            Route::UpdateCurrentUser,
        )))?);

        Ok(())
    }
}

poll_req!(UpdateCurrentUser<'_>, User);

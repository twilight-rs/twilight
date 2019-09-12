use dawn_model::invite::Invite;
use super::prelude::*;

pub struct GetInvite<'a> {
    with_counts: bool,
    code: String,
    fut: Option<PendingBody<'a, Option<Invite>>>,
    http: &'a Client,
}

impl<'a> GetInvite<'a> {
    pub(crate) fn new(http: &'a Client, code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            fut: None,
            http,
            with_counts: false,
        }
    }

    pub fn with_counts(mut self) -> Self {
        self.with_counts = true;

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(self.http.request(Request {
            route: Route::GetInvite {
                code: &self.code,
                with_counts: self.with_counts,
            },
            ..Default::default()
        })?);

        Ok(())
    }
}

poll_req!(GetInvite<'_>, Option<Invite>);

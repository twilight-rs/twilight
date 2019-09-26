use super::prelude::*;
use dawn_model::invite::Invite;

pub struct GetInvite<'a> {
    with_counts: bool,
    code: String,
    fut: Option<Pin<Box<dyn Future<Output = Result<Option<Invite>>> + Send + 'a>>>,
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
        self.fut.replace(Box::pin(self.http.request(Request::from(
            Route::GetInvite {
                code: self.code.to_owned(),
                with_counts: self.with_counts,
            },
        ))));

        Ok(())
    }
}

poll_req!(GetInvite<'_>, Option<Invite>);

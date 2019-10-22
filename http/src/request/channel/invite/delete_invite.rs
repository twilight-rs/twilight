use crate::request::prelude::*;

pub struct DeleteInvite<'a> {
    code: String,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
}

impl<'a> DeleteInvite<'a> {
    pub(crate) fn new(http: &'a Client, code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            fut: None,
            http,
        }
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.verify(Request::from(
            Route::DeleteInvite {
                code: self.code.clone(),
            },
        ))));

        Ok(())
    }
}

poll_req!(DeleteInvite<'_>, ());

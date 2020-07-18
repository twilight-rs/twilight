use crate::request::prelude::*;
use std::borrow::Cow;

/// Delete an invite by its code.
pub struct DeleteInvite<'a> {
    code: Cow<'a, str>,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
    reason: Option<Cow<'a, str>>,
}

impl<'a> DeleteInvite<'a> {
    pub(crate) fn new(http: &'a Client, code: impl Into<Cow<'a, str>>) -> Self {
        Self {
            code: code.into(),
            fut: None,
            http,
            reason: None,
        }
    }

    /// Attach an audit log reason to this request.
    pub fn reason(mut self, reason: impl Into<Cow<'a, str>>) -> Self {
        self.reason.replace(reason.into());

        self
    }

    fn start(&mut self) -> Result<()> {
        let reason = self.reason.take();

        let request = if let Some(reason) = reason.as_deref() {
            let headers = audit_header(&reason)?;
            Request::from((headers, Route::DeleteInvite { code: &self.code }))
        } else {
            Request::from(Route::DeleteInvite { code: &self.code })
        };

        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

poll_req!(DeleteInvite<'_>, ());

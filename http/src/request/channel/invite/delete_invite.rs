use crate::request::prelude::*;

/// Delete an invite by its code.
pub struct DeleteInvite<'a> {
    code: String,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
    reason: Option<String>,
}

impl<'a> DeleteInvite<'a> {
    pub(crate) fn new(http: &'a Client, code: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            fut: None,
            http,
            reason: None,
        }
    }

    fn start(&mut self) -> Result<()> {
        let mut request = Request::builder(Route::DeleteInvite {
            code: self.code.clone(),
        });

        if let Some(reason) = &self.reason {
            request = request.headers(audit_header(reason)?);
        }

        self.fut
            .replace(Box::pin(self.http.verify(request.build())));

        Ok(())
    }
}

impl<'a> AuditLogReason for DeleteInvite<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        self.reason
            .replace(AuditLogReasonError::validate(reason.into())?);

        Ok(self)
    }
}

poll_req!(DeleteInvite<'_>, ());

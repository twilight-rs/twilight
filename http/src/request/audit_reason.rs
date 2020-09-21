pub trait AuditLogReason {
    fn reason(self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError>
    where
        Self: Sized;
}

impl AuditLogReasonError {
    /// The maximum audit log reason length in codepoints.
    pub const AUDIT_REASON_LENGTH: usize = 512;

    pub(crate) fn validate(reason: String) -> Result<String, AuditLogReasonError> {
        if reason.chars().count() <= Self::AUDIT_REASON_LENGTH {
            Ok(reason)
        } else {
            Err(AuditLogReasonError::ReasonInvalid { reason })
        }
    }
}

/// The error created when a reason can not be used as configured.
pub enum AuditLogReasonError {
    /// Returned when the reason is over 512 UTF-16 characters.
    ReasonInvalid { reason: String },
}

//! Constants, error types, and functions for validating Invite Target Users
//! requests.

use crate::request::ValidationError;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Maximum amount of users to add in a Bulk Add Target Users request.
pub const BULK_ADD_TARGET_USERS_LIMIT: usize = 1000;

/// Maximum amount of users to delete in a Bulk Delete Target Users request.
pub const BULK_DELETE_TARGET_USERS_LIMIT: usize = 1000;

/// A target user request is invalid.
#[derive(Debug)]
pub struct InviteTargetUsersValidationError {
    /// Type of error that occurred.
    kind: InviteTargetUsersValidationErrorType,
    /// Source of the error, if any.
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl InviteTargetUsersValidationError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &InviteTargetUsersValidationErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        self.source
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        InviteTargetUsersValidationErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, self.source)
    }

    /// Create a [`InviteTargetUsersValidationError`] from a [`ValidationError`].
    #[must_use = "has no effect if unused"]
    pub fn from_validation_error(
        kind: InviteTargetUsersValidationErrorType,
        source: ValidationError,
    ) -> Self {
        Self {
            kind,
            source: Some(Box::new(source)),
        }
    }
}

impl Display for InviteTargetUsersValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            InviteTargetUsersValidationErrorType::BulkAddTargetUsersIdsLimit { value } => {
                f.write_str("amount of user IDs provided is ")?;
                Display::fmt(value, f)?;
                f.write_str(" but must be at most ")?;

                Display::fmt(&BULK_ADD_TARGET_USERS_LIMIT, f)
            }
            InviteTargetUsersValidationErrorType::BulkDeleteTargetUsersIdsLimit { value } => {
                f.write_str("amount of user IDs provided is ")?;
                Display::fmt(value, f)?;
                f.write_str(" but must be at most ")?;

                Display::fmt(&BULK_DELETE_TARGET_USERS_LIMIT, f)
            }
        }
    }
}

impl Error for InviteTargetUsersValidationError {}

/// Type of [`InviteTargetUsersValidationError`] that occurred.
#[derive(Debug)]
pub enum InviteTargetUsersValidationErrorType {
    /// There are an invalid number of user IDs in a Bulk Add Target Users
    /// request.
    BulkAddTargetUsersIdsLimit {
        /// Invalid count.
        value: usize,
    },
    /// There are an invalid number of user IDs in a Bulk Delete Target Users
    /// request.
    BulkDeleteTargetUsersIdsLimit {
        /// Invalid count.
        value: usize,
    },
}

/// Ensure that the amount of user IDs in a Bulk Add Target Users request is
/// valid.
///
/// There must be at most [`BULK_ADD_TARGET_USERS_LIMIT`] user IDs. This is based on
/// [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`BulkAddTargetUsersIdsLimit`] if the length is invalid.
///
/// [`BulkAddTargetUsersIdsLimit`]: InviteTargetUsersValidationErrorType::BulkAddTargetUsersIdsLimit
/// [this documentation entry]: https://docs.discord.com/developers/resources/invite#bulk-add-target-users-json-params
pub fn bulk_add_target_users_ids(count: usize) -> Result<(), InviteTargetUsersValidationError> {
    if count > BULK_ADD_TARGET_USERS_LIMIT {
        return Err(InviteTargetUsersValidationError {
            kind: InviteTargetUsersValidationErrorType::BulkAddTargetUsersIdsLimit { value: count },
            source: None,
        });
    }

    Ok(())
}

/// Ensure that the amount of user IDs in a Bulk Delete Target Users request is
/// valid.
///
/// There must be at most [`BULK_DELETE_TARGET_USERS_LIMIT`] user IDs. This is based on
/// [this documentation entry].
///
/// # Errors
///
/// Returns an error of type [`BulkDeleteTargetUsersIdsLimit`] if the length is invalid.
///
/// [`BulkDeleteTargetUsersIdsLimit`]: InviteTargetUsersValidationErrorType::BulkDeleteTargetUsersIdsLimit
/// [this documentation entry]: https://docs.discord.com/developers/resources/invite#bulk-delete-target-users-json-params
pub fn bulk_delete_target_users_ids(count: usize) -> Result<(), InviteTargetUsersValidationError> {
    if count > BULK_DELETE_TARGET_USERS_LIMIT {
        return Err(InviteTargetUsersValidationError {
            kind: InviteTargetUsersValidationErrorType::BulkDeleteTargetUsersIdsLimit {
                value: count,
            },
            source: None,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bulk_add_target_users_ids_limit() {
        assert!(bulk_add_target_users_ids(0).is_ok());
        assert!(bulk_add_target_users_ids(1000).is_ok());

        assert!(matches!(
            bulk_add_target_users_ids(1001).unwrap_err().kind(),
            InviteTargetUsersValidationErrorType::BulkAddTargetUsersIdsLimit { value: 1001 }
        ));
    }

    #[test]
    fn bulk_delete_target_users_ids_limit() {
        assert!(bulk_delete_target_users_ids(0).is_ok());
        assert!(bulk_delete_target_users_ids(1000).is_ok());

        assert!(matches!(
            bulk_delete_target_users_ids(1001).unwrap_err().kind(),
            InviteTargetUsersValidationErrorType::BulkDeleteTargetUsersIdsLimit { value: 1001 }
        ));
    }
}

//! Requests for working with the target users of an invite.

pub mod get_target_users;
pub mod get_target_users_job_status;
pub mod update_target_users;

mod add_target_user;
mod bulk_add_target_users;
mod bulk_delete_target_users;
mod remove_target_user;

pub use self::{
    add_target_user::AddTargetUser, bulk_add_target_users::BulkAddTargetUsers,
    bulk_delete_target_users::BulkDeleteTargetUsers, get_target_users::GetTargetUsers,
    get_target_users_job_status::GetTargetUsersJobStatus, remove_target_user::RemoveTargetUser,
    update_target_users::UpdateTargetUsers,
};

use crate::{Error, error::ErrorType};
use csv::WriterBuilder;
use twilight_model::id::{Id, marker::UserMarker};

pub(in crate::request) fn file(target_user_ids: &[Id<UserMarker>]) -> Result<Vec<u8>, Error> {
    let mut writer = WriterBuilder::new()
        .has_headers(false)
        .from_writer(Vec::new());

    for target_user_id in target_user_ids {
        writer
            .write_record(&[target_user_id.to_string()])
            .map_err(|source| Error {
                kind: ErrorType::WritingCsv,
                source: Some(Box::new(source)),
            })?;
    }

    writer.into_inner().map_err(|source| Error {
        kind: ErrorType::WritingCsv,
        source: Some(Box::new(source)),
    })
}

//! Requests for working with invites.

mod delete_invite;
mod get_invite;

pub use self::{delete_invite::DeleteInvite, get_invite::GetInvite};

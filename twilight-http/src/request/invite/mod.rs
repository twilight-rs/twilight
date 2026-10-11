//! Requests for working with invites.

pub mod target_user;

mod delete_invite;
mod get_invite;

pub use self::{delete_invite::DeleteInvite, get_invite::GetInvite};

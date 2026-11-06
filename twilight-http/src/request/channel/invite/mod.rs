//! Requests for working with invites in the context of a channel.

mod create_channel_invite;
mod get_channel_invites;

pub use self::{
    create_channel_invite::CreateChannelInvite, get_channel_invites::GetChannelInvites,
};

#[deprecated(since = "0.17.2", note = "renamed to `CreateChannelInvite`")]
#[doc(hidden)]
pub use self::create_channel_invite::CreateChannelInvite as CreateInvite;

#[deprecated(since = "0.17.2", note = "moved to request::invite")]
#[doc(hidden)]
pub use crate::request::invite::{DeleteInvite, GetInvite};

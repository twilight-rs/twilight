//! Markers denoting the type of response body.
//!
//! Markers are used depending on the type of response that Twilight expects
//! from an endpoint. For example, [`DeleteRole`] responses have no body, so an
//! [`EmptyBody`] marker is used in the [`Response`]. For a request like
//! [`GetMember`] a [`MemberBody`] is used due to member deserialization
//! requiring special optimizations.
//!
//! [`DeleteRole`]: super::super::request::guild::role::DeleteRole
//! [`GetMember`]: super::super::request::guild::member::GetMember
//! [`Response`]: super::Response

use std::marker::PhantomData;

/// Marker that a response has no body. Responses with this marker can't be
/// deserialized.
///
/// Requests like [`AddRoleToMember`] or [`DeleteRole`] use this.
///
/// [`AddRoleToMember`]: crate::request::guild::member::AddRoleToMember
/// [`DeleteRole`]: crate::request::guild::role::DeleteRole
#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct EmptyBody;

/// Marker that a response has a list of something.
///
/// A `ListBody` is advantageous because it allows the retrieval of a
/// [`ModelIter`] to allow lazy deserialization. Instead of deserializing all of
/// the entries and collecting them into a `Vec` via [`Response::models`], you
/// can use [`Response::iter`] to iterate over and deserialize entries
/// on-demand. This saves processing time if the iterator is short-circuited.
/// Refer to the documentation for [`ModelIter`] for additional information.
///
/// May be used via the [`Response::iter`] or [`Response::models`] methods.
///
/// [`ModelIter`]: super::ModelIter
/// [`Response::iter`]: super::Response::<ListBody<T>>::iter
/// [`Response::models`]: super::Response::<ListBody<T>>::models
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ListBody<T> {
    phantom: PhantomData<T>,
}

/// Marker that a response has a member.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemberBody;

/// Marker that a response has a list of members.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemberListBody;

#[cfg(test)]
mod tests {
    use super::{EmptyBody, ListBody, MemberBody, MemberListBody};
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(EmptyBody: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(ListBody<String>: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(MemberBody: Clone, Debug, Eq, PartialEq, Send, Sync);
    assert_impl_all!(MemberListBody: Clone, Debug, Eq, PartialEq, Send, Sync);
}

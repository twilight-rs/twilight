use crate::{
    gateway::opcode::OpCode,
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
};
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Provided IDs is invalid for the request.
///
/// Returned by [`RequestGuildMembersBuilder::user_ids`].
#[derive(Debug)]
pub struct UserIdsError {
    kind: UserIdsErrorType,
}

impl UserIdsError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &UserIdsErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[allow(clippy::unused_self)]
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(self) -> (UserIdsErrorType, Option<Box<dyn Error + Send + Sync>>) {
        (self.kind, None)
    }

    const fn too_many(ids: Vec<Id<UserMarker>>) -> Self {
        Self {
            kind: UserIdsErrorType::TooMany { ids },
        }
    }
}

impl Display for UserIdsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            UserIdsErrorType::TooMany { ids } => {
                Display::fmt(&ids.len(), f)?;
                f.write_str(" user IDs were provided when only a maximum of 100 is allowed")
            }
        }
    }
}

impl Error for UserIdsError {}

/// Type of [`UserIdsError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum UserIdsErrorType {
    /// More than 100 user IDs were provided.
    TooMany {
        /// Provided list of user IDs.
        ids: Vec<Id<UserMarker>>,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RequestGuildMembers {
    pub d: RequestGuildMembersInfo,
    pub op: OpCode,
}

impl RequestGuildMembers {
    /// Create a new builder to configure a guild members request.
    ///
    /// This is an alias to [`RequestGuildMembersBuilder::new`]. Refer to its
    /// documentation for more information.
    pub const fn builder(guild_id: Id<GuildMarker>) -> RequestGuildMembersBuilder {
        RequestGuildMembersBuilder::new(guild_id)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestGuildMembersBuilder {
    guild_id: Id<GuildMarker>,
    nonce: Option<String>,
    presences: Option<bool>,
}

impl RequestGuildMembersBuilder {
    /// Create a new builder to configure and construct a
    /// [`RequestGuildMembers`].
    pub const fn new(guild_id: Id<GuildMarker>) -> Self {
        Self {
            guild_id,
            nonce: None,
            presences: None,
        }
    }

    /// Set the nonce to identify the member chunk response.
    ///
    /// By default, this uses Discord's default.
    #[must_use = "has no effect if not built into a RequestGuildMembers"]
    pub fn nonce(self, nonce: impl Into<String>) -> Self {
        self._nonce(nonce.into())
    }

    fn _nonce(mut self, nonce: String) -> Self {
        self.nonce.replace(nonce);

        self
    }

    /// Request that guild members' presences are included in member chunks.
    ///
    /// By default, this uses Discord's default.
    #[must_use = "has no effect if not built into a RequestGuildMembers"]
    pub fn presences(mut self, presences: bool) -> Self {
        self.presences.replace(presences);

        self
    }

    /// Consume the builder, creating a request for users whose usernames start
    /// with the provided string and optionally limiting the number of members
    /// to retrieve.
    ///
    /// If you specify no limit, then Discord's default will be used, which will
    /// be an unbounded number of members. Specifying 0 is also equivalent.
    ///
    /// To request the entire member list, pass in an empty string and no limit.
    /// You must also have the `GUILD_MEMBERS` intent enabled.
    ///
    /// # Examples
    ///
    /// Request all of the guild members that start with the letter "a" and
    /// their presences:
    ///
    /// ```
    /// use twilight_model::{gateway::payload::outgoing::RequestGuildMembers, id::Id};
    ///
    /// let request = RequestGuildMembers::builder(Id::new(1))
    ///     .presences(true)
    ///     .query("a", None);
    ///
    /// assert_eq!(Id::new(1), request.d.guild_id);
    /// assert_eq!(Some(0), request.d.limit);
    /// assert_eq!(Some("a"), request.d.query.as_deref());
    /// assert_eq!(Some(true), request.d.presences);
    /// ```
    pub fn query(self, query: impl Into<String>, limit: Option<u64>) -> RequestGuildMembers {
        self._query(query.into(), limit)
    }

    fn _query(self, query: String, limit: Option<u64>) -> RequestGuildMembers {
        RequestGuildMembers {
            d: RequestGuildMembersInfo {
                guild_id: self.guild_id,
                limit: Some(limit.unwrap_or_default()),
                nonce: self.nonce,
                presences: self.presences,
                query: Some(query),
                user_ids: None,
            },
            op: OpCode::RequestGuildMembers,
        }
    }

    /// Consume the builder, creating a request that requests the provided
    /// member in the specified guild(s).
    ///
    /// # Examples
    ///
    /// Request a member within a guild and specify a nonce of "test":
    ///
    /// ```
    /// use twilight_model::{
    ///     gateway::payload::outgoing::request_guild_members::{
    ///         RequestGuildMemberId, RequestGuildMembers,
    ///     },
    ///     id::Id,
    /// };
    ///
    /// let request = RequestGuildMembers::builder(Id::new(1))
    ///     .nonce("test")
    ///     .user_id(Id::new(2));
    ///
    /// assert_eq!(
    ///     Some(RequestGuildMemberId::One(Id::new(2))),
    ///     request.d.user_ids
    /// );
    /// ```
    #[allow(clippy::missing_const_for_fn)]
    pub fn user_id(self, user_id: Id<UserMarker>) -> RequestGuildMembers {
        RequestGuildMembers {
            d: RequestGuildMembersInfo {
                guild_id: self.guild_id,
                limit: None,
                nonce: self.nonce,
                presences: self.presences,
                query: None,
                user_ids: Some(RequestGuildMemberId::One(user_id)),
            },
            op: OpCode::RequestGuildMembers,
        }
    }

    /// Consume the builder, creating a request that requests the provided
    /// user(s) in the specified guild(s).
    ///
    /// Only up to 100 user IDs can be requested at once.
    ///
    /// # Examples
    ///
    /// Request two members within one guild and specify a nonce of "test":
    ///
    /// ```
    /// use twilight_model::{
    ///     gateway::payload::outgoing::request_guild_members::{
    ///         RequestGuildMemberId,
    ///         RequestGuildMembers,
    ///     },
    ///     id::Id,
    /// };
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let request = RequestGuildMembers::builder(Id::new(1))
    ///     .nonce("test")
    ///     .user_ids(vec![Id::new(2), Id::new(2)])?;
    ///
    /// assert!(matches!(request.d.user_ids, Some(RequestGuildMemberId::Multiple(ids)) if ids.len() == 2));
    /// # Ok(()) }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a [`UserIdsErrorType::TooMany`] error type if more than 100 user
    /// IDs were provided.
    pub fn user_ids(
        self,
        user_ids: impl Into<Vec<Id<UserMarker>>>,
    ) -> Result<RequestGuildMembers, UserIdsError> {
        self._user_ids(user_ids.into())
    }

    fn _user_ids(self, user_ids: Vec<Id<UserMarker>>) -> Result<RequestGuildMembers, UserIdsError> {
        if user_ids.len() > 100 {
            return Err(UserIdsError::too_many(user_ids));
        }

        Ok(RequestGuildMembers {
            d: RequestGuildMembersInfo {
                guild_id: self.guild_id,
                limit: None,
                nonce: self.nonce,
                presences: self.presences,
                query: None,
                user_ids: Some(RequestGuildMemberId::Multiple(user_ids)),
            },
            op: OpCode::RequestGuildMembers,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct RequestGuildMembersInfo {
    /// Guild ID.
    pub guild_id: Id<GuildMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Maximum number of members to request.
    pub limit: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presences: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<RequestGuildMemberId<Id<UserMarker>>>,
}

/// One or a list of IDs in a request.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum RequestGuildMemberId<T> {
    /// Single ID specified.
    One(T),
    /// List of IDs specified.
    Multiple(Vec<T>),
}

impl<T> From<T> for RequestGuildMemberId<T> {
    fn from(id: T) -> Self {
        Self::One(id)
    }
}

impl<T> From<Vec<T>> for RequestGuildMemberId<T> {
    fn from(ids: Vec<T>) -> Self {
        Self::Multiple(ids)
    }
}

#[cfg(test)]
mod tests {
    use super::RequestGuildMembersBuilder;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(
        RequestGuildMembersBuilder: Clone,
        Debug,
        Eq,
        PartialEq,
        Send,
        Sync
    );
}

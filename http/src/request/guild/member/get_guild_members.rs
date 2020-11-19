use crate::request::prelude::*;
use bytes::Bytes;
use serde::de::DeserializeSeed;
use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use twilight_model::{
    guild::member::{Member, MemberDeserializer},
    id::{GuildId, UserId},
};

#[cfg(not(feature = "simd-json"))]
use serde_json::Value;
#[cfg(feature = "simd-json")]
use simd_json::value::OwnedValue as Value;

/// The error created when the members can not be fetched as configured.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum GetGuildMembersError {
    /// The limit is either 0 or more than 1000.
    LimitInvalid {
        /// Provided limit.
        limit: u64,
    },
}

impl Display for GetGuildMembersError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Self::LimitInvalid { .. } => f.write_str("the limit is invalid"),
        }
    }
}

impl Error for GetGuildMembersError {}

#[derive(Default)]
struct GetGuildMembersFields {
    after: Option<UserId>,
    limit: Option<u64>,
    presences: Option<bool>,
}

/// Get the members of a guild, by id.
///
/// The upper limit to this request is 1000. If more than 1000 members are needed, the requests
/// must be chained. Discord defaults the limit to 1.
///
/// # Examples
///
/// Get the first 500 members of guild `100` after user ID `3000`:
///
/// ```rust,no_run
/// use twilight_http::Client;
/// use twilight_model::id::{GuildId, UserId};
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let client = Client::new("my token");
///
/// let guild_id = GuildId(100);
/// let user_id = UserId(3000);
/// let members = client.guild_members(guild_id).after(user_id).await?;
/// # Ok(()) }
/// ```
///
/// # Errors
///
/// Returns [`GetGuildMembersError::LimitInvalid`] if the limit is invalid.
pub struct GetGuildMembers<'a> {
    fields: GetGuildMembersFields,
    fut: Option<Pending<'a, Bytes>>,
    guild_id: GuildId,
    http: &'a Client,
}

impl<'a> GetGuildMembers<'a> {
    pub(crate) fn new(http: &'a Client, guild_id: GuildId) -> Self {
        Self {
            fields: GetGuildMembersFields::default(),
            fut: None,
            guild_id,
            http,
        }
    }

    /// Sets the user ID to get members after.
    pub fn after(mut self, after: UserId) -> Self {
        self.fields.after.replace(after);

        self
    }

    /// Sets the number of members to retrieve per request.
    ///
    /// The limit must be greater than 0 and less than 1000.
    ///
    /// # Errors
    ///
    /// Returns [`GetGuildMembersError::LimitInvalid`] if the limit is 0 or
    /// greater than 1000.
    pub fn limit(mut self, limit: u64) -> Result<Self, GetGuildMembersError> {
        if !validate::get_guild_members_limit(limit) {
            return Err(GetGuildMembersError::LimitInvalid { limit });
        }

        self.fields.limit.replace(limit);

        Ok(self)
    }

    /// Sets whether to retrieve matched member presences
    pub fn presences(mut self, presences: bool) -> Self {
        self.fields.presences.replace(presences);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut
            .replace(Box::pin(self.http.request_bytes(Request::from(
                Route::GetGuildMembers {
                    after: self.fields.after.map(|x| x.0),
                    guild_id: self.guild_id.0,
                    limit: self.fields.limit,
                    presences: self.fields.presences,
                },
            ))));

        Ok(())
    }
}

impl Future for GetGuildMembers<'_> {
    type Output = Result<Vec<Member>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.fut.is_none() {
            self.as_mut().start()?;
        }

        let fut = self.fut.as_mut().expect("future is created");

        match fut.as_mut().poll(cx) {
            Poll::Ready(res) => {
                let bytes = res?;
                let mut members = Vec::new();

                let mut bytes = bytes.as_ref().to_vec();
                let values = crate::json_from_slice::<Vec<Value>>(&mut bytes)?;

                for value in values {
                    let member_deserializer = MemberDeserializer::new(self.guild_id);
                    members.push(member_deserializer.deserialize(value)?);
                }

                Poll::Ready(Ok(members))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

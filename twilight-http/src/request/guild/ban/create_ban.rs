use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{marker::EmptyBody, Response, ResponseFuture},
    routing::Route,
};
use std::future::IntoFuture;
use twilight_model::id::{
    marker::{GuildMarker, UserMarker},
    Id,
};
use twilight_validate::request::{
    audit_reason as validate_audit_reason,
    create_guild_ban_delete_message_seconds as validate_create_guild_ban_delete_message_seconds,
    ValidationError,
};

struct CreateBanFields {
    delete_message_seconds: Option<u32>,
}

/// Bans a user from a guild, optionally with the number of days' worth of
/// messages to delete and the reason.
///
/// # Examples
///
/// Ban user `200` from guild `100`, deleting
/// 1 day's (`86_400` second's) worth of messages, for the reason `"memes"`:
///
/// ```no_run
/// use twilight_http::{request::AuditLogReason, Client};
/// use twilight_model::id::Id;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new("my token".to_owned());
///
/// let guild_id = Id::new(100);
/// let user_id = Id::new(200);
/// client
///     .create_ban(guild_id, user_id)
///     .delete_message_seconds(86_400)?
///     .reason("memes")?
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct CreateBan<'a> {
    fields: CreateBanFields,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    reason: Option<&'a str>,
    user_id: Id<UserMarker>,
}

impl<'a> CreateBan<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        user_id: Id<UserMarker>,
    ) -> Self {
        Self {
            fields: CreateBanFields {
                delete_message_seconds: None,
            },
            guild_id,
            http,
            reason: None,
            user_id,
        }
    }

    /// Set the number of seconds' worth of messages to delete.
    ///
    /// The number of seconds must be less than or equal to `604_800` (this is equivalent to `7` days).
    ///
    /// # Errors
    ///
    /// Returns an error of type [`CreateGuildBanDeleteMessageSeconds`] if the
    /// number of seconds is greater than `604_800` (this is equivalent to `7` days).
    ///
    /// [`CreateGuildBanDeleteMessageSeconds`]: twilight_validate::request::ValidationErrorType::CreateGuildBanDeleteMessageSeconds
    pub const fn delete_message_seconds(mut self, seconds: u32) -> Result<Self, ValidationError> {
        #[allow(clippy::question_mark)]
        if let Err(source) = validate_create_guild_ban_delete_message_seconds(seconds) {
            return Err(source);
        }

        self.fields.delete_message_seconds = Some(seconds);

        Ok(self)
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    #[deprecated(since = "0.14.0", note = "use `.await` or `into_future` instead")]
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        self.into_future()
    }
}

impl<'a> AuditLogReason<'a> for CreateBan<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, ValidationError> {
        validate_audit_reason(reason)?;

        self.reason.replace(reason);

        Ok(self)
    }
}

impl IntoFuture for CreateBan<'_> {
    type Output = Result<Response<EmptyBody>, Error>;

    type IntoFuture = ResponseFuture<EmptyBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for CreateBan<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::CreateBan {
            delete_message_seconds: self.fields.delete_message_seconds,
            guild_id: self.guild_id.get(),
            user_id: self.user_id.get(),
        });

        if let Some(reason) = self.reason.as_ref() {
            let header = request::audit_header(reason)?;

            request = request.headers(header);
        }

        Ok(request.build())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        client::Client,
        request::{AuditLogReason, TryIntoRequest, REASON_HEADER_NAME},
    };
    use hyper::header::HeaderValue;
    use std::error::Error;
    use twilight_http_ratelimiting::Method;
    use twilight_model::id::{
        marker::{GuildMarker, UserMarker},
        Id,
    };

    #[test]
    fn request() -> Result<(), Box<dyn Error>> {
        const GUILD_ID: Id<GuildMarker> = Id::new(1);
        const REASON: &str = "spam";
        const USER_ID: Id<UserMarker> = Id::new(2);

        let client = Client::new(String::new());
        let request = client
            .create_ban(GUILD_ID, USER_ID)
            .reason(REASON)?
            .try_into_request()?;

        assert!(request.body().is_none());
        assert!(request.form().is_none());
        assert_eq!(Method::Put, request.method());

        let header = HeaderValue::from_static(REASON);
        assert!(matches!(
            request.headers(),
            Some(map)
            if map.len() == 1 && map.get(REASON_HEADER_NAME) == Some(&header)));
        assert!(request.use_authorization_token());

        Ok(())
    }
}

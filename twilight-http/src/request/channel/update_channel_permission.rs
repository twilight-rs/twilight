use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{marker::EmptyBody, Response, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use std::future::IntoFuture;
use twilight_model::{
    guild::Permissions,
    http::permission_overwrite::{PermissionOverwrite, PermissionOverwriteType},
    id::{
        marker::{ChannelMarker, GenericMarker},
        Id,
    },
};
use twilight_validate::request::{audit_reason as validate_audit_reason, ValidationError};

#[derive(Serialize)]
struct UpdateChannelPermissionFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow: Option<Permissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deny: Option<Permissions>,
    #[serde(rename = "type")]
    kind: PermissionOverwriteType,
}

/// Update the permissions for a role or a user in a channel.
///
/// # Examples:
///
/// Create permission overrides for a role to view the channel, but not send
/// messages:
///
/// ```no_run
/// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # use twilight_http::Client;
/// # let client = Client::new("my token".to_owned());
/// #
/// use twilight_model::{
///     guild::Permissions,
///     http::permission_overwrite::{PermissionOverwrite, PermissionOverwriteType},
///     id::Id,
/// };
///
/// let channel_id = Id::new(123);
/// let permission_overwrite = PermissionOverwrite {
///     allow: Some(Permissions::VIEW_CHANNEL),
///     deny: Some(Permissions::SEND_MESSAGES),
///     id: Id::new(432),
///     kind: PermissionOverwriteType::Role,
/// };
///
/// client
///     .update_channel_permission(channel_id, &permission_overwrite)
///     .await?;
/// # Ok(()) }
/// ```
#[must_use = "requests must be configured and executed"]
pub struct UpdateChannelPermission<'a> {
    channel_id: Id<ChannelMarker>,
    fields: UpdateChannelPermissionFields,
    http: &'a Client,
    reason: Result<Option<&'a str>, ValidationError>,
    target_id: Id<GenericMarker>,
}

impl<'a> UpdateChannelPermission<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        permission_overwrite: &PermissionOverwrite,
    ) -> Self {
        Self {
            channel_id,
            http,
            fields: UpdateChannelPermissionFields {
                allow: permission_overwrite.allow,
                deny: permission_overwrite.deny,
                kind: permission_overwrite.kind,
            },
            reason: Ok(None),
            target_id: permission_overwrite.id,
        }
    }
}

impl<'a> AuditLogReason<'a> for UpdateChannelPermission<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl IntoFuture for UpdateChannelPermission<'_> {
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

impl TryIntoRequest for UpdateChannelPermission<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::UpdatePermissionOverwrite {
            channel_id: self.channel_id.get(),
            target_id: self.target_id.get(),
        })
        .json(&self.fields);

        if let Some(reason) = self.reason.map_err(Error::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request() {
        let permission_overwrite = PermissionOverwrite {
            allow: None,
            deny: Some(Permissions::SEND_MESSAGES),
            id: Id::new(2),
            kind: PermissionOverwriteType::Member,
        };

        let client = Client::new("foo".to_owned());
        let builder = UpdateChannelPermission::new(&client, Id::new(1), &permission_overwrite);
        let actual = builder
            .try_into_request()
            .expect("failed to create request");

        let body = crate::json::to_vec(&UpdateChannelPermissionFields {
            allow: None,
            deny: Some(Permissions::SEND_MESSAGES),
            kind: PermissionOverwriteType::Member,
        })
        .expect("failed to serialize payload");
        let route = Route::UpdatePermissionOverwrite {
            channel_id: 1,
            target_id: 2,
        };
        let expected = Request::builder(&route).body(body).build().unwrap();

        assert_eq!(expected.body, actual.body);
        assert_eq!(expected.path, actual.path);
    }
}

use crate::{
    client::Client,
    error::Error,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::{marker::EmptyBody, ResponseFuture},
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    channel::permission_overwrite::{PermissionOverwriteTargetType, PermissionOverwriteType},
    guild::Permissions,
    id::{marker::ChannelMarker, Id},
};
use twilight_validate::request::{audit_reason as validate_audit_reason, ValidationError};

#[derive(Serialize)]
struct UpdateChannelPermissionConfiguredFields {
    allow: Permissions,
    deny: Permissions,
    #[serde(rename = "type")]
    kind: PermissionOverwriteTargetType,
}

/// Created when either `member` or `role` is called on a `DeleteChannelPermission` struct.
#[must_use = "requests must be configured and executed"]
pub struct UpdateChannelPermissionConfigured<'a> {
    channel_id: Id<ChannelMarker>,
    fields: UpdateChannelPermissionConfiguredFields,
    http: &'a Client,
    target_id: u64,
    reason: Option<&'a str>,
}

impl<'a> UpdateChannelPermissionConfigured<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        channel_id: Id<ChannelMarker>,
        allow: Permissions,
        deny: Permissions,
        target: PermissionOverwriteType,
    ) -> Self {
        let (name, target_id) = match target {
            PermissionOverwriteType::Member(user_id) => {
                (PermissionOverwriteTargetType::Member, user_id.get())
            }
            PermissionOverwriteType::Role(role_id) => {
                (PermissionOverwriteTargetType::Role, role_id.get())
            }
        };

        Self {
            channel_id,
            fields: UpdateChannelPermissionConfiguredFields {
                allow,
                deny,
                kind: name,
            },
            http,
            target_id,
            reason: None,
        }
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    pub fn exec(self) -> ResponseFuture<EmptyBody> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl<'a> AuditLogReason<'a> for UpdateChannelPermissionConfigured<'a> {
    fn reason(mut self, reason: &'a str) -> Result<Self, ValidationError> {
        validate_audit_reason(reason)?;

        self.reason.replace(reason);

        Ok(self)
    }
}

impl TryIntoRequest for UpdateChannelPermissionConfigured<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let mut request = Request::builder(&Route::UpdatePermissionOverwrite {
            channel_id: self.channel_id.get(),
            target_id: self.target_id,
        })
        .json(&self.fields)?;

        if let Some(reason) = &self.reason {
            request = request.headers(request::audit_header(reason)?);
        }

        Ok(request.build())
    }
}

#[cfg(test)]
mod tests {
    use super::{UpdateChannelPermissionConfigured, UpdateChannelPermissionConfiguredFields};
    use crate::{
        request::{Request, TryIntoRequest},
        routing::Route,
        Client,
    };
    use twilight_model::{
        channel::permission_overwrite::{PermissionOverwriteTargetType, PermissionOverwriteType},
        guild::Permissions,
        id::Id,
    };

    #[test]
    fn test_request() {
        let client = Client::new("foo".to_owned());
        let builder = UpdateChannelPermissionConfigured::new(
            &client,
            Id::new(1),
            Permissions::empty(),
            Permissions::SEND_MESSAGES,
            PermissionOverwriteType::Member(Id::new(2)),
        );
        let actual = builder
            .try_into_request()
            .expect("failed to create request");

        let body = crate::json::to_vec(&UpdateChannelPermissionConfiguredFields {
            allow: Permissions::empty(),
            deny: Permissions::SEND_MESSAGES,
            kind: PermissionOverwriteTargetType::Member,
        })
        .expect("failed to serialize payload");
        let route = Route::UpdatePermissionOverwrite {
            channel_id: 1,
            target_id: 2,
        };
        let expected = Request::builder(&route).body(body).build();

        assert_eq!(expected.body, actual.body);
        assert_eq!(expected.path, actual.path);
    }
}

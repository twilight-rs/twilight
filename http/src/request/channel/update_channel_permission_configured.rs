use crate::request::prelude::*;
use twilight_model::{
    channel::permission_overwrite::{PermissionOverwriteType, PermissionOverwriteTypeName},
    guild::Permissions,
    id::ChannelId,
};

#[derive(Serialize)]
struct UpdateChannelPermissionConfiguredFields {
    allow: Permissions,
    deny: Permissions,
    #[serde(rename = "type")]
    kind: PermissionOverwriteTypeName,
}

/// Created when either `member` or `role` is called on a `DeleteChannelPermission` struct.
pub struct UpdateChannelPermissionConfigured<'a> {
    channel_id: ChannelId,
    fields: UpdateChannelPermissionConfiguredFields,
    fut: Option<Pending<'a, ()>>,
    http: &'a Client,
    target_id: u64,
    reason: Option<String>,
}

impl<'a> UpdateChannelPermissionConfigured<'a> {
    pub(crate) fn new(
        http: &'a Client,
        channel_id: ChannelId,
        allow: Permissions,
        deny: Permissions,
        target: &PermissionOverwriteType,
    ) -> Self {
        let (name, target_id) = match target {
            PermissionOverwriteType::Member(user_id) => {
                (PermissionOverwriteTypeName::Member, user_id.0)
            }
            PermissionOverwriteType::Role(role_id) => {
                (PermissionOverwriteTypeName::Role, role_id.0)
            }
        };

        Self {
            channel_id,
            fields: UpdateChannelPermissionConfiguredFields {
                allow,
                deny,
                kind: name,
            },
            fut: None,
            http,
            target_id,
            reason: None,
        }
    }

    fn request(&self) -> Result<Request> {
        Ok(if let Some(reason) = &self.reason {
            let headers = audit_header(&reason)?;
            Request::from((
                crate::json_to_vec(&self.fields)?,
                headers,
                Route::UpdatePermissionOverwrite {
                    channel_id: self.channel_id.0,
                    target_id: self.target_id,
                },
            ))
        } else {
            Request::from((
                crate::json_to_vec(&self.fields)?,
                Route::UpdatePermissionOverwrite {
                    channel_id: self.channel_id.0,
                    target_id: self.target_id,
                },
            ))
        })
    }

    fn start(&mut self) -> Result<()> {
        let request = self.request()?;

        self.fut.replace(Box::pin(self.http.verify(request)));

        Ok(())
    }
}

impl<'a> AuditLogReason for UpdateChannelPermissionConfigured<'a> {
    fn reason(mut self, reason: impl Into<String>) -> Result<Self, AuditLogReasonError> {
        self.reason
            .replace(AuditLogReasonError::validate(reason.into())?);

        Ok(self)
    }
}

poll_req!(UpdateChannelPermissionConfigured<'_>, ());

#[cfg(test)]
mod tests {
    use super::{UpdateChannelPermissionConfigured, UpdateChannelPermissionConfiguredFields};
    use crate::{request::Request, routing::Route, Client};
    use twilight_model::{
        channel::permission_overwrite::{PermissionOverwriteType, PermissionOverwriteTypeName},
        guild::Permissions,
        id::{ChannelId, UserId},
    };

    #[test]
    fn test_request() {
        let client = Client::new("foo");
        let builder = UpdateChannelPermissionConfigured::new(
            &client,
            ChannelId(1),
            Permissions::empty(),
            Permissions::SEND_MESSAGES,
            &PermissionOverwriteType::Member(UserId(2)),
        );
        let actual = builder.request().expect("failed to create request");

        let body = crate::json_to_vec(&UpdateChannelPermissionConfiguredFields {
            allow: Permissions::empty(),
            deny: Permissions::SEND_MESSAGES,
            kind: PermissionOverwriteTypeName::Member,
        })
        .expect("failed to serialize payload");
        let route = Route::UpdatePermissionOverwrite {
            channel_id: 1,
            target_id: 2,
        };
        let expected = Request::from((body, route));

        assert_eq!(expected.body, actual.body);
        assert_eq!(expected.path, actual.path);
    }
}

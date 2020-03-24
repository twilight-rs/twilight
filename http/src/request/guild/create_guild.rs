use crate::request::prelude::*;
use twilight_model::{
    channel::GuildChannel,
    guild::{
        DefaultMessageNotificationLevel,
        ExplicitContentFilter,
        PartialGuild,
        Role,
        VerificationLevel,
    },
};

#[derive(Serialize)]
struct CreateGuildFields {
    channels: Option<Vec<GuildChannel>>,
    default_message_notifications: Option<DefaultMessageNotificationLevel>,
    explicit_content_filter: Option<ExplicitContentFilter>,
    icon: Option<String>,
    name: String,
    region: Option<String>,
    roles: Option<Vec<Role>>,
    verification_level: Option<VerificationLevel>,
}

pub struct CreateGuild<'a> {
    fields: CreateGuildFields,
    fut: Option<Pending<'a, PartialGuild>>,
    http: &'a Client,
}

impl<'a> CreateGuild<'a> {
    pub(crate) fn new(http: &'a Client, name: impl Into<String>) -> Self {
        Self {
            fields: CreateGuildFields {
                channels: None,
                default_message_notifications: None,
                explicit_content_filter: None,
                icon: None,
                name: name.into(),
                region: None,
                roles: None,
                verification_level: None,
            },
            fut: None,
            http,
        }
    }

    pub fn channels(mut self, channels: Vec<GuildChannel>) -> Self {
        self.fields.channels.replace(channels);

        self
    }

    pub fn default_message_notifications(
        mut self,
        default_message_notifications: DefaultMessageNotificationLevel,
    ) -> Self {
        self.fields
            .default_message_notifications
            .replace(default_message_notifications);

        self
    }

    pub fn explicit_content_filter(
        mut self,
        explicit_content_filter: ExplicitContentFilter,
    ) -> Self {
        self.fields
            .explicit_content_filter
            .replace(explicit_content_filter);

        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.fields.icon.replace(icon.into());

        self
    }

    pub fn region(mut self, region: impl Into<String>) -> Self {
        self.fields.region.replace(region.into());

        self
    }

    pub fn roles(mut self, roles: Vec<Role>) -> Self {
        self.fields.roles.replace(roles);

        self
    }

    fn start(&mut self) -> Result<()> {
        self.fut.replace(Box::pin(self.http.request(Request::from((
            serde_json::to_vec(&self.fields)?,
            Route::CreateGuild,
        )))));

        Ok(())
    }
}

poll_req!(CreateGuild<'_>, PartialGuild);

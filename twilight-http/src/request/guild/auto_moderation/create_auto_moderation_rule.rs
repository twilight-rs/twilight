use crate::{
    client::Client,
    error::Error as HttpError,
    request::{self, AuditLogReason, Request, TryIntoRequest},
    response::ResponseFuture,
    routing::Route,
};
use serde::Serialize;
use twilight_model::{
    guild::auto_moderation::{
        AutoModerationActionType, AutoModerationEventType, AutoModerationKeywordPresetType,
        AutoModerationRule, AutoModerationTriggerType,
    },
    id::{
        marker::{ChannelMarker, GuildMarker, RoleMarker},
        Id,
    },
};
use twilight_validate::request::{
    audit_reason as validate_audit_reason,
    auto_moderation_action_metadata_duration_seconds as validate_auto_moderation_action_metadata_duration_seconds,
    auto_moderation_block_action_custom_message_limit as validate_auto_moderation_block_action_custom_message_limit,
    auto_moderation_exempt_channels as validate_auto_moderation_exempt_channels,
    auto_moderation_exempt_roles as validate_auto_moderation_exempt_roles,
    auto_moderation_metadata_keyword_allow_list as validate_auto_moderation_metadata_keyword_allow_list,
    auto_moderation_metadata_keyword_filter as validate_auto_moderation_metadata_keyword_filter,
    auto_moderation_metadata_mention_total_limit as validate_auto_moderation_metadata_mention_total_limit,
    auto_moderation_metadata_regex_patterns as validate_auto_moderation_metadata_regex_patterns,
    ValidationError,
};

#[derive(Serialize)]
struct CreateAutoModerationRuleFieldsAction {
    /// Type of action.
    #[serde(rename = "type")]
    pub kind: AutoModerationActionType,
    /// Additional metadata needed during execution for this specific action
    /// type.
    pub metadata: CreateAutoModerationRuleFieldsActionMetadata,
}

#[derive(Default, Serialize)]
struct CreateAutoModerationRuleFieldsActionMetadata {
    /// Channel to which user content should be logged.
    pub channel_id: Option<Id<ChannelMarker>>,
    /// Additional explanation that will be shown to members whenever their message is blocked.
    ///
    /// Maximum value length is 150 characters.
    pub custom_message: Option<String>,
    /// Timeout duration in seconds.
    ///
    /// Maximum value is 2419200 seconds, or 4 weeks.
    pub duration_seconds: Option<u32>,
}

#[derive(Serialize)]
struct CreateAutoModerationRuleFieldsTriggerMetadata<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_list: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keyword_filter: Option<&'a [&'a str]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    presets: Option<&'a [AutoModerationKeywordPresetType]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mention_total_limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    regex_patterns: Option<&'a [&'a str]>,
}

#[derive(Serialize)]
struct CreateAutoModerationRuleFields<'a> {
    actions: Option<Vec<CreateAutoModerationRuleFieldsAction>>,
    enabled: Option<bool>,
    event_type: AutoModerationEventType,
    exempt_channels: Option<&'a [Id<ChannelMarker>]>,
    exempt_roles: Option<&'a [Id<RoleMarker>]>,
    name: &'a str,
    trigger_metadata: Option<CreateAutoModerationRuleFieldsTriggerMetadata<'a>>,
    trigger_type: Option<AutoModerationTriggerType>,
}

/// Create an auto moderation rule within a guild.
///
/// Requires the [`MANAGE_GUILD`] permission.
///
/// # Examples
///
/// Create a rule that deletes messages that contain the word "darn":
///
/// ```no_run
/// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// use twilight_http::Client;
/// use twilight_model::{guild::auto_moderation::AutoModerationEventType, id::Id};
///
/// let client = Client::new("my token".to_owned());
///
/// let guild_id = Id::new(1);
/// client
///     .create_auto_moderation_rule(guild_id, "no darns", AutoModerationEventType::MessageSend)
///     .action_block_message()
///     .enabled(true)
///     .with_keyword(&["darn"], &["d(?:4|a)rn"], &["darn it"])
///     .await?;
/// # Ok(()) }
/// ```
///
/// [`MANAGE_GUILD`]: twilight_model::guild::Permissions::MANAGE_GUILD
#[must_use = "requests must be configured and executed"]
pub struct CreateAutoModerationRule<'a> {
    fields: Result<CreateAutoModerationRuleFields<'a>, ValidationError>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
    reason: Result<Option<&'a str>, ValidationError>,
}

impl<'a> CreateAutoModerationRule<'a> {
    pub(crate) const fn new(
        http: &'a Client,
        guild_id: Id<GuildMarker>,
        name: &'a str,
        event_type: AutoModerationEventType,
    ) -> Self {
        Self {
            fields: Ok(CreateAutoModerationRuleFields {
                actions: None,
                enabled: None,
                event_type,
                exempt_channels: None,
                exempt_roles: None,
                name,
                trigger_metadata: None,
                trigger_type: None,
            }),
            guild_id,
            http,
            reason: Ok(None),
        }
    }

    /// Append an action of type [`BlockMessage`].
    ///
    /// [`BlockMessage`]: AutoModerationActionType::BlockMessage
    pub fn action_block_message(mut self) -> Self {
        self.fields = self.fields.map(|mut fields| {
            fields.actions.get_or_insert_with(Vec::new).push(
                CreateAutoModerationRuleFieldsAction {
                    kind: AutoModerationActionType::BlockMessage,
                    metadata: CreateAutoModerationRuleFieldsActionMetadata::default(),
                },
            );

            fields
        });

        self
    }

    /// Append an action of type [`BlockMessage`] with an explanation for blocking messages.
    ///
    /// # Errors
    ///
    /// Returns a [`ValidationErrorType::AutoModerationBlockActionCustomMessageLimit`] if the custom message length
    /// is invalid.
    ///
    /// [`ValidationErrorType::AutoModerationBlockActionCustomMessageLimit`]: twilight_validate::request::ValidationErrorType::AutoModerationBlockActionCustomMessageLimit
    /// [`BlockMessage`]: AutoModerationActionType::BlockMessage
    pub fn action_block_message_with_explanation(mut self, custom_message: &'a str) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_auto_moderation_block_action_custom_message_limit(custom_message)?;
            fields.actions.get_or_insert_with(Vec::new).push(
                CreateAutoModerationRuleFieldsAction {
                    kind: AutoModerationActionType::BlockMessage,
                    metadata: CreateAutoModerationRuleFieldsActionMetadata {
                        custom_message: Some(String::from(custom_message)),
                        ..Default::default()
                    },
                },
            );

            Ok(fields)
        });

        self
    }

    /// Append an action of type [`SendAlertMessage`].
    ///
    /// [`SendAlertMessage`]: AutoModerationActionType::SendAlertMessage
    pub fn action_send_alert_message(mut self, channel_id: Id<ChannelMarker>) -> Self {
        self.fields = self.fields.map(|mut fields| {
            fields.actions.get_or_insert_with(Vec::new).push(
                CreateAutoModerationRuleFieldsAction {
                    kind: AutoModerationActionType::SendAlertMessage,
                    metadata: CreateAutoModerationRuleFieldsActionMetadata {
                        channel_id: Some(channel_id),
                        ..Default::default()
                    },
                },
            );

            fields
        });

        self
    }

    /// Append an action of type [`Timeout`].
    ///
    /// # Errors
    ///
    /// Returns [`ValidationErrorType::AutoModerationActionMetadataDurationSeconds`] if the duration
    /// is invalid.
    ///
    /// [`Timeout`]: AutoModerationActionType::Timeout
    /// [`ValidationErrorType::AutoModerationActionMetadataDurationSeconds`]: twilight_validate::request::ValidationErrorType::AutoModerationActionMetadataDurationSeconds
    pub fn action_timeout(mut self, duration_seconds: u32) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_auto_moderation_action_metadata_duration_seconds(duration_seconds)?;
            fields.actions.get_or_insert_with(Vec::new).push(
                CreateAutoModerationRuleFieldsAction {
                    kind: AutoModerationActionType::Timeout,
                    metadata: CreateAutoModerationRuleFieldsActionMetadata {
                        duration_seconds: Some(duration_seconds),
                        ..Default::default()
                    },
                },
            );

            Ok(fields)
        });

        self
    }

    /// Set whether the rule is enabled.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.fields = self.fields.map(|mut fields| {
            fields.enabled = Some(enabled);

            fields
        });

        self
    }

    /// Set the channels where the rule does not apply.
    /// See [Discord Docs/Trigger Metadata].
    ///
    /// # Errors
    ///
    /// Returns [`ValidationErrorType::AutoModerationExemptChannels`] if the `exempt_roles` field is invalid.
    ///
    /// [Discord Docs/Trigger Metadata]: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-metadata
    /// [`ValidationErrorType::AutoModerationExemptChannels`]: twilight_validate::request::ValidationErrorType::AutoModerationExemptChannels
    pub fn exempt_channels(mut self, exempt_channels: &'a [Id<ChannelMarker>]) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_auto_moderation_exempt_channels(exempt_channels)?;
            fields.exempt_channels = Some(exempt_channels);

            Ok(fields)
        });

        self
    }

    /// Set the roles to which the rule does not apply.
    /// See [Discord Docs/Trigger Metadata].
    ///
    /// # Errors
    ///
    /// Returns [`ValidationErrorType::AutoModerationExemptRoles`] if the `exempt_roles` field is invalid.
    ///
    /// [Discord Docs/Trigger Metadata]: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-metadata
    /// [`ValidationErrorType::AutoModerationExemptRoles`]: twilight_validate::request::ValidationErrorType::AutoModerationExemptRoles
    pub fn exempt_roles(mut self, exempt_roles: &'a [Id<RoleMarker>]) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_auto_moderation_exempt_roles(exempt_roles)?;
            fields.exempt_roles = Some(exempt_roles);

            Ok(fields)
        });

        self
    }

    /// Create the request with the trigger type [`Keyword`], then execute it.
    ///
    /// Rules of this type require the `keyword_filter`, `regex_patterns` and
    /// `allow_list` fields specified, and this method ensures this.
    /// See [Discord Docs/Keyword Matching Strategies] and
    /// [Discord Docs/Trigger Metadata] for more information.
    ///
    /// Only rust-flavored regex is currently supported by Discord.
    ///
    /// # Errors
    ///
    /// Returns [`ValidationErrorType::AutoModerationMetadataKeywordFilter`] if the `keyword_filter`
    /// field is invalid.
    ///
    /// Returns [`ValidationErrorType::AutoModerationMetadataKeywordFilterItem`] if a `keyword_filter`
    /// item is invalid.
    ///
    /// Returns [`ValidationErrorType::AutoModerationMetadataAllowList`] if the `allow_list` field is
    /// invalid.
    ///
    /// Returns [`ValidationErrorType::AutoModerationMetadataAllowListItem`] if an `allow_list` item
    /// is invalid.
    ///
    /// Returns [`ValidationErrorType::AutoModerationMetadataRegexPatterns`] if the `regex_patterns`
    /// field is invalid.
    ///
    /// Returns [`ValidationErrorType::AutoModerationMetadataRegexPatternsItem`] if a `regex_patterns`
    /// item is invalid.
    ///
    /// [`Keyword`]: AutoModerationTriggerType::Keyword
    /// [Discord Docs/Keyword Matching Strategies]: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-keyword-matching-strategies
    /// [Discord Docs/Trigger Metadata]: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-metadata
    /// [`ValidationErrorType::AutoModerationMetadataKeywordFilter`]: twilight_validate::request::ValidationErrorType::AutoModerationMetadataKeywordFilter
    /// [`ValidationErrorType::AutoModerationMetadataKeywordFilterItem`]: twilight_validate::request::ValidationErrorType::AutoModerationMetadataKeywordFilterItem
    /// [`ValidationErrorType::AutoModerationMetadataAllowList`]: twilight_validate::request::ValidationErrorType::AutoModerationMetadataAllowList
    /// [`ValidationErrorType::AutoModerationMetadataAllowListItem`]: twilight_validate::request::ValidationErrorType::AutoModerationMetadataAllowListItem
    /// [`ValidationErrorType::AutoModerationMetadataRegexPatterns`]: twilight_validate::request::ValidationErrorType::AutoModerationMetadataRegexPatterns
    /// [`ValidationErrorType::AutoModerationMetadataRegexPatternsItem`]: twilight_validate::request::ValidationErrorType::AutoModerationMetadataRegexPatternsItem
    pub fn with_keyword(
        mut self,
        keyword_filter: &'a [&'a str],
        regex_patterns: &'a [&'a str],
        allow_list: &'a [&'a str],
    ) -> ResponseFuture<AutoModerationRule> {
        self.fields = self.fields.and_then(|mut fields| {
            validate_auto_moderation_metadata_keyword_allow_list(allow_list)?;
            validate_auto_moderation_metadata_keyword_filter(keyword_filter)?;
            validate_auto_moderation_metadata_regex_patterns(regex_patterns)?;
            fields.trigger_metadata = Some(CreateAutoModerationRuleFieldsTriggerMetadata {
                allow_list: Some(allow_list),
                keyword_filter: Some(keyword_filter),
                presets: None,
                mention_total_limit: None,
                regex_patterns: Some(regex_patterns),
            });

            fields.trigger_type = Some(AutoModerationTriggerType::Keyword);

            Ok(fields)
        });

        self.exec()
    }

    /// Create the request with the trigger type [`Spam`], then execute it.
    ///
    /// [`Spam`]: AutoModerationTriggerType::Spam
    pub fn with_spam(mut self) -> ResponseFuture<AutoModerationRule> {
        self.fields = self.fields.map(|mut fields| {
            fields.trigger_type = Some(AutoModerationTriggerType::Spam);

            fields
        });

        self.exec()
    }

    /// Create the request with the trigger type [`KeywordPreset`], then execute
    /// it.
    ///
    /// Rules of this type require the `presets` and `allow_list` fields
    /// specified, and this method ensures this. See [Discord Docs/TriggerMetadata].
    ///
    /// # Errors
    ///
    /// Returns [`ValidationErrorType::AutoModerationMetadataPresetAllowList`] if the `allow_list` is
    /// invalid.
    ///
    /// Returns [`ValidationErrorType::AutoModerationMetadataPresetAllowListItem`] if a `allow_list`
    /// item is invalid.
    ///
    /// [`KeywordPreset`]: AutoModerationTriggerType::KeywordPreset
    /// [Discord Docs/Trigger Metadata]: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-metadata
    /// [`ValidationErrorType::AutoModerationMetadataPresetAllowList`]: twilight_validate::request::ValidationErrorType::AutoModerationMetadataPresetAllowList
    /// [`ValidationErrorType::AutoModerationMetadataPresetAllowListItem`]: twilight_validate::request::ValidationErrorType::AutoModerationMetadataPresetAllowListItem
    pub fn with_keyword_preset(
        mut self,
        presets: &'a [AutoModerationKeywordPresetType],
        allow_list: &'a [&'a str],
    ) -> ResponseFuture<AutoModerationRule> {
        self.fields = self.fields.and_then(|mut fields| {
            validate_auto_moderation_metadata_keyword_allow_list(allow_list)?;
            fields.trigger_metadata = Some(CreateAutoModerationRuleFieldsTriggerMetadata {
                allow_list: Some(allow_list),
                keyword_filter: None,
                presets: Some(presets),
                mention_total_limit: None,
                regex_patterns: None,
            });

            fields.trigger_type = Some(AutoModerationTriggerType::KeywordPreset);

            Ok(fields)
        });

        self.exec()
    }

    /// Create the request with the trigger type [`MentionSpam`], then execute
    /// it.
    ///
    /// Rules of this type requires the `mention_total_limit` field specified,
    /// and this method ensures this. See [Discord Docs/Trigger Metadata].
    ///
    /// # Errors
    ///
    /// Returns a [`ValidationErrorType::AutoModerationMetadataMentionTotalLimit`] if `mention_total_limit`
    /// is invalid.
    ///
    /// [`MentionSpam`]: AutoModerationTriggerType::MentionSpam
    /// [Discord Docs/Trigger Metadata]: https://discord.com/developers/docs/resources/auto-moderation#auto-moderation-rule-object-trigger-metadata
    /// [`ValidationErrorType::AutoModerationMetadataMentionTotalLimit`]: twilight_validate::request::ValidationErrorType::AutoModerationMetadataMentionTotalLimit
    pub fn with_mention_spam(
        mut self,
        mention_total_limit: u8,
    ) -> ResponseFuture<AutoModerationRule> {
        self.fields = self.fields.and_then(|mut fields| {
            validate_auto_moderation_metadata_mention_total_limit(mention_total_limit)?;
            fields.trigger_metadata = Some(CreateAutoModerationRuleFieldsTriggerMetadata {
                allow_list: None,
                keyword_filter: None,
                presets: None,
                mention_total_limit: Some(mention_total_limit),
                regex_patterns: None,
            });
            fields.trigger_type = Some(AutoModerationTriggerType::MentionSpam);

            Ok(fields)
        });

        self.exec()
    }

    /// Execute the request, returning a future resolving to a [`Response`].
    ///
    /// [`Response`]: crate::response::Response
    fn exec(self) -> ResponseFuture<AutoModerationRule> {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl<'a> AuditLogReason<'a> for CreateAutoModerationRule<'a> {
    fn reason(mut self, reason: &'a str) -> Self {
        self.reason = validate_audit_reason(reason).and(Ok(Some(reason)));

        self
    }
}

impl TryIntoRequest for CreateAutoModerationRule<'_> {
    fn try_into_request(self) -> Result<Request, HttpError> {
        let fields = self.fields.map_err(HttpError::validation)?;
        let mut request = Request::builder(&Route::CreateAutoModerationRule {
            guild_id: self.guild_id.get(),
        })
        .json(&fields);

        if let Some(reason) = self.reason.map_err(HttpError::validation)? {
            request = request.headers(request::audit_header(reason)?);
        }

        request.build()
    }
}

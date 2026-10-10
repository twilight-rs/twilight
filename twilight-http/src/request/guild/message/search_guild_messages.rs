//! HTTP request for searching for [`Message`]s within a [`Guild`].
//!
//! This endpoint supports a suite of filters for narrowing search results.
//! While all parameters—such as [`channel_id`], which narrows results to only
//! messages within a list of channels—include only results with those values,
//! some fields support *negating* a given value to exclude those results. Refer
//! to [`Negate`] for those fields. Each field's method in the
//! [`SearchGuildMessages`] request builder also notes whether the field can be
//! negated.
//!
//! # Indexing
//!
//! If the entity you are searching has not yet been indexed the endpoint will
//! return a 202 Accepted response and the response body variant will be
//! [`SearchGuildMessagesResponseBody::Accepted`]; otherwise, the response body
//! variant will be [`SearchGuildMessagesResponseBody::Ok`] with message
//! results.
//!
//! # Pagination
//!
//! Due to speed optimizations, search may return slightly fewer results than
//! the limit specified when messages have not been accessed for a long time.
//! Applications should not rely on the length of the messages array to paginate
//! results.
//!
//! Additionally, when messages are actively being created or deleted,
//! [`SearchGuildMessagesOkResponseBody::total_results`] may not be accurate.
//!
//! # Examples
//!
//! Search for messages within two channels, including only messages with a poll
//! but excluding messages authored by bot users:
//!
//! ```rust,no_run
//! use std::env;
//! use twilight_http::{
//!     Client,
//!     request::guild::message::search_guild_messages::{
//!         AuthorType, Filter, SearchGuildMessages, SearchGuildMessagesResponseBody,
//!         SearchHasTypes,
//!     },
//! };
//! use twilight_model::id::Id;
//!
//! # fn find_channels() -> Vec<Id<twilight_model::id::marker::ChannelMarker>> {
//! #     Vec::new()
//! # }
//! #
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let guild_id = Id::new(1);
//! let client = Client::new(env::var("DISCORD_TOKEN")?);
//! let channel_ids = find_channels();
//!
//! let response = client
//!     .search_guild_messages(guild_id)
//!     .channel_id(&channel_ids)
//!     // Include only messages with images
//!     .has(&[Filter::Include(SearchHasTypes::Image)])
//!     // Exclude messages authored by bots
//!     .author_type(&[Filter::Exclude(AuthorType::Bot)])
//!     .await?;
//! let body = response.model().await?;
//!
//! match body {
//!     SearchGuildMessagesResponseBody::Accepted(accepted) => {
//!         println!(
//!             "message indexing in progress; retry in {} seconds",
//!             accepted.retry_after
//!         );
//!     }
//!     SearchGuildMessagesResponseBody::Ok(results) => {
//!         println!("there are {} total results", results.total_results);
//!     }
//! }
//! # Ok(()) }
//! ```
//!
//! [`channel_id`]: SearchGuildMessages::channel_id
//! [`Guild`]: twilight_model::guild::Guild

pub use twilight_model::http::message_search::*;

use crate::{
    client::Client,
    error::Error,
    request::{Request, TryIntoRequest},
    response::{Response, ResponseFuture},
    routing::Route,
};
use serde::{Deserialize, Serialize};
use std::future::IntoFuture;
use twilight_model::{
    channel::{Channel, Message, thread::ThreadMember},
    id::{
        Id,
        marker::{ChannelMarker, GuildMarker, MessageMarker, RoleMarker, UserMarker},
    },
};
use twilight_validate::search_guild_messages::{
    SearchGuildMessagesError, attachment_extension as validate_attachment_extension,
    attachment_filename as validate_attachment_filename, author_id as validate_author_id,
    channel_id as validate_channel_id, content as validate_content,
    embed_provider as validate_embed_provider, limit as validate_limit,
    link_hostname as validate_link_hostname, mentions as validate_mentions,
    mentions_role_id as validate_mentions_role_id, offset as validate_offset,
    replied_to_message_id as validate_replied_to_message_id,
    replied_to_user_id as validate_replied_to_user_id, slop as validate_slop,
};

mod search_guild_messages_response_body_messages {
    //! serde deserializer and serializer for
    //! [`SearchGuildMessagesResponseBody::messages`].
    //!
    //! The `messages` field is an array of array of message objects for
    //! historical and now unused reasons. To simplify things for users, we
    //! remove one layer of this nested array so users simply get an array of
    //! messages.
    //!
    //! The JSON view of expected data is:
    //!
    //! ```json
    //! [
    //!   [
    //!     {
    //!       # message 1...
    //!     },
    //!     {
    //!       # message 2...
    //!     }
    //!   ]
    //! ]
    //! ```

    use serde::{
        de::{Deserialize, Deserializer, Error as _},
        ser::Serializer,
    };
    use std::iter;
    use twilight_model::channel::Message;

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<Message>, D::Error> {
        let mut nested_array = Vec::<Vec<Message>>::deserialize(deserializer)?;

        if nested_array.len() > 1 {
            return Err(D::Error::invalid_length(
                nested_array.len(),
                &"nested array with 0 or 1 element, an array of messages",
            ));
        }

        // There can be 1 element containing a list of messages, or 0 elements
        // if there are no messages, so we need to default the list.
        Ok(nested_array.pop().unwrap_or_default())
    }

    pub fn serialize<S: Serializer>(
        value: &Vec<Message>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(iter::once(value))
    }
}

/// Potential successful response bodies.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SearchGuildMessagesResponseBody {
    /// The request has been completed and indexed messages have been provided.
    ///
    /// The response is a 200 OK HTTP status code.
    Ok(SearchGuildMessagesOkResponseBody),
    /// The request has been accepted and the server is now indexing messages in
    /// the guild.
    ///
    /// The request will have to be retried later. Refer to the documentation
    /// for [`SearchGuildMessagesAcceptedResponseBody`] for more information.
    ///
    /// The response is a 202 Accepted HTTP status code.
    Accepted(SearchGuildMessagesAcceptedResponseBody),
}

/// Body of the response when the returned status code is 200 OK.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SearchGuildMessagesOkResponseBody {
    /// Whether the guild is undergoing a deep historical indexing operation.
    pub doing_deep_historical_index: bool,
    /// The number of documents that have been indexed during the current index
    /// operation, if any.
    pub documents_indexed: Option<u64>,
    /// A thread member object for each thread returned in [`threads`] the
    /// current user has joined.
    ///
    /// [`threads`]: Self::threads
    pub members: Option<Vec<ThreadMember>>,
    /// A nested array of messages that match the query.
    #[serde(with = "search_guild_messages_response_body_messages")]
    pub messages: Vec<Message>,
    /// The threads that contain the returned messages.
    pub threads: Option<Vec<Channel>>,
    /// The total number of results that match the query.
    pub total_results: u64,
}

/// Body of the response when the returned status code is 202 Accepted.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SearchGuildMessagesAcceptedResponseBody {
    /// Unique identifying code for the error response.
    pub code: u32,
    /// The number of documents that have been indexed during the current index
    /// operation, if any.
    pub documents_indexed: u64,
    /// Human-legible reason for the [`code`][Self::code].
    pub message: String,
    /// Number of seconds to wait before trying again. If the value is 0 you
    /// should retry the request after a short delay.
    pub retry_after: u64,
}

struct SearchGuildMessagesFields<'a> {
    /// Filter messages by attachment extension (e.g. `txt`).
    attachment_extension: Option<&'a [&'a str]>,
    /// Filter messages by attachment filename.
    attachment_filename: Option<&'a [&'a str]>,
    /// Filter messages by these authors.
    author_id: Option<&'a [Id<UserMarker>]>,
    /// Filter messages by author type.
    author_type: Option<&'a [Filter<AuthorType>]>,
    /// Filter messages by these channels.
    channel_id: Option<&'a [Id<ChannelMarker>]>,
    /// Filter messages by content.
    content: Option<&'a str>,
    /// Filter messages by embed provider (case-sensitive, e.g. `Tenor`).
    embed_provider: Option<&'a [&'a str]>,
    /// Filter messages by embed type.
    embed_type: Option<&'a [SearchEmbedTypes]>,
    /// Filter messages by whether or not they have specific things.
    has: Option<&'a [Filter<SearchHasTypes>]>,
    /// Whether to include results from age-restricted channels.
    ///
    /// Discord defaults this to `false`.
    include_nsfw: Option<bool>,
    /// Max number of messages to return.
    ///
    /// Discord defaults this to 25.
    limit: Option<u16>,
    /// Filter messages by link hostname (e.g. `discordapp.com`).
    link_hostname: Option<&'a [&'a str]>,
    /// Get messages before this message ID.
    max_id: Option<Id<MessageMarker>>,
    /// Filter messages that do or do not mention `@everyone`.
    mention_everyone: Option<bool>,
    /// Filter messages that mention these users.
    mentions: Option<&'a [Id<UserMarker>]>,
    /// Filter messages that mention these roles.
    mentions_role_id: Option<&'a [Id<RoleMarker>]>,
    /// Get messages after this message ID.
    min_id: Option<Id<MessageMarker>>,
    /// Number to offset the returned messages by.
    offset: Option<u16>,
    /// Filter messages by whether they are or are not pinned.
    pinned: Option<bool>,
    /// Filter messages that reply to these messages.
    replied_to_message_id: Option<&'a [Id<MessageMarker>]>,
    /// Filter messages that reply to these users.
    replied_to_user_id: Option<&'a [Id<UserMarker>]>,
    /// Max number of words to skip between matching tokens in the search
    /// content.
    slop: Option<u16>,
    /// The sorting algorithm to use.
    sort_by: Option<SearchSortModes>,
    /// The direction to sort.
    ///
    /// Discord defaults this to [`SearchSortOrder::Descending`].
    sort_order: Option<SearchSortOrder>,
}

/// Search a guild's messages.
///
/// Returned messages do not have the `reactions` field.
///
/// Refer to the module-level documentation for more information about message
/// indexing and pagination, as well as for examples.
///
/// Requires the [`READ_MESSAGE_HISTORY`] permission. This endpoint is
/// restricted according to whether the [`MESSAGE_CONTENT` Privileged Intent]
/// is enabled for your application.
///
/// [`READ_MESSAGE_HISTORY`]: twilight_model::guild::Permissions::READ_MESSAGE_HISTORY
/// [`MESSAGE_CONTENT` Privileged Intent]: twilight_model::gateway::Intents::MESSAGE_CONTENT
#[must_use = "requests must be configured and executed"]
pub struct SearchGuildMessages<'a> {
    fields: Result<SearchGuildMessagesFields<'a>, SearchGuildMessagesError>,
    guild_id: Id<GuildMarker>,
    http: &'a Client,
}

impl<'a> SearchGuildMessages<'a> {
    pub(crate) const fn new(http: &'a Client, guild_id: Id<GuildMarker>) -> Self {
        Self {
            fields: Ok(SearchGuildMessagesFields {
                attachment_extension: None,
                attachment_filename: None,
                author_id: None,
                author_type: None,
                channel_id: None,
                content: None,
                embed_provider: None,
                embed_type: None,
                has: None,
                include_nsfw: None,
                limit: None,
                link_hostname: None,
                max_id: None,
                mention_everyone: None,
                mentions: None,
                mentions_role_id: None,
                min_id: None,
                offset: None,
                pinned: None,
                replied_to_message_id: None,
                replied_to_user_id: None,
                slop: None,
                sort_by: None,
                sort_order: None,
            }),
            guild_id,
            http,
        }
    }

    /// Filter messages by attachment extension (e.g. `txt`).
    ///
    /// # Errors
    ///
    /// Returns an error of type [`AttachmentExtensionCount`] if the number of
    /// attachment extension entries is invalid.
    ///
    /// Returns an error of type [`AttachmentExtensionEntryLength`] if one of the
    /// attachment extension entries has an invalid length.
    ///
    /// [`AttachmentExtensionCount`]: twilight_validate::search_guild_messages::SearchGuildMessagesErrorType::AttachmentExtensionCount
    /// [`AttachmentExtensionEntryLength`]: twilight_validate::search_guild_messages::SearchGuildMessagesErrorType::AttachmentExtensionEntryLength
    pub fn attachment_extension(mut self, attachment_extension: &'a [&'a str]) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_attachment_extension(attachment_extension)?;
            fields.attachment_extension = Some(attachment_extension);

            Ok(fields)
        });

        self
    }

    /// Filter messages by attachment filename.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`AttachmentFilenameCount`] if the number of
    /// attachment filename entries is invalid.
    ///
    /// Returns an error of type [`AttachmentFilenameEntryLength`] if one of the
    /// attachment filename entries has an invalid length.
    ///
    /// [`AttachmentFilenameCount`]: twilight_validate::search_guild_messages::SearchGuildMessagesErrorType::AttachmentFilenameCount
    /// [`AttachmentFilenameEntryLength`]: twilight_validate::search_guild_messages::SearchGuildMessagesErrorType::AttachmentFilenameEntryLength
    pub fn attachment_filename(mut self, attachment_filename: &'a [&'a str]) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_attachment_filename(attachment_filename)?;
            fields.attachment_filename = Some(attachment_filename);

            Ok(fields)
        });

        self
    }

    /// Filter messages by these authors.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`AuthorIdCount`] if the number of author IDs is
    /// invalid.
    ///
    /// [`AuthorIdCount`]: twilight_validate::search_guild_messages::SearchGuildMessagesErrorType::AuthorIdCount
    pub fn author_id(mut self, author_id: &'a [Id<UserMarker>]) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_author_id(author_id)?;
            fields.author_id = Some(author_id);

            Ok(fields)
        });

        self
    }

    /// Filter messages by author type.
    ///
    /// This field can be negated, causing results to exclude a particular type.
    /// Use [`Filter::Exclude`] to exclude results.
    pub fn author_type(mut self, author_type: &'a [Filter<AuthorType>]) -> Self {
        self.fields = self.fields.map(|mut fields| {
            fields.author_type = Some(author_type);

            fields
        });

        self
    }

    /// Filter messages by these channels.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`ChannelIdCount`] if the number of channel IDs is
    /// invalid.
    ///
    /// [`ChannelIdCount`]: twilight_validate::search_guild_messages::SearchGuildMessagesErrorType::ChannelIdCount
    pub fn channel_id(mut self, channel_id: &'a [Id<ChannelMarker>]) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_channel_id(channel_id)?;
            fields.channel_id = Some(channel_id);

            Ok(fields)
        });

        self
    }

    /// Filter messages by content.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`ContentLength`] is the content is too long.
    ///
    /// [`ContentLength`]: twilight_validate::search_guild_messages::SearchGuildMessagesErrorType::ContentLength
    pub fn content(mut self, content: &'a str) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_content(content)?;
            fields.content = Some(content);

            Ok(fields)
        });

        self
    }

    /// Filter messages by embed provider (case-sensitive, e.g. `Tenor`).
    ///
    /// # Errors
    ///
    /// Returns an error of type [`EmbedProviderCount`] if the number of
    /// embed provider entries is invalid.
    ///
    /// Returns an error of type [`EmbedProviderEntryLength`] if one of the
    /// embed provider entries has an invalid length.
    ///
    /// [`EmbedProviderCount`]: twilight_validate::search_guild_messages::SearchGuildMessagesErrorType::EmbedProviderCount
    /// [`EmbedProviderEntryLength`]: twilight_validate::search_guild_messages::SearchGuildMessagesErrorType::EmbedProviderEntryLength
    pub fn embed_provider(mut self, embed_provider: &'a [&'a str]) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_embed_provider(embed_provider)?;
            fields.embed_provider = Some(embed_provider);

            Ok(fields)
        });

        self
    }

    /// Filter messages by embed type.
    pub fn embed_type(mut self, embed_type: &'a [SearchEmbedTypes]) -> Self {
        self.fields = self.fields.map(|mut fields| {
            fields.embed_type = Some(embed_type);

            fields
        });

        self
    }

    /// Filter messages by whether or not they have specific things.
    ///
    /// This field can be negated, causing results to exclude a particular type.
    /// Use [`Filter::Exclude`] to exclude results.
    pub fn has(mut self, has: &'a [Filter<SearchHasTypes>]) -> Self {
        self.fields = self.fields.map(|mut fields| {
            fields.has = Some(has);

            fields
        });

        self
    }

    /// Whether to include results from age-restricted channels.
    ///
    /// Discord defaults this to `false`.
    pub fn include_nsfw(mut self, include_nsfw: bool) -> Self {
        self.fields = self.fields.map(|mut fields| {
            fields.include_nsfw = Some(include_nsfw);

            fields
        });

        self
    }

    /// Max number of messages to return.
    ///
    /// Discord defaults this to 25.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`Limit`] if the limit is invalid.
    ///
    /// [`Limit`]: twilight_validate::search_guild_messages::SearchGuildMessagesErrorType::Limit
    pub fn limit(mut self, limit: u16) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_limit(usize::from(limit))?;
            fields.limit = Some(limit);

            Ok(fields)
        });

        self
    }

    /// Filter messages by link hostname (e.g. `discordapp.com`).
    ///
    /// # Errors
    ///
    /// Returns an error of type [`LinkHostnameCount`] if the number of
    /// link hostname entries is invalid.
    ///
    /// Returns an error of type [`LinkHostnameEntryLength`] if one of the
    /// link hostname entries has an invalid length.
    ///
    /// [`LinkHostnameCount`]: twilight_validate::search_guild_messages::SearchGuildMessagesErrorType::LinkHostnameCount
    /// [`LinkHostnameEntryLength`]: twilight_validate::search_guild_messages::SearchGuildMessagesErrorType::LinkHostnameEntryLength
    pub fn link_hostname(mut self, link_hostname: &'a [&'a str]) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_link_hostname(link_hostname)?;
            fields.link_hostname = Some(link_hostname);

            Ok(fields)
        });

        self
    }

    /// Get messages before this message ID.
    pub fn max_id(mut self, max_id: Id<MessageMarker>) -> Self {
        self.fields = self.fields.map(|mut fields| {
            fields.max_id = Some(max_id);

            fields
        });

        self
    }

    /// Filter messages that do or do not mention `@everyone`.
    pub fn mention_everyone(mut self, mention_everyone: bool) -> Self {
        self.fields = self.fields.map(|mut fields| {
            fields.mention_everyone = Some(mention_everyone);

            fields
        });

        self
    }

    /// Filter messages that mention these users.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`MentionsCount`] if the number of mentions is
    /// invalid.
    ///
    /// [`MentionsCount`]: twilight_validate::search_guild_messages::SearchGuildMessagesErrorType::MentionsCount
    pub fn mentions(mut self, mentions: &'a [Id<UserMarker>]) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_mentions(mentions)?;
            fields.mentions = Some(mentions);

            Ok(fields)
        });

        self
    }

    /// Filter messages that mention these roles.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`MentionsRoleIdCount`] if the number of mention
    /// roles is invalid.
    ///
    /// [`MentionsRoleIdCount`]: twilight_validate::search_guild_messages::SearchGuildMessagesErrorType::MentionsRoleIdCount
    pub fn mentions_role_id(mut self, mentions_role_id: &'a [Id<RoleMarker>]) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_mentions_role_id(mentions_role_id)?;
            fields.mentions_role_id = Some(mentions_role_id);

            Ok(fields)
        });

        self
    }

    /// Get messages after this message ID.
    pub fn min_id(mut self, min_id: Id<MessageMarker>) -> Self {
        self.fields = self.fields.map(|mut fields| {
            fields.min_id = Some(min_id);

            fields
        });

        self
    }

    /// Number to offset the returned messages by.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`Offset`] if the offset is invalid.
    ///
    /// [`Offset`]: twilight_validate::search_guild_messages::SearchGuildMessagesErrorType::Offset
    pub fn offset(mut self, offset: u16) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_offset(usize::from(offset))?;
            fields.offset = Some(offset);

            Ok(fields)
        });

        self
    }

    /// Filter messages by whether they are or are not pinned.
    pub fn pinned(mut self, pinned: bool) -> Self {
        self.fields = self.fields.map(|mut fields| {
            fields.pinned = Some(pinned);

            fields
        });

        self
    }

    /// Filter messages that reply to these messages.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`RepliedToMessageIdCount`] if the number of
    /// replied to message IDs is invalid.
    ///
    /// [`RepliedToMessageIdCount`]: twilight_validate::search_guild_messages::SearchGuildMessagesErrorType::RepliedToMessageIdCount
    pub fn replied_to_message_id(mut self, replied_to_message_id: &'a [Id<MessageMarker>]) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_replied_to_message_id(replied_to_message_id)?;
            fields.replied_to_message_id = Some(replied_to_message_id);

            Ok(fields)
        });

        self
    }

    /// Filter messages that reply to these users.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`RepliedToUserIdCount`] if the number of replied
    /// to user IDs is invalid.
    ///
    /// [`RepliedToUserIdCount`]: twilight_validate::search_guild_messages::SearchGuildMessagesErrorType::RepliedToUserIdCount
    pub fn replied_to_user_id(mut self, replied_to_user_id: &'a [Id<UserMarker>]) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_replied_to_user_id(replied_to_user_id)?;
            fields.replied_to_user_id = Some(replied_to_user_id);

            Ok(fields)
        });

        self
    }

    /// Max number of words to skip between matching tokens in the search
    /// content.
    ///
    /// Discord defaults this to 2.
    ///
    /// # Errors
    ///
    /// Returns an error of type [`Slop`] if the slop is invalid.
    ///
    /// [`Slop`]: twilight_validate::search_guild_messages::SearchGuildMessagesErrorType::Slop
    pub fn slop(mut self, slop: u16) -> Self {
        self.fields = self.fields.and_then(|mut fields| {
            validate_slop(usize::from(slop))?;
            fields.slop = Some(slop);

            Ok(fields)
        });

        self
    }

    /// The sorting algorithm to use.
    ///
    /// When the sort mode is set to [`SearchSortModes::Relevance`] Discord will
    /// not respect the sort order set via [`sort_order`][Self::sort_order].
    ///
    /// Discord defaults this to [`SearchSortModes::Timestamp`].
    pub fn sort_by(mut self, sort_by: SearchSortModes) -> Self {
        self.fields = self.fields.map(|mut fields| {
            fields.sort_by = Some(sort_by);

            fields
        });

        self
    }

    /// The direction to sort.
    ///
    /// Discord will not respect the sort order when the sort mode is
    /// [`SearchSortModes::Relevance`] via [`sort_by`][Self::sort_by].
    ///
    /// Discord defaults this to [`SearchSortOrder::Descending`].
    pub fn sort_order(mut self, sort_order: SearchSortOrder) -> Self {
        self.fields = self.fields.map(|mut fields| {
            fields.sort_order = Some(sort_order);

            fields
        });

        self
    }
}

impl IntoFuture for SearchGuildMessages<'_> {
    type Output = Result<Response<SearchGuildMessagesResponseBody>, Error>;

    type IntoFuture = ResponseFuture<SearchGuildMessagesResponseBody>;

    fn into_future(self) -> Self::IntoFuture {
        let http = self.http;

        match self.try_into_request() {
            Ok(request) => http.request(request),
            Err(source) => ResponseFuture::error(source),
        }
    }
}

impl TryIntoRequest for SearchGuildMessages<'_> {
    fn try_into_request(self) -> Result<Request, Error> {
        let fields = self.fields.map_err(Error::validation)?;

        Ok(Request::from_route(&Route::SearchGuildMessages {
            attachment_extension: fields.attachment_extension,
            attachment_filename: fields.attachment_filename,
            author_id: fields.author_id,
            author_type: fields.author_type,
            channel_id: fields.channel_id,
            content: fields.content,
            embed_provider: fields.embed_provider,
            embed_type: fields.embed_type,
            guild_id: self.guild_id.get(),
            has: fields.has,
            include_nsfw: fields.include_nsfw,
            limit: fields.limit,
            link_hostname: fields.link_hostname,
            max_id: fields.max_id.map(Id::get),
            mention_everyone: fields.mention_everyone,
            mentions: fields.mentions,
            mentions_role_id: fields.mentions_role_id,
            min_id: fields.min_id.map(Id::get),
            offset: fields.offset,
            pinned: fields.pinned,
            replied_to_message_id: fields.replied_to_message_id,
            replied_to_user_id: fields.replied_to_user_id,
            slop: fields.slop,
            sort_by: fields.sort_by,
            sort_order: fields.sort_order,
        }))
    }
}

//! Constants, error types, and functions for validating the query parameters of
//! the Search Guild Messages HTTP request.
//!
//! All constants are based on the values provided in the documentation for the
//! request located at [Discord Docs/Search Guild Messages][docs].
//!
//! [docs]: https://docs.discord.com/developers/resources/message#search-guild-messages-query-string-params

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use twilight_model::id::{
    Id,
    marker::{ChannelMarker, MessageMarker, RoleMarker, UserMarker},
};

/// Maximum length of an individual attachment extension entry.
pub const ATTACHMENT_EXTENSION_ENTRY_MAX: usize = 256;

/// Maximum number of attachment extensions to filter by.
pub const ATTACHMENT_EXTENSION_LIMIT: usize = 100;

/// Maximum length of an individual attachment filename entry.
pub const ATTACHMENT_FILENAME_ENTRY_MAX: usize = 1024;

/// Maximum number of attachment filenames to filter by.
pub const ATTACHMENT_FILENAME_LIMIT: usize = 100;

/// Maximum number of author IDs to filter by.
pub const AUTHOR_ID_LIMIT: usize = 100;

/// Maximum length of content.
pub const CONTENT_LENGTH_MAX: usize = 1024;

/// Maximum number of channel IDs to filter by.
pub const CHANNEL_ID_LIMIT: usize = 500;

/// Maximum length of an individual embed provider entry.
pub const EMBED_PROVIDER_ENTRY_MAX: usize = 256;

/// Maximum number of embed providers to filter by.
pub const EMBED_PROVIDER_LIMIT: usize = 100;

/// Maximum amount of messages to get.
pub const LIMIT_MAX: usize = 25;

/// Minimum amount of messages to get.
pub const LIMIT_MIN: usize = 1;

/// Maximum length of an individual link hostname entry.
pub const LINK_HOSTNAME_ENTRY_MAX: usize = 256;

/// Maximum number of link hostnames to filter by.
pub const LINK_HOSTNAME_LIMIT: usize = 100;

/// Maximum number of user mentions to filter by.
pub const MENTIONS_LIMIT: usize = 100;

/// Maximum number of role mentions to filter by.
pub const MENTION_ROLE_ID_LIMIT: usize = 100;

/// Maximum offset of messages.
pub const OFFSET_MAX: usize = 9975;

/// Maximum number of replied messages to filter by.
pub const REPLIED_TO_MESSAGE_ID_LIMIT: usize = 100;

/// Maximum number of replied users to filter by.
pub const REPLIED_TO_USER_ID_LIMIT: usize = 100;

/// Maximum amount of slop.
pub const SLOP_MAX: usize = 100;

/// A message is not valid.
#[derive(Debug)]
pub struct SearchGuildMessagesError {
    /// Type of error that occurred.
    kind: SearchGuildMessagesErrorType,
}

impl SearchGuildMessagesError {
    /// Immutable reference to the type of error that occurred.
    #[must_use = "retrieving the type has no effect if left unused"]
    pub const fn kind(&self) -> &SearchGuildMessagesErrorType {
        &self.kind
    }

    /// Consume the error, returning the source error if there is any.
    #[must_use = "consuming the error and retrieving the source has no effect if left unused"]
    pub fn into_source(self) -> Option<Box<dyn Error + Send + Sync>> {
        None
    }

    /// Consume the error, returning the owned error type and the source error.
    #[must_use = "consuming the error into its parts has no effect if left unused"]
    pub fn into_parts(
        self,
    ) -> (
        SearchGuildMessagesErrorType,
        Option<Box<dyn Error + Send + Sync>>,
    ) {
        (self.kind, None)
    }
}

#[expect(clippy::too_many_lines)]
impl Display for SearchGuildMessagesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self.kind {
            SearchGuildMessagesErrorType::AttachmentExtensionCount { count } => {
                f.write_str("amount of attachment extensions provided is ")?;
                Display::fmt(count, f)?;
                f.write_str(" but must be at most ")?;

                Display::fmt(&ATTACHMENT_EXTENSION_LIMIT, f)
            }
            SearchGuildMessagesErrorType::AttachmentExtensionEntryLength { index, value } => {
                f.write_str("attachment extension entry at index ")?;
                Display::fmt(index, f)?;
                f.write_str(" is ")?;
                Display::fmt(value, f)?;
                f.write_str(" characters long but must be at most ")?;
                Display::fmt(&ATTACHMENT_EXTENSION_ENTRY_MAX, f)?;

                f.write_str(" characters long")
            }
            SearchGuildMessagesErrorType::AttachmentFilenameCount { count } => {
                f.write_str("amount of attachment filenames provided is ")?;
                Display::fmt(count, f)?;
                f.write_str(" but must be at most ")?;

                Display::fmt(&ATTACHMENT_FILENAME_LIMIT, f)
            }
            SearchGuildMessagesErrorType::AttachmentFilenameEntryLength { index, value } => {
                f.write_str("attachment filename entry at index ")?;
                Display::fmt(index, f)?;
                f.write_str(" is ")?;
                Display::fmt(value, f)?;
                f.write_str(" characters long but must be at most ")?;
                Display::fmt(&ATTACHMENT_FILENAME_ENTRY_MAX, f)?;

                f.write_str(" characters long")
            }
            SearchGuildMessagesErrorType::AuthorIdCount { count } => {
                f.write_str("amount of author IDs provided is ")?;
                Display::fmt(count, f)?;
                f.write_str(" but must be at most ")?;

                Display::fmt(&AUTHOR_ID_LIMIT, f)
            }
            SearchGuildMessagesErrorType::ChannelIdCount { count } => {
                f.write_str("amount of channel IDs provided is ")?;
                Display::fmt(count, f)?;
                f.write_str(" but must be at most ")?;

                Display::fmt(&CHANNEL_ID_LIMIT, f)
            }
            SearchGuildMessagesErrorType::ContentLength { value } => {
                f.write_str("content is ")?;
                Display::fmt(value, f)?;
                f.write_str(" characters long but must be at most ")?;
                Display::fmt(&CONTENT_LENGTH_MAX, f)?;

                f.write_str(" characters long")
            }
            SearchGuildMessagesErrorType::EmbedProviderCount { count } => {
                f.write_str("amount of embed providers provided is ")?;
                Display::fmt(count, f)?;
                f.write_str(" but must be at most ")?;

                Display::fmt(&EMBED_PROVIDER_LIMIT, f)
            }
            SearchGuildMessagesErrorType::EmbedProviderEntryLength { index, value } => {
                f.write_str("embed provider entry at index ")?;
                Display::fmt(index, f)?;
                f.write_str(" is ")?;
                Display::fmt(value, f)?;
                f.write_str(" characters long but must be at most ")?;
                Display::fmt(&EMBED_PROVIDER_ENTRY_MAX, f)?;

                f.write_str(" characters long")
            }
            SearchGuildMessagesErrorType::Limit { value } => {
                f.write_str("provided limit is ")?;
                Display::fmt(value, f)?;
                f.write_str(" but it must be at least ")?;
                Display::fmt(&LIMIT_MIN, f)?;
                f.write_str(" and at most ")?;

                Display::fmt(&LIMIT_MAX, f)
            }
            SearchGuildMessagesErrorType::LinkHostnameCount { count } => {
                f.write_str("amount of link hostnames provided is ")?;
                Display::fmt(count, f)?;
                f.write_str(" but must be at most ")?;

                Display::fmt(&LINK_HOSTNAME_LIMIT, f)
            }
            SearchGuildMessagesErrorType::LinkHostnameEntryLength { index, value } => {
                f.write_str("link hostname entry at index ")?;
                Display::fmt(index, f)?;
                f.write_str(" is ")?;
                Display::fmt(value, f)?;
                f.write_str(" characters long but must be at most ")?;
                Display::fmt(&LINK_HOSTNAME_ENTRY_MAX, f)?;

                f.write_str(" characters long")
            }
            SearchGuildMessagesErrorType::MentionsCount { count } => {
                f.write_str("amount of mentions provided is ")?;
                Display::fmt(count, f)?;
                f.write_str(" but must be at most ")?;

                Display::fmt(&MENTIONS_LIMIT, f)
            }
            SearchGuildMessagesErrorType::MentionsRoleIdCount { count } => {
                f.write_str("amount of mention role IDs provided is ")?;
                Display::fmt(count, f)?;
                f.write_str(" but must be at most ")?;

                Display::fmt(&MENTION_ROLE_ID_LIMIT, f)
            }
            SearchGuildMessagesErrorType::Offset { value } => {
                f.write_str("provided offset is ")?;
                Display::fmt(value, f)?;
                f.write_str(" but must be at most ")?;

                Display::fmt(&OFFSET_MAX, f)
            }
            SearchGuildMessagesErrorType::RepliedToMessageIdCount { count } => {
                f.write_str("amount of replied to message IDs provided is ")?;
                Display::fmt(count, f)?;
                f.write_str(" but must be at most ")?;

                Display::fmt(&REPLIED_TO_MESSAGE_ID_LIMIT, f)
            }
            SearchGuildMessagesErrorType::RepliedToUserIdCount { count } => {
                f.write_str("amount of replied to user IDs provided is ")?;
                Display::fmt(count, f)?;
                f.write_str(" but must be at most ")?;

                Display::fmt(&REPLIED_TO_USER_ID_LIMIT, f)
            }
            SearchGuildMessagesErrorType::Slop { value } => {
                f.write_str("provided slop is ")?;
                Display::fmt(value, f)?;
                f.write_str(" but it must be at most ")?;

                Display::fmt(&SLOP_MAX, f)
            }
        }
    }
}

impl Error for SearchGuildMessagesError {}

/// Type of [`SearchGuildMessagesError`] that occurred.
#[derive(Debug)]
#[non_exhaustive]
pub enum SearchGuildMessagesErrorType {
    /// Number of attachment extensions is invalid.
    AttachmentExtensionCount {
        /// Invalid count.
        count: usize,
    },
    /// Length of an attachment extension entry is invalid.
    AttachmentExtensionEntryLength {
        /// Index of the first invalid entry.
        index: usize,
        /// Invalid length.
        value: usize,
    },
    /// Number of attachment filenames is invalid.
    AttachmentFilenameCount {
        /// Invalid count.
        count: usize,
    },
    /// Length of an attachment filename entry is invalid.
    AttachmentFilenameEntryLength {
        /// Index of the first invalid entry.
        index: usize,
        /// Invalid length.
        value: usize,
    },
    /// Number of author IDs is invalid.
    AuthorIdCount {
        /// Invalid count.
        count: usize,
    },
    /// Number of channel IDs is invalid.
    ChannelIdCount {
        /// Invalid count.
        count: usize,
    },
    /// Length of content is invalid.
    ContentLength {
        /// Invalid length.
        value: usize,
    },
    /// Number of embed providers is invalid.
    EmbedProviderCount {
        /// Invalid count.
        count: usize,
    },
    /// Length of an embed provider entry is invalid.
    EmbedProviderEntryLength {
        /// Index of the first invalid entry.
        index: usize,
        /// Invalid length.
        value: usize,
    },
    /// Number of messages to return is too small or too large.
    Limit {
        /// Invalid value.
        value: usize,
    },
    /// Length of a link hostname entry is invalid.
    LinkHostnameEntryLength {
        /// Index of the first invalid entry.
        index: usize,
        /// Invalid length.
        value: usize,
    },
    /// Number of link hostnames is invalid.
    LinkHostnameCount {
        /// Invalid count.
        count: usize,
    },
    /// Number of mentions is invalid.
    MentionsCount {
        /// Invalid count.
        count: usize,
    },
    /// Number of mention roles is invalid.
    MentionsRoleIdCount {
        /// Invalid count.
        count: usize,
    },
    /// Number of messages to offset by is invalid.
    Offset {
        /// Invalid value.
        value: usize,
    },
    /// Number of replied messages is invalid.
    RepliedToMessageIdCount {
        /// Invalid count.
        count: usize,
    },
    /// Number of replied users is invalid.
    RepliedToUserIdCount {
        /// Invalid count.
        count: usize,
    },
    /// Amount of slop is invalid.
    Slop {
        /// Invalid value.
        value: usize,
    },
}

/// Ensure a list of attachment extensions is correct.
///
/// # Errors
///
/// Returns an error of type [`AttachmentExtensionCount`] if the number of
/// attachment extension entries is invalid.
///
/// Returns an error of type [`AttachmentExtensionEntryLength`] if one of the
/// attachment extension entries has an invalid length.
///
/// [`AttachmentExtensionCount`]: SearchGuildMessagesErrorType::AttachmentExtensionCount
/// [`AttachmentExtensionEntryLength`]: SearchGuildMessagesErrorType::AttachmentExtensionEntryLength
pub fn attachment_extension(attachment_extension: &[&str]) -> Result<(), SearchGuildMessagesError> {
    if attachment_extension.len() > ATTACHMENT_EXTENSION_LIMIT {
        return Err(SearchGuildMessagesError {
            kind: SearchGuildMessagesErrorType::AttachmentExtensionCount {
                count: attachment_extension.len(),
            },
        });
    }

    for (index, entry) in attachment_extension.iter().enumerate() {
        let length = entry.chars().count();

        if length > ATTACHMENT_EXTENSION_ENTRY_MAX {
            return Err(SearchGuildMessagesError {
                kind: SearchGuildMessagesErrorType::AttachmentExtensionEntryLength {
                    index,
                    value: length,
                },
            });
        }
    }

    Ok(())
}

/// Ensure a list of attachment filenames is correct.
///
/// # Errors
///
/// Returns an error of type [`AttachmentFilenameCount`] if the number of
/// attachment filename entries is invalid.
///
/// Returns an error of type [`AttachmentFilenameEntryLength`] if one of the
/// attachment filename entries has an invalid length.
///
/// [`AttachmentFilenameCount`]: SearchGuildMessagesErrorType::AttachmentFilenameCount
/// [`AttachmentFilenameEntryLength`]: SearchGuildMessagesErrorType::AttachmentFilenameEntryLength
pub fn attachment_filename(attachment_filename: &[&str]) -> Result<(), SearchGuildMessagesError> {
    if attachment_filename.len() > ATTACHMENT_FILENAME_LIMIT {
        return Err(SearchGuildMessagesError {
            kind: SearchGuildMessagesErrorType::AttachmentFilenameCount {
                count: attachment_filename.len(),
            },
        });
    }

    for (index, entry) in attachment_filename.iter().enumerate() {
        let length = entry.chars().count();

        if length > ATTACHMENT_FILENAME_ENTRY_MAX {
            return Err(SearchGuildMessagesError {
                kind: SearchGuildMessagesErrorType::AttachmentFilenameEntryLength {
                    index,
                    value: length,
                },
            });
        }
    }

    Ok(())
}

/// Ensure that the amount of author IDs is correct.
///
/// # Errors
///
/// Returns an error of type [`AuthorIdCount`] if the number of author IDs is
/// invalid.
///
/// [`AuthorIdCount`]: SearchGuildMessagesErrorType::AuthorIdCount
pub const fn author_id(author_id: &[Id<UserMarker>]) -> Result<(), SearchGuildMessagesError> {
    if author_id.len() > AUTHOR_ID_LIMIT {
        return Err(SearchGuildMessagesError {
            kind: SearchGuildMessagesErrorType::AuthorIdCount {
                count: author_id.len(),
            },
        });
    }

    Ok(())
}

/// Ensure that the amount of channel IDs is correct.
///
/// # Errors
///
/// Returns an error of type [`ChannelIdCount`] if the number of channel IDs is
/// invalid.
///
/// [`ChannelIdCount`]: SearchGuildMessagesErrorType::ChannelIdCount
pub const fn channel_id(channel_id: &[Id<ChannelMarker>]) -> Result<(), SearchGuildMessagesError> {
    if channel_id.len() > CHANNEL_ID_LIMIT {
        return Err(SearchGuildMessagesError {
            kind: SearchGuildMessagesErrorType::ChannelIdCount {
                count: channel_id.len(),
            },
        });
    }

    Ok(())
}

/// Ensure the content is correct.
///
/// # Errors
///
/// Returns an error of type [`ContentLength`] is the content is too long.
///
/// [`ContentLength`]: SearchGuildMessagesErrorType::ContentLength
pub fn content(content: &str) -> Result<(), SearchGuildMessagesError> {
    let length = content.chars().count();

    if length > CONTENT_LENGTH_MAX {
        return Err(SearchGuildMessagesError {
            kind: SearchGuildMessagesErrorType::ContentLength { value: length },
        });
    }

    Ok(())
}

/// Ensure a list of embed providers is correct.
///
/// # Errors
///
/// Returns an error of type [`EmbedProviderCount`] if the number of
/// embed provider entries is invalid.
///
/// Returns an error of type [`EmbedProviderEntryLength`] if one of the
/// embed provider entries has an invalid length.
///
/// [`EmbedProviderCount`]: SearchGuildMessagesErrorType::EmbedProviderCount
/// [`EmbedProviderEntryLength`]: SearchGuildMessagesErrorType::EmbedProviderEntryLength
pub fn embed_provider(embed_provider: &[&str]) -> Result<(), SearchGuildMessagesError> {
    if embed_provider.len() > EMBED_PROVIDER_LIMIT {
        return Err(SearchGuildMessagesError {
            kind: SearchGuildMessagesErrorType::EmbedProviderCount {
                count: embed_provider.len(),
            },
        });
    }

    for (index, entry) in embed_provider.iter().enumerate() {
        let length = entry.chars().count();

        if length > EMBED_PROVIDER_ENTRY_MAX {
            return Err(SearchGuildMessagesError {
                kind: SearchGuildMessagesErrorType::EmbedProviderEntryLength {
                    index,
                    value: length,
                },
            });
        }
    }

    Ok(())
}

/// Ensure that the limit for the is correct.
///
/// The limit must be at least [`LIMIT_MIN`] and at most [`LIMIT_MAX`].
///
/// # Errors
///
/// Returns an error of type [`Limit`] if the limit is invalid.
///
/// [`Limit`]: SearchGuildMessagesErrorType::Limit
pub const fn limit(value: usize) -> Result<(), SearchGuildMessagesError> {
    if value < LIMIT_MIN || value > LIMIT_MAX {
        return Err(SearchGuildMessagesError {
            kind: SearchGuildMessagesErrorType::Limit { value },
        });
    }

    Ok(())
}

/// Ensure a list of link hostnames is correct.
///
/// # Errors
///
/// Returns an error of type [`LinkHostnameCount`] if the number of
/// link hostname entries is invalid.
///
/// Returns an error of type [`LinkHostnameEntryLength`] if one of the
/// link hostname entries has an invalid length.
///
/// [`LinkHostnameCount`]: SearchGuildMessagesErrorType::LinkHostnameCount
/// [`LinkHostnameEntryLength`]: SearchGuildMessagesErrorType::LinkHostnameEntryLength
pub fn link_hostname(link_hostname: &[&str]) -> Result<(), SearchGuildMessagesError> {
    if link_hostname.len() > LINK_HOSTNAME_LIMIT {
        return Err(SearchGuildMessagesError {
            kind: SearchGuildMessagesErrorType::LinkHostnameCount {
                count: link_hostname.len(),
            },
        });
    }

    for (index, entry) in link_hostname.iter().enumerate() {
        let length = entry.chars().count();

        if length > LINK_HOSTNAME_ENTRY_MAX {
            return Err(SearchGuildMessagesError {
                kind: SearchGuildMessagesErrorType::LinkHostnameEntryLength {
                    index,
                    value: length,
                },
            });
        }
    }

    Ok(())
}

/// Ensure that the amount of mentions is correct.
///
/// # Errors
///
/// Returns an error of type [`MentionsCount`] if the number of mentions is
/// invalid.
///
/// [`MentionsCount`]: SearchGuildMessagesErrorType::MentionsCount
pub const fn mentions(mentions: &[Id<UserMarker>]) -> Result<(), SearchGuildMessagesError> {
    if mentions.len() > MENTIONS_LIMIT {
        return Err(SearchGuildMessagesError {
            kind: SearchGuildMessagesErrorType::MentionsCount {
                count: mentions.len(),
            },
        });
    }

    Ok(())
}

/// Ensure that the amount of mention roles is correct.
///
/// # Errors
///
/// Returns an error of type [`MentionsRoleIdCount`] if the number of mention
/// roles is invalid.
///
/// [`MentionsRoleIdCount`]: SearchGuildMessagesErrorType::MentionsRoleIdCount
pub const fn mentions_role_id(
    mentions_role_id: &[Id<RoleMarker>],
) -> Result<(), SearchGuildMessagesError> {
    if mentions_role_id.len() > MENTION_ROLE_ID_LIMIT {
        return Err(SearchGuildMessagesError {
            kind: SearchGuildMessagesErrorType::MentionsRoleIdCount {
                count: mentions_role_id.len(),
            },
        });
    }

    Ok(())
}

/// Ensure that the offset for the is correct.
///
/// The offset must be at most [`OFFSET_MAX`].
///
/// # Errors
///
/// Returns an error of type [`Offset`] if the offset is invalid.
///
/// [`Offset`]: SearchGuildMessagesErrorType::Offset
pub const fn offset(value: usize) -> Result<(), SearchGuildMessagesError> {
    if value > OFFSET_MAX {
        return Err(SearchGuildMessagesError {
            kind: SearchGuildMessagesErrorType::Offset { value },
        });
    }

    Ok(())
}

/// Ensure that the amount of replied to message IDs is correct.
///
/// # Errors
///
/// Returns an error of type [`RepliedToMessageIdCount`] if the number of
/// replied to message IDs is invalid.
///
/// [`RepliedToMessageIdCount`]: SearchGuildMessagesErrorType::RepliedToMessageIdCount
pub const fn replied_to_message_id(
    replied_to_message_id: &[Id<MessageMarker>],
) -> Result<(), SearchGuildMessagesError> {
    if replied_to_message_id.len() > REPLIED_TO_MESSAGE_ID_LIMIT {
        return Err(SearchGuildMessagesError {
            kind: SearchGuildMessagesErrorType::RepliedToMessageIdCount {
                count: replied_to_message_id.len(),
            },
        });
    }

    Ok(())
}

/// Ensure that the amount of replied to user IDs is correct.
///
/// # Errors
///
/// Returns an error of type [`RepliedToUserIdCount`] if the number of replied
/// to user IDs is invalid.
///
/// [`RepliedToUserIdCount`]: SearchGuildMessagesErrorType::RepliedToUserIdCount
pub const fn replied_to_user_id(
    replied_to_user_id: &[Id<UserMarker>],
) -> Result<(), SearchGuildMessagesError> {
    if replied_to_user_id.len() > REPLIED_TO_USER_ID_LIMIT {
        return Err(SearchGuildMessagesError {
            kind: SearchGuildMessagesErrorType::RepliedToUserIdCount {
                count: replied_to_user_id.len(),
            },
        });
    }

    Ok(())
}

/// Ensure that the slop is correct.
///
/// The limit must be at most [`SLOP_MAX`].
///
/// # Errors
///
/// Returns an error of type [`Slop`] if the slop is invalid.
///
/// [`Slop`]: SearchGuildMessagesErrorType::Slop
pub const fn slop(value: usize) -> Result<(), SearchGuildMessagesError> {
    if value > SLOP_MAX {
        return Err(SearchGuildMessagesError {
            kind: SearchGuildMessagesErrorType::Slop { value },
        });
    }

    Ok(())
}

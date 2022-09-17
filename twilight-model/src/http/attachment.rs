//! Models used when sending attachments to Discord.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// Attachments used in messages.
///
/// # Examples
///
/// Create an attachment of a short JSON blob describing a cat with a
/// description for screen readers:
///
/// ```
/// use std::borrow::Cow;
/// use twilight_model::http::attachment::Attachment;
///
/// let filename = "twilight_sparkle.json".to_owned();
/// let file_content = br#"{
///     "best_friend": "Spike",
///     "cutie_mark": "sparkles",
///     "name": "Twilight Sparkle"
/// }"#
/// .to_vec();
/// let id = 1;
///
/// let mut attachment = Attachment::from_bytes(filename, Cow::from(file_content), id);
/// attachment.description("Raw data about Twilight Sparkle".to_owned());
/// ```
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Attachment<'a> {
    /// Description of the attachment, useful for screen readers and users
    /// requiring alt text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Content of the file.
    #[serde(skip)]
    pub file: Cow<'a, [u8]>,
    /// Name of the file.
    ///
    /// Examples may be "twilight_sparkle.png", "cat.jpg", or "logs.txt".
    pub filename: String,
    /// Unique ID of the attachment in the message.
    ///
    /// While attachment IDs can be the same as attachments in other messages,
    /// they must be unique within the same message. Attachment IDs don't need
    /// to be in any particular format; for example, IDs of 0, 100, the current
    /// timestamp, and so on are all valid.
    pub id: u64,
}

impl<'a> Attachment<'a> {
    /// Create an attachment from a filename and bytes.
    ///
    /// # Examples
    ///
    /// Create an attachment with a grocery list named "grocerylist.txt":
    ///
    /// ```
    /// use std::borrowed::Cow;
    /// use twilight_model::http::attachment::Attachment;
    ///
    /// let filename = "grocerylist.txt".to_owned();
    /// let file_content = b"Apples\nGrapes\nLemonade".to_vec();
    /// let id = 1;
    ///
    /// let attachment = Attachment::from_bytes(filename, Cow::from(file_content), id);
    /// ```
    pub const fn from_bytes(filename: String, file: Cow<'a, [u8]>, id: u64) -> Self {
        Self {
            description: None,
            file,
            filename,
            id,
        }
    }

    /// Set the description of the attachment.
    ///
    /// Attachment descriptions are useful for those requiring screen readers
    /// and are displayed as alt text.
    pub fn description(&mut self, description: String) {
        self.description = Some(description);
    }
}

//! Models used when sending attachments to Discord.

use serde::{Deserialize, Serialize};

/// Attachment for when creating and updating messages.
///
/// `id` can be any placeholder value. If attaching multiple files to the
/// message, each must be unique.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Attachment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip)]
    pub file: Vec<u8>,
    pub filename: String,
    pub id: u64,
}

impl Attachment {
    /// Create a attachment from a filename and bytes.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_http::Client;
    /// use twilight_model::http::attachment;
    ///
    /// let client = Client::new("my token".to_owned());
    ///
    /// let grocery_list: &str = "Apples/nGrapes/nLemonade";
    /// let message_content = "Here is the grocery list!".into();
    /// let attachments = Vec::from([Attachment::from_bytes(
    ///    "grocerylist.txt".to_owned(),
    ///    Vec::from(grocery_list),
    ///    0
    /// )]);
    ///
    /// http.create_message(package)
    ///    .content(&message_content)?
    ///    .attachments(&attachments)
    ///    .exec()
    ///    .await?;
    ///
    ///
    /// # Ok(()) }
    /// ```
    pub const fn from_bytes(filename: String, file: Vec<u8>, id: u64) -> Self {
        Self {
            description: None,
            file,
            filename,
            id,
        }
    }

    /// Set the description of a attachment, this is used for alt-text
    /// on Discords end.
    pub fn description(&mut self, description: String) {
        self.description = Some(description);
    }
}

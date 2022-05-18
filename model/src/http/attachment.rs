//! Models used when sending attachments to Discord.

use serde::{Deserialize, Serialize};

/// Attachment for when creating and updating messages.
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
    /// The id field is placeholder that can be any u64 value, such as 0,1,2...
    pub const fn from_bytes(filename: String, file: Vec<u8>, id: u64) -> Self {
        Self {
            description: None,
            file,
            filename,
            id,
        }
    }
    
     /// # Examples
    ///
    /// ```no_run
    /// # #[tokio::main] async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use twilight_http::Client;
    /// use twilight_model::http::attachment;
    ///
    /// let client = Client::new("my token".to_owned());
    /// let mut attachments: Vec<Attachment> = vec![];
    ///
    /// let grocery_list: &str = "Apples/nGrapes/nLemonade";
    /// let message_content = format!("Here is the grocery list!");
    ///         
    /// attachments.push(Attachment::from_bytes(
    ///    "grocerylist.txt".to_owned(),
    ///    Vec::from(grocery_list),
    ///    0
    /// ));
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

    /// Set the description of a attachment, this is used for alt-text
    /// on Discords end.
    pub fn description(&mut self, description: String) {
        self.description = Some(description);
    }
}

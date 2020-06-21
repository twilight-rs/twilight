use twilight_model::channel::embed::*;

/// Create an embed via a builder.
///
/// # Examples
///
/// ```rust,no_run
/// use twilight_builders::embed::EmbedBuilder;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
/// let embed = EmbedBuilder::new()
///     .description("Here's a list of reasons why Twilight is the best pony:")
///     .add_field("Wings", "She has wings.")
///         .inline()
///         .commit()
///     .add_field("Horn", "She can do magic, and she's really good at it.")
///         .commit()
///     .build();
/// # Ok(()) }
/// ```
#[derive(Clone, Debug)]
#[must_use = "The embed is not constructed. You need to call build to construct the embed."]
pub struct EmbedBuilder(Embed);

impl EmbedBuilder {
    pub fn new() -> Self {
        EmbedBuilder(Embed {
            author: None,
            color: None,
            description: None,
            fields: vec![],
            footer: None,
            image: None,
            kind: String::from("rich"),
            provider: None,
            thumbnail: None,
            timestamp: None,
            title: None,
            url: None,
            video: None,
        })
    }

    pub fn author(self) -> AuthorBuilder {
        AuthorBuilder::new(self)
    }

    pub fn color(mut self, color: u32) -> Self {
        self.0.color.replace(color);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.0.description.replace(description.into());
        self
    }

    pub fn add_field(self, name: impl Into<String>, value: impl Into<String>) -> FieldBuilder {
        FieldBuilder::new(self, name.into(), value.into())
    }

    pub fn footer(self, text: impl Into<String>) -> FooterBuilder {
        FooterBuilder::new(self, text.into())
    }

    pub fn image(mut self, url: impl Into<String>) -> Self {
        let image = EmbedImage {
            height: None,
            proxy_url: None,
            url: Some(url.into()),
            width: None,
        };
        self.0.image.replace(image);
        self
    }

    pub fn thumbnail(mut self, url: impl Into<String>) -> Self {
        let image = EmbedThumbnail {
            height: None,
            proxy_url: None,
            url: Some(url.into()),
            width: None,
        };
        self.0.thumbnail.replace(image);
        self
    }

    pub fn timestamp(mut self, timestamp: impl Into<String>) -> Self {
        self.0.timestamp.replace(timestamp.into());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.0.title.replace(title.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.0.url.replace(url.into());
        self
    }

    pub fn build(self) -> Embed {
        self.0
    }
}

impl Default for EmbedBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[must_use = "If commit is not run the author will not be changed."]
pub struct AuthorBuilder(EmbedAuthor, EmbedBuilder);

impl AuthorBuilder {
    fn new(ebb: EmbedBuilder) -> Self {
        AuthorBuilder(
            EmbedAuthor {
                icon_url: None,
                name: None,
                proxy_icon_url: None,
                url: None,
            },
            ebb,
        )
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.0.name.replace(name.into());
        self
    }

    pub fn icon_url(mut self, icon_url: impl Into<String>) -> Self {
        self.0.icon_url.replace(icon_url.into());
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.0.url.replace(url.into());
        self
    }

    pub fn commit(mut self) -> EmbedBuilder {
        (self.1).0.author.replace(self.0);
        self.1
    }
}

#[must_use = "If commit is not run the field will not be added."]
pub struct FieldBuilder(EmbedField, EmbedBuilder);

impl FieldBuilder {
    fn new(ebb: EmbedBuilder, name: String, value: String) -> Self {
        FieldBuilder(
            EmbedField {
                inline: false,
                name,
                value,
            },
            ebb,
        )
    }

    pub fn inline(mut self) -> Self {
        self.0.inline = true;
        self
    }

    pub fn commit(mut self) -> EmbedBuilder {
        (self.1).0.fields.push(self.0);
        self.1
    }
}

#[must_use = "If commit is not run the footer will not be added."]
pub struct FooterBuilder(EmbedFooter, EmbedBuilder);

impl FooterBuilder {
    fn new(ebb: EmbedBuilder, text: String) -> Self {
        FooterBuilder(
            EmbedFooter {
                icon_url: None,
                proxy_icon_url: None,
                text,
            },
            ebb,
        )
    }

    pub fn icon_url(mut self, url: impl Into<String>) -> Self {
        self.0.icon_url.replace(url.into());
        self
    }

    pub fn commit(mut self) -> EmbedBuilder {
        (self.1).0.footer.replace(self.0);
        self.1
    }
}

#[test]
fn builder_test() {
    let embed = EmbedBuilder::new()
        .color(0x0043FF)
        .description("Description")
        .timestamp("123")
        .footer("Warn")
        .icon_url("icon")
        .commit()
        .add_field("name", "title")
        .inline()
        .commit()
        .build();

    let expected = Embed {
        author: None,
        color: Some(17407),
        description: Some("Description".to_string()),
        fields: [EmbedField {
            inline: true,
            name: "name".to_string(),
            value: "title".to_string(),
        }]
        .to_vec(),
        footer: Some(EmbedFooter {
            icon_url: Some("icon".to_string()),
            proxy_icon_url: None,
            text: "Warn".to_string(),
        }),
        image: None,
        kind: "rich".to_string(),
        provider: None,
        thumbnail: None,
        timestamp: Some("123".to_string()),
        title: None,
        url: None,
        video: None,
    };

    assert_eq!(embed, expected);
}

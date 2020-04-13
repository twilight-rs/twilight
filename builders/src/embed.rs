use twilight_model::channel::embed::*;

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

    pub fn author(&mut self) -> AuthorBuilder<'_> {
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

    pub fn add_field(
        &mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> FieldBuilder<'_> {
        FieldBuilder::new(self, name.into(), value.into())
    }

    pub fn footer(&mut self, text: impl Into<String>) -> FooterBuilder<'_> {
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
pub struct AuthorBuilder<'a>(EmbedAuthor, &'a mut EmbedBuilder);

impl<'a> AuthorBuilder<'a> {
    fn new(ebb: &'a mut EmbedBuilder) -> Self {
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

    pub fn commit(self) {
        (self.1).0.author.replace(self.0);
    }
}

#[must_use = "If commit is not run the field will not be added."]
pub struct FieldBuilder<'a>(EmbedField, &'a mut EmbedBuilder);

impl<'a> FieldBuilder<'a> {
    fn new(ebb: &'a mut EmbedBuilder, name: String, value: String) -> Self {
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

    pub fn commit(self) {
        (self.1).0.fields.push(self.0);
    }
}

#[must_use = "If commit is not run the footer will not be added."]
pub struct FooterBuilder<'a>(EmbedFooter, &'a mut EmbedBuilder);

impl<'a> FooterBuilder<'a> {
    fn new(ebb: &'a mut EmbedBuilder, text: String) -> Self {
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

    pub fn commit(self) {
        (self.1).0.footer.replace(self.0);
    }
}

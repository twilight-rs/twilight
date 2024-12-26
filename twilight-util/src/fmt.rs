//! Provides the Format trait for specifying formatting with Discord markdown for strings.

use std::fmt::{Display, Formatter, Result as FmtResult};

/// Format is a trait specifying formatting with Discord markdown for strings.
pub trait MarkdownFormat<'a, T> {
    /// Returns the block quote formatting.
    #[must_use]
    fn block_quote(self) -> MarkdownDisplay<'a, T>;

    /// Returns the bold formatting.
    #[must_use]
    fn bold(self) -> MarkdownDisplay<'a, T>;

    /// Returns the codeblock formatting.
    #[must_use]
    fn codeblock(self, language: Option<&'a str>) -> MarkdownDisplay<'a, T>;

    /// Returns the H1 formatting.
    #[must_use]
    fn h1(self) -> MarkdownDisplay<'a, T>;

    /// Returns the H2 formatting.
    #[must_use]
    fn h2(self) -> MarkdownDisplay<'a, T>;

    /// Returns the H3 formatting.
    #[must_use]
    fn h3(self) -> MarkdownDisplay<'a, T>;

    /// Returns the inline code formatting.
    #[must_use]
    fn inline_code(self) -> MarkdownDisplay<'a, T>;

    /// Returns the italic formatting.
    #[must_use]
    fn italic(self) -> MarkdownDisplay<'a, T>;

    /// Returns the quote formatting.
    #[must_use]
    fn line_quote(self) -> MarkdownDisplay<'a, T>;

    /// Returns the masked links formatting.
    ///
    /// This assumes `self` being the URL to be masked.
    #[must_use]
    fn masked_link(self, mask: &'a str) -> MarkdownDisplay<'a, T>;

    /// Returns the underline formatting.
    #[must_use]
    fn underline(self) -> MarkdownDisplay<'a, T>;

    /// Returns the spoiler formatting.
    #[must_use]
    fn spoiler(self) -> MarkdownDisplay<'a, T>;

    /// Returns the strikethrough formatting.
    #[must_use]
    fn strikethrough(self) -> MarkdownDisplay<'a, T>;
}

impl<'a, T> MarkdownFormat<'a, T> for T {
    fn block_quote(self) -> MarkdownDisplay<'a, T> {
        MarkdownDisplay::new(self, MarkdownFlavour::BlockQuote)
    }

    fn bold(self) -> MarkdownDisplay<'a, T> {
        MarkdownDisplay::new(self, MarkdownFlavour::Bold)
    }

    fn codeblock(self, language: Option<&'a str>) -> MarkdownDisplay<'a, T> {
        MarkdownDisplay::new(self, MarkdownFlavour::Codeblock { language })
    }

    fn h1(self) -> MarkdownDisplay<'a, T> {
        MarkdownDisplay::new(self, MarkdownFlavour::H1)
    }

    fn h2(self) -> MarkdownDisplay<'a, T> {
        MarkdownDisplay::new(self, MarkdownFlavour::H2)
    }

    fn h3(self) -> MarkdownDisplay<'a, T> {
        MarkdownDisplay::new(self, MarkdownFlavour::H3)
    }

    fn inline_code(self) -> MarkdownDisplay<'a, T> {
        MarkdownDisplay::new(self, MarkdownFlavour::InlineCode)
    }

    fn italic(self) -> MarkdownDisplay<'a, T> {
        MarkdownDisplay::new(self, MarkdownFlavour::Italic)
    }

    fn line_quote(self) -> MarkdownDisplay<'a, T> {
        MarkdownDisplay::new(self, MarkdownFlavour::LineQuote)
    }

    fn masked_link(self, mask: &'a str) -> MarkdownDisplay<'a, T> {
        MarkdownDisplay::new(self, MarkdownFlavour::MaskedLink { mask })
    }

    fn underline(self) -> MarkdownDisplay<'a, T> {
        MarkdownDisplay::new(self, MarkdownFlavour::Underline)
    }

    fn spoiler(self) -> MarkdownDisplay<'a, T> {
        MarkdownDisplay::new(self, MarkdownFlavour::Spoiler)
    }

    fn strikethrough(self) -> MarkdownDisplay<'a, T> {
        MarkdownDisplay::new(self, MarkdownFlavour::Strikethrough)
    }
}

/// Formatter to display some content with markdown formatting
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MarkdownDisplay<'a, T> {
    content: T,
    flavour: MarkdownFlavour<'a>,
}

impl<'a, T> MarkdownDisplay<'a, T> {
    pub(self) fn new(content: T, flavour: MarkdownFlavour<'a>) -> Self {
        Self { content, flavour }
    }
}

impl<'a, T: Display> Display for MarkdownDisplay<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.flavour {
            MarkdownFlavour::BlockQuote => f.write_str(">>> ")?,
            MarkdownFlavour::Bold => f.write_str("**")?,
            MarkdownFlavour::Codeblock { language } => {
                f.write_str("```")?;
                if let Some(language) = language {
                    f.write_str(language)?;
                }
                f.write_str("\n")?;
            }
            MarkdownFlavour::H1 => f.write_str("# ")?,
            MarkdownFlavour::H2 => f.write_str("## ")?,
            MarkdownFlavour::H3 => f.write_str("### ")?,
            MarkdownFlavour::InlineCode => f.write_str("`")?,
            MarkdownFlavour::Italic => f.write_str("*")?,
            MarkdownFlavour::LineQuote => f.write_str("> ")?,
            MarkdownFlavour::MaskedLink { mask } => {
                f.write_str("[")?;
                f.write_str(mask)?;
                f.write_str("](")?;
            }
            MarkdownFlavour::Underline => f.write_str("__")?,
            MarkdownFlavour::Spoiler => f.write_str("||")?,
            MarkdownFlavour::Strikethrough => f.write_str("~~")?,
        };

        Display::fmt(&self.content, f)?;

        match self.flavour {
            MarkdownFlavour::Bold => f.write_str("**"),
            MarkdownFlavour::Codeblock { language } => f.write_str("```"),
            MarkdownFlavour::InlineCode => f.write_str("`"),
            MarkdownFlavour::Italic => f.write_str("*"),
            MarkdownFlavour::MaskedLink { mask } => f.write_str(")"),
            MarkdownFlavour::Underline => f.write_str("__"),
            MarkdownFlavour::Spoiler => f.write_str("||"),
            MarkdownFlavour::Strikethrough => f.write_str("~~"),
            _ => Ok(()),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(self) enum MarkdownFlavour<'a> {
    BlockQuote,
    Bold,
    Codeblock { language: Option<&'a str> },
    H1,
    H2,
    H3,
    InlineCode,
    Italic,
    LineQuote,
    MaskedLink { mask: &'a str },
    Underline,
    Spoiler,
    Strikethrough,
}

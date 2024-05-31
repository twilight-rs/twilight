//! Provides the Format trait for specifying formatting with Discord markdown for strings.

/// Format is a trait specifying formatting with Discord markdown for strings.
pub trait Format {
    /// Returns the block quote formatting for a string.
    fn block_quote(self) -> Self;

    /// Returns the bold formatting for a string.
    fn bold(self) -> Self;

    /// Returns the codeblock formatting for a string.
    fn codeblock(self, language: &str) -> Self;

    /// Returns the H1 formatting for a string.
    fn h1(self) -> Self;

    /// Returns the H2 formatting for a string.
    fn h2(self) -> Self;

    /// Returns the H3 formatting for a string.
    fn h3(self) -> Self;

    /// Returns the inline code formatting for a string.
    fn inline_code(self) -> Self;

    /// Returns the italic formatting for a string.
    fn italic(self) -> Self;

    /// Returns the quote formatting for a string.
    fn line_quote(self) -> Self;

    /// Returns the masked links formatting for a string.
    ///
    /// This assumes `self` being the URL to be masked.
    fn masked_links(self, text: &str) -> Self;

    /// Returns the relative timestamp formatting for a string.
    fn relative_timestamp(self) -> Self;

    /// Returns the underline formatting for a string.
    fn underline(self) -> Self;

    /// Returns the spoiler formatting for a string.
    fn spoiler(self) -> Self;

    /// Returns the strikethrough formatting for a string.
    fn strikethrough(self) -> Self;
}

impl Format for String {
    fn block_quote(self) -> Self {
        format!(">>> {self}")
    }

    fn bold(self) -> Self {
        format!("**{self}**")
    }

    fn codeblock(self, language: &str) -> Self {
        format!("```{language}\n{self}```")
    }

    fn h1(self) -> Self {
        format!("# {self}")
    }

    fn h2(self) -> Self {
        format!("## {self}")
    }

    fn h3(self) -> Self {
        format!("### {self}")
    }

    fn inline_code(self) -> Self {
        format!("`{self}`")
    }

    fn italic(self) -> Self {
        format!("*{self}*")
    }

    fn line_quote(self) -> Self {
        format!("> {self}")
    }

    fn masked_links(self, text: &str) -> Self {
        format!("[{text}]({self})")
    }

    fn relative_timestamp(self) -> Self {
        format!("<t:{self}:R>")
    }

    fn underline(self) -> Self {
        format!("__{self}__")
    }

    fn spoiler(self) -> Self {
        format!("||{self}||")
    }

    fn strikethrough(self) -> Self {
        format!("~~{self}~~")
    }
}

//! Provides the Format trait for specifying formatting with Discord markdown for strings.

/// Format is a trait specifying formatting with Discord markdown for strings
pub trait Format {
    /// Returns the bold formatting for a string.
    fn bold(self) -> Self;

    /// Returns the codeblock formatting for a string.
    fn codeblock(self, language: &str) -> Self;

    /// Returns the inline code formatting for a string.
    fn inline_code(self) -> Self;

    /// Returns the italic formatting for a string.
    fn italic(self) -> Self;

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
    fn bold(self) -> Self {
        format!("**{self}**")
    }

    fn codeblock(self, language: &str) -> Self {
        format!("```{language}\n{self}```")
    }

    fn inline_code(self) -> Self {
        format!("`{self}`")
    }

    fn italic(self) -> Self {
        format!("*{self}*")
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

use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    str::CharIndices,
};

/// An iterator over command arguments.
#[derive(Clone)]
pub struct Arguments<'a> {
    buf: &'a str,
    indices: CharIndices<'a>,
    idx: usize,
}

impl<'a> Arguments<'a> {
    /// Returns a new iterator of arguments from a buffer.
    pub fn new(buf: &'a str) -> Self {
        Self::from(buf)
    }

    /// Returns a view of the underlying buffer of arguments.
    ///
    /// This is exactly like [`std::str::Chars::as_str`].
    ///
    /// # Examples
    ///
    /// When the command is `"!echo foo bar baz"` and the command is `"echo"`,
    /// then this returns `"foo bar baz"`.
    ///
    /// ```rust
    /// use twilight_command_parser::{Command, CommandParserConfig, Parser};
    ///
    /// let mut config = CommandParserConfig::new();
    /// config.add_prefix("!");
    /// config.add_command("echo", false);
    /// let parser = Parser::new(config);
    ///
    /// if let Some(Command { arguments, .. }) = parser.parse("!echo foo bar baz") {
    ///     assert_eq!("foo bar baz", arguments.as_str());
    /// }
    /// # else { panic!("Not command match"); }
    /// ```
    pub fn as_str(&self) -> &str {
        self.buf
    }

    /// Returns the remainder of the buffer that hasn't been parsed.
    ///
    /// # Examples
    ///
    /// If you have extracted two arguments already, then you can consume the
    /// rest of the arguments:
    ///
    /// ```rust
    /// use twilight_command_parser::Arguments;
    ///
    /// let mut args = Arguments::new("1 2 3 4 5");
    /// assert_eq!(Some("1"), args.next());
    /// assert_eq!(Some("2"), args.next());
    /// assert_eq!(Some("3 4 5"), args.into_remainder());
    /// ```
    pub fn into_remainder(self) -> Option<&'a str> {
        self.buf.get(self.idx..)
    }
}

impl<'a> From<&'a str> for Arguments<'a> {
    fn from(buf: &'a str) -> Self {
        Self {
            buf: buf.trim(),
            indices: buf.trim().char_indices(),
            idx: 0,
        }
    }
}

impl<'a> Debug for Arguments<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.debug_struct("Arguments")
            .field("buf", &self.buf)
            .field("idx", &self.idx)
            .finish()
    }
}

impl<'a> Iterator for Arguments<'a> {
    type Item = &'a str;

    // todo: clean this up
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx > self.buf.len() {
            return None;
        }

        let mut start_idx = self.idx;
        let mut quoted = false;
        let mut started = false;

        while let Some((i, ch)) = self.indices.next() {
            if quoted {
                if ch == '"' {
                    let v = self.buf.get(start_idx..i);
                    self.idx = i + 1;

                    return v.map(str::trim);
                }
            } else if ch == ' ' {
                if started {
                    let v = self.buf.get(start_idx..i);
                    self.idx = i + 1;

                    return v.map(str::trim);
                }
                self.idx = i;
                start_idx = i;
                started = true;
                continue;
            } else if ch == '"' {
                start_idx = i + 1;
                quoted = true;
            }

            self.idx = i;
            started = true;
        }

        self.idx = usize::max_value();

        match self.buf.get(start_idx..) {
            Some("") | None => None,
            Some(v) => Some(v.trim()),
        }
    }
}

#[allow(clippy::non_ascii_literal)]
#[cfg(test)]
mod tests {
    use super::Arguments;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(Arguments<'_>: Clone, Debug, From<&'static str>, Iterator, Send, Sync);

    #[test]
    fn test_as_str() {
        let args = Arguments::from("foo bar baz");
        assert_eq!("foo bar baz", args.as_str());
    }

    #[test]
    fn test_simple_args() {
        let mut args = Arguments::new("foo bar baz");
        assert_eq!(Some("foo"), args.next());
        assert_eq!(Some("bar"), args.next());
        assert_eq!(Some("baz"), args.next());
        assert_eq!(None, args.next());
    }

    #[test]
    fn test_quoted_args() {
        let mut args = Arguments::new(r#"this "is a longer argument" here"#);
        assert_eq!(Some("this"), args.next());
        assert_eq!(Some("is a longer argument"), args.next());
        assert_eq!(Some("here"), args.next());
        assert_eq!(None, args.next());
    }

    #[test]
    fn test_quoted_close_args() {
        let mut args = Arguments::new(r#""kind of weird""but okay"#);
        assert_eq!(Some("kind of weird"), args.next());
        assert_eq!(Some("but okay"), args.next());
        assert_eq!(None, args.next());
    }

    #[test]
    fn test_unicode_chars_1() {
        let mut args = Arguments::new("𝓒𝓢𝓐 nice try");
        assert_eq!(Some("𝓒𝓢𝓐"), args.next());
        assert_eq!(Some("nice"), args.next());
        assert_eq!(Some("try"), args.next());
        assert_eq!(None, args.next());
    }

    #[test]
    fn test_unicode_chars_2() {
        let mut args = Arguments::new("Saighdiúr réalta what even");
        assert_eq!(Some("Saighdiúr"), args.next());
        assert_eq!(Some("réalta"), args.next());
        assert_eq!(Some("what"), args.next());
        assert_eq!(Some("even"), args.next());
        assert_eq!(None, args.next());
    }

    #[test]
    fn test_quoted_unicode_chars() {
        let mut args = Arguments::new(r#""𝓒𝓢𝓐 | CSA" amazing try"#);
        assert_eq!(Some("𝓒𝓢𝓐 | CSA"), args.next());
        assert_eq!(Some("amazing"), args.next());
        assert_eq!(Some("try"), args.next());
        assert_eq!(None, args.next());
    }

    #[test]
    fn test_emote() {
        let mut args = Arguments::new("why an emote 🙃");
        assert_eq!(Some("why"), args.next());
        assert_eq!(Some("an"), args.next());
        assert_eq!(Some("emote"), args.next());
        assert_eq!(Some("🙃"), args.next());
        assert_eq!(None, args.next());
    }

    #[test]
    fn test_quoted_emote() {
        let mut args = Arguments::new(r#"omg "😕 - 😟" kewl"#);
        assert_eq!(Some("omg"), args.next());
        assert_eq!(Some("😕 - 😟"), args.next());
        assert_eq!(Some("kewl"), args.next());
        assert_eq!(None, args.next());
    }
}

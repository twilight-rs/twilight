/// An iterator over command arguments.
#[derive(Clone, Debug)]
pub struct Arguments<'a> {
    buf: &'a str,
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
    /// use dawn_command_parser::{Command, Config, Parser};
    ///
    /// let mut config = Config::new();
    /// config.add_prefix("!");
    /// config.add_command("echo");
    /// let parser = Parser::new(config);
    ///
    /// if let Some(Command { arguments, .. }) = parser.parse("!echo foo bar baz") {
    ///     assert_eq!("foo bar baz", arguments.as_str());
    /// }
    /// # else { panic!("Not command match"); }
    /// ```
    ///
    /// [`std::str::Chars::as_str`]: https://doc.rust-lang.org/std/str/struct.Chars.html#method.as_str
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
    /// use dawn_command_parser::Arguments;
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
            idx: 0,
        }
    }
}

impl<'a> Iterator for Arguments<'a> {
    type Item = &'a str;

    // todo: clean this up
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx > self.buf.len() {
            return None;
        }

        let mut idx = self.idx;
        let mut quoted = false;
        let mut started = false;

        if let Some(r#"""#) = self.buf.get(idx..=idx) {
            idx += 1;
            quoted = true;
            started = true;
            self.idx += 1;
        }

        while let Some(ch) = self.buf.get(idx..=idx) {
            if quoted {
                if ch == r#"""# {
                    let v = self.buf.get(self.idx..idx);
                    self.idx = idx + 1;

                    return v.map(str::trim);
                }
            } else if ch == " " {
                if started {
                    let v = self.buf.get(self.idx..idx);
                    self.idx = idx + 1;

                    return v.map(str::trim);
                } else {
                    self.idx += 1;
                    idx += 1;

                    continue;
                }
            }

            idx += 1;
            started = true;
        }

        let idx = self.idx;
        self.idx = usize::max_value();

        match self.buf.get(idx..) {
            Some("") | None => None,
            Some(v) => Some(v.trim()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Arguments;

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
}

use crate::{Arguments, CommandParserConfig};

/// Indicator that a command was used.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Command<'a> {
    /// A lazy iterator of command arguments. Refer to its documentation on
    /// how to use it.
    pub arguments: Arguments<'a>,
    /// The name of the command that was called.
    pub name: &'a str,
    /// The prefix used to call the command.
    pub prefix: &'a str,
}

/// A struct to parse prefixes, commands, and arguments out of messages.
///
/// While parsing, the parser takes into account the configuration that it was
/// configured with. This configuration is mutable during runtime via the
/// [`Parser::config_mut`] method.
///
/// After parsing, you're given an optional [`Command`]: a struct representing a
/// command and its relevant information. Refer to its documentation for more
/// information.
///
/// # Examples
///
/// Using a parser configured with the commands `"echo"` and `"ping"` and the
/// prefix `"!"`, parse the message "!echo foo bar baz":
///
/// ```rust
/// use twilight_command_parser::{Command, CommandParserConfig, Parser};
///
/// let mut config = CommandParserConfig::new();
/// config.add_command("echo", false);
/// config.add_command("ping", false);
/// config.add_prefix("!");
///
/// let parser = Parser::new(config);
///
/// if let Some(command) = parser.parse("!echo foo bar baz") {
///     match command {
///         Command { name: "echo", arguments, .. } => {
///             let content = arguments.as_str();
///
///             println!("Got a request to echo `{}`", content);
///         },
///         Command { name: "ping", .. } => {
///             println!("Got a ping request");
///         },
///         _ => {},
///     }
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Parser<'a> {
    config: CommandParserConfig<'a>,
}

impl<'a> Parser<'a> {
    /// Creates a new parser from a given configuration.
    pub fn new(config: impl Into<CommandParserConfig<'a>>) -> Self {
        Self {
            config: config.into(),
        }
    }

    /// Returns an immutable reference to the configuration.
    pub fn config(&self) -> &CommandParserConfig<'a> {
        &self.config
    }

    /// Returns a mutable reference to the configuration.
    pub fn config_mut(&mut self) -> &mut CommandParserConfig<'a> {
        &mut self.config
    }

    /// Parses a command out of a buffer.
    ///
    /// If a configured prefix and command are in the buffer, then some
    /// [`Command`] is returned with them and a lazy iterator of the
    /// argument list.
    ///
    /// If a matching prefix or command weren't found, then `None` is returned.
    ///
    /// Refer to the struct-level documentation on how to use this.
    pub fn parse(&'a self, buf: &'a str) -> Option<Command<'a>> {
        let prefix = self.find_prefix(buf)?;
        self.parse_with_prefix(prefix, buf)
    }

    /// Parse a command out of a buffer with a specific prefix.
    ///
    /// Instead of using the list of set prefixes, give a specific prefix
    /// to parse the message, this can be used to have a kind of dynamic
    /// prefixes.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use twilight_command_parser::{Command, CommandParserConfig, Parser};
    /// # fn example() -> Option<()> {
    /// let mut config = CommandParserConfig::new();
    /// config.add_prefix("!");
    /// config.add_command("echo", false);
    ///
    /// let parser = Parser::new(config);
    ///
    /// let command = parser.parse_with_prefix("=", "=echo foo")?;
    ///
    /// assert_eq!("=", command.prefix);
    /// assert_eq!("echo", command.name);
    /// # Some(())
    /// # }
    /// ```
    pub fn parse_with_prefix(&'a self, prefix: &'a str, buf: &'a str) -> Option<Command<'a>> {
        if !buf.starts_with(prefix) {
            return None;
        }

        let mut idx = prefix.len();
        let command_buf = buf.get(idx..)?;
        let command = self.find_command(command_buf)?;

        idx += command.len();

        // Advance from the amount of whitespace that was between the prefix and
        // the command name.
        idx += command_buf.len() - command_buf.trim_start().len();

        Some(Command {
            arguments: Arguments::new(buf.get(idx..)?),
            name: command,
            prefix,
        })
    }

    fn find_command(&'a self, buf: &'a str) -> Option<&'a str> {
        let buf = buf.split_whitespace().next()?;
        self.config.commands.iter().find_map(|command| {
            if command == buf {
                Some(command.as_ref())
            } else {
                None
            }
        })
    }

    fn find_prefix(&self, buf: &str) -> Option<&str> {
        self.config.prefixes.iter().find_map(|prefix| {
            if buf.starts_with(prefix.as_ref()) {
                Some(prefix.as_ref())
            } else {
                None
            }
        })
    }
}

impl<'a, T: Into<CommandParserConfig<'a>>> From<T> for Parser<'a> {
    fn from(config: T) -> Self {
        Self::new(config)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Command, CommandParserConfig, Parser};
    use static_assertions::{assert_fields, assert_impl_all};
    use std::fmt::Debug;

    assert_fields!(Command<'_>: arguments, name, prefix);
    assert_impl_all!(Command<'_>: Clone, Debug, Send, Sync);
    assert_impl_all!(Parser<'_>: Clone, Debug, Send, Sync);

    fn simple_config() -> Parser<'static> {
        let mut config = CommandParserConfig::new();
        config.add_prefix("!");
        config.add_command("echo", false);

        Parser::new(config)
    }

    #[test]
    fn double_command() {
        let parser = simple_config();
        if parser.parse("!echoecho").is_some() {
            panic!("Double match!")
        }
    }

    #[test]
    fn test_case_sensitive() {
        let mut parser = simple_config();
        let message_ascii = "!EcHo this is case insensitive";
        let message_unicode = "!wEiSS is white";
        let message_unicode_2 = "!\u{3b4} is delta";

        // Case insensitive - ASCII
        let command = parser
            .parse(message_ascii)
            .expect("Parser is case sensitive");
        assert_eq!(
            "echo", command.name,
            "Command name should have the same case as in the CommandParserConfig"
        );

        // Case insensitive - Unicode
        parser.config.add_command("wei\u{df}", false);
        let command = parser
            .parse(message_unicode)
            .expect("Parser is case sensitive");
        assert_eq!(
            "wei\u{df}", command.name,
            "Command name should have the same case as in the CommandParserConfig"
        );

        parser.config.add_command("\u{394}", false);
        let command = parser
            .parse(message_unicode_2)
            .expect("Parser is case sensitive");
        assert_eq!(
            "\u{394}", command.name,
            "Command name should have the same case as in the CommandParserConfig"
        );

        // Case sensitive
        let config = parser.config_mut();
        config.commands.clear();
        config.add_command("echo", true);
        config.add_command("wei\u{df}", true);
        config.add_command("\u{394}", true);
        assert!(
            parser.parse(message_ascii).is_none(),
            "Parser is not case sensitive"
        );
        assert!(
            parser.parse(message_unicode).is_none(),
            "Parser is not case sensitive"
        );
        assert!(
            parser.parse(message_unicode_2).is_none(),
            "Parser is not case sensitive"
        );
    }

    #[test]
    fn test_simple_config_no_prefix() {
        let mut parser = simple_config();
        parser.config_mut().remove_prefix("!");
    }

    #[test]
    fn test_simple_config_parser() {
        let parser = simple_config();

        match parser.parse("!echo what a test") {
            Some(Command { name, prefix, .. }) => {
                assert_eq!("echo", name);
                assert_eq!("!", prefix);
            }
            other => panic!("Not command: {:?}", other),
        }
    }

    #[test]
    fn test_unicode_command() {
        let mut parser = simple_config();
        parser.config_mut().add_command("\u{1f44e}", false);

        assert!(parser.parse("!\u{1f44e}").is_some());
    }

    #[test]
    fn test_unicode_prefix() {
        let mut parser = simple_config();
        parser.config_mut().add_prefix("\u{1f44d}"); // thumbs up unicode

        assert!(parser.parse("\u{1f44d}echo foo").is_some());
    }

    #[test]
    fn test_dynamic_prefix() {
        let parser = simple_config();

        let command = parser.parse_with_prefix("=", "=echo foo").unwrap();

        assert_eq!("=", command.prefix);
        assert_eq!("echo", command.name);
    }

    #[test]
    fn test_prefix_mention() {
        let mut config = CommandParserConfig::new();
        config.add_prefix("foo");
        config.add_command("dump", false);
        let parser = Parser::new(config);

        let Command {
            mut arguments,
            name,
            prefix,
        } = parser.parse("foo dump test").unwrap();
        assert_eq!("foo", prefix);
        assert_eq!("dump", name);
        assert_eq!(Some("test"), arguments.next());
        assert!(arguments.next().is_none());
    }
}

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
/// config.command("echo").add();
/// config.command("ping").add();
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
///
/// [`Command`]: struct.Command.html
/// [`Parser::config_mut`]: #method.config_mut
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
    ///
    /// [`Command`]: struct.Command.html
    pub fn parse(&'a self, buf: &'a str) -> Option<Command<'a>> {
        let (prefix, padding) = self.find_prefix(buf)?;
        let mut idx = prefix.len();

        match buf.get(idx..)? {
            v if !v.starts_with(padding) => return None,
            _ => {},
        }

        idx += padding.len();

        let command_buf = buf.get(idx..)?;
        let command = self.find_command(command_buf)?;

        idx += command.len();

        Some(Command {
            arguments: Arguments::new(buf.get(idx..)?),
            name: command,
            prefix,
        })
    }

    fn find_command(&'a self, buf: &'a str) -> Option<&'a str> {
        let buf = buf.split_whitespace().next()?;
        self.config.commands().iter().find_map(|command| {
            if command == buf {
                Some(command.as_ref())
            } else {
                None
            }
        })
    }

    fn find_prefix(&self, buf: &str) -> Option<(&str, &str)> {
        self.config.prefixes().iter().find_map(|(prefix, padding)| {
            if buf.starts_with(prefix.as_ref()) {
                Some((prefix.as_ref(), padding.as_ref()))
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

    fn simple_config() -> Parser<'static> {
        let mut config = CommandParserConfig::new();
        config.add_prefix("!");
        config.command("echo").add();

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
        let Command {
            name, ..
        } = parser
            .parse(message_ascii)
            .expect("Parser is case sensitive");
        assert_eq!(
            "echo", name,
            "Command name should have the same case as in the CommandParserConfig"
        );

        // Case insensitive - Unicode
        parser.config.command("wei\u{df}").add();
        let Command {
            name, ..
        } = parser
            .parse(message_unicode)
            .expect("Parser is case sensitive");
        assert_eq!(
            "wei\u{df}", name,
            "Command name should have the same case as in the CommandParserConfig"
        );

        parser.config.command("\u{394}").add();
        let Command {
            name, ..
        } = parser
            .parse(message_unicode_2)
            .expect("Parser is case sensitive");
        assert_eq!(
            "\u{394}", name,
            "Command name should have the same case as in the CommandParserConfig"
        );

        // Case sensitive
        let config = parser.config_mut();
        config.commands_mut().clear();
        config.command("echo").case_sensitive().add();
        config.command("wei\u{df}").case_sensitive().add();
        config.command("\u{394}").case_sensitive().add();
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
            Some(Command {
                name,
                prefix,
                ..
            }) => {
                assert_eq!("echo", name);
                assert_eq!("!", prefix);
            },
            other => panic!("Not command: {:?}", other),
        }
    }

    #[test]
    fn test_unicode_command() {
        let mut parser = simple_config();
        parser.config_mut().command("\u{1f44e}").add(); // thumbs down unicode

        assert!(parser.parse("!\u{1f44e}").is_some());
    }

    #[test]
    fn test_unicode_prefix() {
        let mut parser = simple_config();
        parser.config_mut().add_prefix("\u{1f44d}"); // thumbs up unicode

        assert!(parser.parse("\u{1f44d}echo foo").is_some());
    }
}

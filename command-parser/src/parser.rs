use crate::{Arguments, Config};

/// Indicator that a command was used.
#[derive(Clone, Debug)]
pub struct Command<'a> {
    /// A lazy iterator of command arguments. Refer to its documentation on
    /// how to use it.
    pub arguments: Arguments<'a>,
    /// The name of the command that was called.
    pub name: &'a str,
    /// The prefix used to call the command.
    pub prefix: &'a str,
    __nonexhaustive: (),
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
/// use dawn_command_parser::{Command, Config, Parser};
///
/// let mut config = Config::new();
/// config.add_command("echo");
/// config.add_command("ping");
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
    config: Config<'a>,
}

impl<'a> Parser<'a> {
    /// Creates a new parser from a given configuration.
    pub fn new(config: impl Into<Config<'a>>) -> Self {
        Self {
            config: config.into(),
        }
    }

    /// Returns an immutable reference to the configuration.
    pub fn config(&self) -> &Config<'a> {
        &self.config
    }

    /// Returns a mutable reference to the configuration.
    pub fn config_mut(&mut self) -> &mut Config<'a> {
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
            __nonexhaustive: (),
        })
    }

    fn find_command(&'a self, buf: &'a str) -> Option<&'a str> {
        self.config
            .commands()
            .iter()
            .find(|command| buf.starts_with(&command[..]))
            .map(AsRef::as_ref)
    }

    fn find_prefix(&self, buf: &str) -> Option<(&str, &str)> {
        self.config
            .prefixes()
            .iter()
            .find(|(prefix, _)| buf.starts_with(prefix.as_ref()))
            .map(|(prefix, padding)| (prefix.as_ref(), padding.as_ref()))
    }
}

impl<'a, T: Into<Config<'a>>> From<T> for Parser<'a> {
    fn from(config: T) -> Self {
        Self::new(config)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Command, Config, Parser};

    fn simple_config() -> Parser<'static> {
        let mut config = Config::new();
        config.add_prefix("!");
        config.add_command("echo");

        Parser::new(config)
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
                arguments: _,
                name,
                prefix,
                __nonexhaustive: _,
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
        parser.config_mut().add_command("👎"); // thumbs down unicode

        assert!(parser.parse("!👎").is_some());
    }

    #[test]
    fn test_unicode_prefix() {
        let mut parser = simple_config();
        parser.config_mut().add_prefix("👍"); // thumbs up unicode

        assert!(parser.parse("👍echo foo").is_some());
    }
}

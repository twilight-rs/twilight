use crate::{Arguments, Config};

/// The output of parsing a potential command.
#[derive(Clone, Debug)]
pub enum Output<'a> {
    /// Indicator that a command was used.
    Command {
        /// A lazy iterator of command arguments. Refer to its documentation on
        /// how to use it.
        arguments: Arguments<'a>,
        /// The name of the command that was called.
        name: &'a str,
        /// The prefix used to call the command.
        prefix: &'a str,
    },
    /// Indicator that a message is from an ignored guild.
    IgnoredGuild,
    /// Indicator that a message is from an ignored user.
    IgnoredUser,
    /// Indicator that the message wasn't ignored, but that no command was
    /// parsed from it.
    NoMatch,
}

impl<'a> Output<'a> {
    /// Whether an output is about a command.
    pub fn is_command(&self) -> bool {
        match self {
            Output::Command {
                ..
            } => true,
            _ => false,
        }
    }

    /// Whether an output is about an ignored guild.
    pub fn is_ignored_guild(&self) -> bool {
        match self {
            Output::IgnoredGuild => true,
            _ => false,
        }
    }

    /// Whether an output is about an ignored user.
    pub fn is_ignored_user(&self) -> bool {
        match self {
            Output::IgnoredUser => true,
            _ => false,
        }
    }

    /// Whether an output is that there was no match.
    pub fn is_no_match(&self) -> bool {
        match self {
            Output::NoMatch => true,
            _ => false,
        }
    }
}

/// A struct to parse prefixes, commands, and arguments out of messages.
///
/// While parsing, the parser takes into account the configuration that it was
/// configured with. This configuration is mutable during runtime via the
/// [`Parser::config_mut`] method.
///
/// After parsing, you're given an [`Output`]: an enum representing whether a
/// command was found and if so its relevant information, it was ignored due to
/// an ignored guild or user, or if no match was found. Refer to its
/// documentation for more information on these variants.
///
/// # Examples
///
/// Using a parser configured with the commands `"echo"` and `"ping"` and the
/// prefix `"!"`, parse the message "!echo foo bar baz":
///
/// ```rust
/// use dawn_command_parser::{Config, Output, Parser};
///
/// let mut config = Config::new();
/// config.add_command("echo");
/// config.add_command("ping");
/// config.add_prefix("!");
///
/// let parser = Parser::new(config);
///
/// match parser.parse_str("!echo foo bar baz") {
///     Output::Command { name: "echo", arguments, .. } => {
///         let content = arguments.as_str();
///
///         println!("Got a request to echo `{}`", content);
///     },
///     Output::Command { name: "ping", .. } => {
///         println!("Got a ping request");
///     },
///     _ => {},
/// }
/// ```
///
/// [`Output`]: enum.Output.html
/// [`Parser::config_mut`]: #method.config_mut
#[derive(Clone, Debug)]
pub struct Parser<'a> {
    config: Config<'a>,
}

impl<'a> Parser<'a> {
    /// Creates a new parser from a given configuration.
    pub fn new(config: Config<'a>) -> Self {
        Self {
            config,
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

    /// Parses a `dawn_model` `Message`.
    ///
    /// This will check if the author is in the list of ignored users and return
    /// [`Output::IgnoredUser`] if so. If the guild it was sent in is in the
    /// list of ignored guilds then [`Output::IgnoredGuild`] is returned.
    ///
    /// After these two checks, operations are delegated to [`parse_str`].
    ///
    /// [`Output::IgnoredGuild`]: enum.Output.html#variant.IgnoredGuild
    /// [`Output::IgnoredUser`]: enum.Output.html#variant.IgnoredUser
    /// [`parse_str`]: #method.parse_str
    #[cfg(feature = "dawn-model")]
    pub fn parse(&'a self, msg: &'a dawn_model::channel::Message) -> Output<'a> {
        if let Some(ref guild_id) = msg.guild_id {
            if self.config.ignore_guilds().contains(&guild_id.0) {
                return Output::IgnoredGuild;
            }
        }

        if self.config.ignore_users().contains(&msg.author.id.0) {
            return Output::IgnoredUser;
        }

        self.parse_str(&msg.content)
    }

    /// Parses a command out of a buffer.
    ///
    /// If a configured prefix and command are in the buffer, then
    /// [`Output::Command`] is returned with them and a lazy iterator of the
    /// argument list.
    ///
    /// If a matching prefix or command weren't found, then [`Output::NoMatch`]
    /// is returned.
    ///
    /// Refer to the struct-level documentation on how to use this.
    ///
    /// [`Output::Command`]: enum.Output.html#variant.Command
    /// [`Output::NoMatch`]: enum.Output.html#variant.NoMatch
    pub fn parse_str(&'a self, buf: &'a str) -> Output<'a> {
        let (prefix, padding) = match self.find_prefix(buf) {
            Some(v) => v,
            None => return Output::NoMatch,
        };
        let mut idx = prefix.len();

        match buf.get(idx..) {
            Some(v) if !v.starts_with(padding) => return Output::NoMatch,
            Some(_) => {},
            None => return Output::NoMatch,
        }

        idx += padding.len();

        let command_buf = match buf.get(idx..) {
            Some(command_buf) => command_buf,
            None => return Output::NoMatch,
        };

        let command = match self.find_command(command_buf) {
            Some(v) => v,
            None => return Output::NoMatch,
        };

        idx += command.len();

        Output::Command {
            arguments: Arguments::new(buf.get(idx..).unwrap()),
            name: command,
            prefix,
        }
    }

    fn find_command(&'a self, buf: &'a str) -> Option<&'a str> {
        for command in self.config.commands() {
            if buf.starts_with(command) {
                return Some(command);
            }
        }

        None
    }

    fn find_prefix(&self, buf: &str) -> Option<(&str, &str)> {
        for (prefix, padding) in self.config.prefixes() {
            if buf.starts_with(prefix.as_ref()) {
                return Some((prefix, padding.as_ref()));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{Config, Output, Parser};

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

        match parser.parse_str("!echo what a test") {
            Output::Command {
                arguments: _,
                name,
                prefix,
            } => {
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

        assert!(parser.parse_str("!👎").is_command());
    }

    #[test]
    fn test_unicode_prefix() {
        let mut parser = simple_config();
        parser.config_mut().add_prefix("👍"); // thumbs up unicode

        assert!(parser.parse_str("👍echo foo").is_command());
    }
}

use unicase::UniCase;

use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

/// Configuration for a [`Parser`].
///
/// [`Parser`]: struct.Parser.html
#[derive(Clone, Debug, Default)]
pub struct Config<'a> {
    commands: HashSet<CaseSensitivity>,
    prefixes: HashMap<Cow<'a, str>, Cow<'a, str>>,
}

impl<'a> Config<'a> {
    /// Creates a fresh default configuration with no commands or prefixes.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns an immutable reference to the commands.
    pub fn commands(&self) -> &HashSet<CaseSensitivity> {
        &self.commands
    }

    /// Returns a mutable reference to the commands.
    ///
    /// Use the [`add_command`] and [`remove_command`] methods for an easier way
    /// to manage commands.
    ///
    /// [`add_command`]: #method.add_command
    /// [`remove_command`]: #method.remove_command
    pub fn commands_mut(&mut self) -> &mut HashSet<CaseSensitivity> {
        &mut self.commands
    }

    /// Returns an immutable reference to the prefixes.
    ///
    /// Use the [`add_prefix`] and [`remove_prefix`] methods for an easier way
    /// to manage prefixes.
    ///
    /// [`add_prefix`]: #method.add_prefix
    /// [`remove_prefix`]: #method.remove_prefix
    pub fn prefixes(&self) -> &HashMap<Cow<'_, str>, Cow<'_, str>> {
        &self.prefixes
    }

    /// Returns a mutable reference to the prefixes.
    pub fn prefixes_mut(&mut self) -> &mut HashMap<Cow<'a, str>, Cow<'a, str>> {
        &mut self.prefixes
    }

    /// Adds a command to the list of commands.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dawn_command_parser::Config;
    ///
    /// let mut config = Config::new();
    /// config.add_command("ping");
    /// assert_eq!(1, config.commands().len());
    /// ```
    pub fn add_command(&mut self, command: impl Into<CaseSensitivity>) {
        self.commands.insert(command.into());
    }

    /// Removes a command from the list of commands.
    ///
    /// Any commands that would match the command provided are removed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dawn_command_parser::Config;
    ///
    /// let mut config = Config::new();
    /// config.add_command("ping");
    /// assert_eq!(1, config.commands().len());
    ///
    /// // Now remove it and verify that there are no commands.
    /// config.remove_command("ping");
    /// assert!(config.commands().is_empty());
    /// ```
    pub fn remove_command(&mut self, command: impl AsRef<str>) {
        self.commands.retain(|c| c != command.as_ref());
    }

    /// Adds a prefix to the list of prefixes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dawn_command_parser::Config;
    ///
    /// let mut config = Config::new();
    /// config.add_prefix("!");
    /// assert_eq!(1, config.prefixes().len());
    /// ```
    pub fn add_prefix(&mut self, prefix: impl Into<Cow<'a, str>>) {
        self.prefixes.insert(prefix.into(), Cow::Borrowed(""));
    }

    /// Removes a prefix from the list of prefixes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dawn_command_parser::Config;
    ///
    /// let mut config = Config::new();
    /// config.add_prefix("!");
    /// config.add_prefix("~");
    /// assert_eq!(2, config.prefixes().len());
    ///
    /// // Now remove one and verify that there is only 1 prefix.
    /// config.remove_prefix("!");
    /// assert_eq!(1, config.prefixes().len());
    /// ```
    pub fn remove_prefix(&mut self, prefix: impl Into<Cow<'a, str>>) -> Option<Cow<'a, str>> {
        self.prefixes.remove(&prefix.into())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum CaseSensitivity {
    Insensitive(UniCase<String>),
    Sensitive(String),
}

impl CaseSensitivity {
    pub fn new<S: Into<String>>(command: S) -> Self {
        Self::Insensitive(UniCase::new(command.into()))
    }
    pub fn case_sensitive(self, case_sensitive: bool) -> Self {
        match self {
            Self::Sensitive(s) if !case_sensitive => Self::Insensitive(UniCase::new(s)),
            Self::Insensitive(u) if case_sensitive => Self::Sensitive(u.into_inner()),
            other => other,
        }
    }
}

impl AsRef<str> for CaseSensitivity {
    fn as_ref(&self) -> &str {
        match self {
            Self::Insensitive(u) => u.as_str(),
            Self::Sensitive(s) => s.as_str(),
        }
    }
}

impl PartialEq<str> for CaseSensitivity {
    fn eq(&self, other: &str) -> bool {
        match self {
            Self::Insensitive(u) if u == &UniCase::new(other) => true,
            Self::Sensitive(s) if s.eq(other) => true,
            _ => false,
        }
    }
}

impl From<&str> for CaseSensitivity {
    fn from(string: &str) -> Self {
        Self::Insensitive(UniCase::new(string.into()))
    }
}

impl From<String> for CaseSensitivity {
    fn from(string: String) -> Self {
        Self::Insensitive(UniCase::new(string))
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn test_getters() {
        let mut config = Config::new();
        assert!(config.commands().is_empty());
        assert!(config.commands_mut().is_empty());
        assert!(config.prefixes().is_empty());
        assert!(config.prefixes_mut().is_empty());
    }
}

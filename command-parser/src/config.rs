use std::borrow::Cow;
use std::slice::{Iter, IterMut};

use crate::CaseSensitivity;

/// Configuration for a [`Parser`].
///
/// [`Parser`]: struct.Parser.html
#[derive(Clone, Debug, Default)]
pub struct CommandParserConfig<'a> {
    pub(crate) commands: Vec<CaseSensitivity>,
    pub(crate) prefixes: Vec<Cow<'a, str>>,
}

impl<'a> CommandParserConfig<'a> {
    /// Creates a fresh default configuration with no commands or prefixes.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns an iterator of immutable references to the commands.
    pub fn commands(&self) -> Commands<'_> {
        Commands {
            iter: self.commands.iter(),
        }
    }

    /// Returns an iterator of mutable references to the commands.
    ///
    /// Use the [`command`] and [`remove_command`] methods for an easier way to
    /// manage commands.
    ///
    /// [`command`]: #method.command
    /// [`remove_command`]: #method.remove_command
    pub fn commands_mut(&mut self) -> CommandsMut<'_> {
        CommandsMut {
            iter: self.commands.iter_mut(),
        }
    }

    /// Returns an iterator of immutable references to the prefixes.
    ///
    /// Use the [`add_prefix`] and [`remove_prefix`] methods for an easier way
    /// to manage prefixes.
    ///
    /// [`add_prefix`]: #method.add_prefix
    /// [`remove_prefix`]: #method.remove_prefix
    pub fn prefixes(&self) -> Prefixes<'_> {
        Prefixes {
            iter: self.prefixes.iter(),
        }
    }

    /// Returns an iterator of mutable references to the prefixes.
    pub fn prefixes_mut(&'a mut self) -> PrefixesMut<'a> {
        PrefixesMut {
            iter: self.prefixes.iter_mut(),
        }
    }

    /// Add a command to the list of commands.
    ///
    /// # Examples
    ///
    /// Add a case-sensitive "ping" command:
    ///
    /// ```rust
    /// use twilight_command_parser::CommandParserConfig;
    ///
    /// let mut config = CommandParserConfig::new();
    /// config.add_command("ping", true);
    /// assert_eq!(1, config.commands().len());
    /// ```
    ///
    /// [`CommandBuilder`]: struct.CommandBuilder.html
    pub fn add_command(&mut self, name: impl Into<String>, case_sensitive: bool) -> bool {
        self._add_command(name.into(), case_sensitive)
    }

    fn _add_command(&mut self, name: String, case_sensitive: bool) -> bool {
        let command = if case_sensitive {
            CaseSensitivity::Sensitive(name)
        } else {
            CaseSensitivity::Insensitive(name.into())
        };
        if self.commands.contains(&command) {
            false
        } else {
            self.commands.push(command);
            true
        }
    }

    /// Removes a command from the list of commands.
    ///
    /// Any commands that would match the command provided are removed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_command_parser::CommandParserConfig;
    ///
    /// let mut config = CommandParserConfig::new();
    /// config.add_command("ping", true);
    /// config.add_command("PING", false);
    /// assert_eq!(2, config.commands().len());
    ///
    /// // Now remove it and verify that there are no commands.
    /// config.remove_command("ping");
    /// assert_eq!(config.commands().len(), 0);
    /// ```
    pub fn remove_command(&mut self, command: impl AsRef<str>) {
        self.commands.retain(|c| c != command.as_ref());
    }

    /// Adds a prefix to the list of prefixes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_command_parser::CommandParserConfig;
    ///
    /// let mut config = CommandParserConfig::new();
    /// config.add_prefix("!");
    /// assert_eq!(1, config.prefixes().len());
    /// ```
    pub fn add_prefix(&mut self, prefix: impl Into<Cow<'a, str>>) -> bool {
        let prefix = prefix.into();
        if self.prefixes.contains(&prefix) {
            false
        } else {
            self.prefixes.push(prefix);
            true
        }
    }

    /// Removes a prefix from the list of prefixes.
    ///
    /// Returns whether a prefix with the name was removed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use twilight_command_parser::CommandParserConfig;
    ///
    /// let mut config = CommandParserConfig::new();
    /// config.add_prefix("!");
    /// config.add_prefix("~");
    /// assert_eq!(2, config.prefixes().len());
    ///
    /// // Now remove one and verify that there is only 1 prefix.
    /// config.remove_prefix("!");
    /// assert_eq!(1, config.prefixes().len());
    /// ```
    pub fn remove_prefix(&mut self, prefix: impl Into<Cow<'a, str>>) -> Option<Cow<'a, str>> {
        let needle = prefix.into();
        let pos = self.prefixes.iter().position(|e| *e == needle)?;
        Some(self.prefixes.remove(pos))
    }
}

pub struct Commands<'a> {
    iter: Iter<'a, CaseSensitivity>,
}

impl<'a> Iterator for Commands<'a> {
    type Item = &'a CaseSensitivity;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Commands<'a> {}

pub struct CommandsMut<'a> {
    iter: IterMut<'a, CaseSensitivity>,
}

impl<'a> Iterator for CommandsMut<'a> {
    type Item = &'a mut CaseSensitivity;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for CommandsMut<'a> {}

pub struct Prefixes<'a> {
    iter: Iter<'a, Cow<'a, str>>,
}

impl<'a> Iterator for Prefixes<'a> {
    type Item = &'a Cow<'a, str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Prefixes<'a> {}

pub struct PrefixesMut<'a> {
    iter: IterMut<'a, Cow<'a, str>>,
}

impl<'a> Iterator for PrefixesMut<'a> {
    type Item = &'a mut Cow<'a, str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for PrefixesMut<'a> {}

#[cfg(test)]
mod tests {
    use super::CommandParserConfig;

    #[test]
    fn test_getters() {
        let mut config = CommandParserConfig::new();
        assert!(config.commands().len() == 0);
        assert!(config.commands_mut().len() == 0);
        assert!(config.prefixes().len() == 0);
        assert!(config.prefixes_mut().len() == 0);
    }
}

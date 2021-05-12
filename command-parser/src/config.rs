//! Configuration for a [`Parser`].
//!
//! Provided are methods for [adding commands][`add_command`] and
//! [removing them][`remove_command`], as well as
//! [adding prefixes][`add_prefix`] and [removing prefixes][`remove_prefix`].
//! You can also [iterate over commands][`commands`] and [prefixes][`prefixes`].
//!
//! [`Parser`]: super::Parser
//! [`add_command`]: CommandParserConfig::add_command
//! [`add_prefix`]: CommandParserConfig::add_prefix
//! [`commands`]: CommandParserConfig::commands
//! [`prefixes`]: CommandParserConfig::prefixes
//! [`remove_command`]: CommandParserConfig::remove_command
//! [`remove_prefix`]: CommandParserConfig::remove_prefix

use super::casing::CaseSensitivity;
use std::borrow::Cow;
use std::slice::{Iter, IterMut};

/// Configuration for a [`Parser`].
///
/// [`Parser`]: crate::Parser
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
    /// Use the [`add_command`] and [`remove_command`] methods for an easier way to
    /// manage commands.
    ///
    /// [`add_command`]: Self::add_command
    /// [`remove_command`]: Self::remove_command
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
    /// [`add_prefix`]: Self::add_prefix
    /// [`remove_prefix`]: Self::remove_prefix
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

/// Iterator over the parser configuration's immutably borrowed commands.
pub struct Commands<'a> {
    iter: Iter<'a, CaseSensitivity>,
}

impl<'a> Iterator for Commands<'a> {
    type Item = (&'a str, bool);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|casing| (casing.as_ref(), casing.is_sensitive()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for Commands<'a> {}

/// Iterator over the parser configuration's mutably borrowed commands.
pub struct CommandsMut<'a> {
    iter: IterMut<'a, CaseSensitivity>,
}

impl<'a> Iterator for CommandsMut<'a> {
    type Item = (&'a mut str, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let casing = self.iter.next()?;
        let is_sensitive = casing.is_sensitive();

        Some((casing.as_mut(), is_sensitive))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<'a> ExactSizeIterator for CommandsMut<'a> {}

/// Iterator over the parser configuration's immutably borrowed prefixes.
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

/// Iterator over the parser configuration's mutably borrowed prefixes.
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
    use super::{CommandParserConfig, Commands, CommandsMut, Prefixes, PrefixesMut};
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(CommandParserConfig<'_>: Clone, Debug, Default, Send, Sync);
    assert_impl_all!(CommandsMut<'_>: ExactSizeIterator, Iterator, Send, Sync);
    assert_impl_all!(Commands<'_>: ExactSizeIterator, Iterator, Send, Sync);
    assert_impl_all!(PrefixesMut<'_>: ExactSizeIterator, Iterator, Send, Sync);
    assert_impl_all!(Prefixes<'_>: ExactSizeIterator, Iterator, Send, Sync);

    #[test]
    fn test_getters() {
        let mut config = CommandParserConfig::new();
        assert!(config.commands().len() == 0);
        assert!(config.commands_mut().len() == 0);
        assert!(config.prefixes().len() == 0);
        assert!(config.prefixes_mut().len() == 0);
    }
}

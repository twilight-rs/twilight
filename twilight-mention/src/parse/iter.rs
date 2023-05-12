use super::ParseMention;
use std::{iter::Iterator, marker::PhantomData, str::CharIndices};

/// Iterator of mentions within a buffer.
///
/// Unlike when parsing a mention directly via [`ParseMention::parse`],
/// incomplete mentions - such as when a "<" is found but not valid trailing
/// mention syntax - will not result in errors.
///
/// The iterator returns items consisting of 3 items: the mention itself
/// followed by the starting index and ending index of the mention's source in
/// the buffer.
///
/// # Examples
///
/// Iterate over all of the mentioned users:
///
/// ```
/// use twilight_mention::ParseMention;
/// use twilight_model::id::{marker::UserMarker, Id};
///
/// let buf = "<@123> some <@456> users <@789>!";
/// let mut iter = Id::<UserMarker>::iter(buf);
/// assert!(matches!(iter.next(), Some((user, _, _)) if user.get() == 123));
/// assert!(matches!(iter.next(), Some((user, _, _)) if user.get() == 456));
/// assert!(matches!(iter.next(), Some((user, _, _)) if user.get() == 789));
/// ```
#[derive(Clone, Debug)]
pub struct MentionIter<'a, T> {
    buf: &'a str,
    chars: CharIndices<'a>,
    phantom: PhantomData<T>,
}

impl<'a, T> MentionIter<'a, T> {
    #[must_use]
    pub(in crate::parse) fn new(buf: &'a str) -> Self {
        let chars = buf.char_indices();

        Self {
            buf,
            chars,
            phantom: PhantomData,
        }
    }

    /// Return an immutable reference to the underlying buffer of the iterator.
    #[must_use]
    pub const fn as_str(&self) -> &'a str {
        self.buf
    }
}

impl<'a, T: ParseMention> Iterator for MentionIter<'a, T> {
    /// Found mention followed by the start and ending indexes in the source
    /// string returned by [`as_str`].
    ///
    /// [`as_str`]: Self::as_str
    type Item = (T, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        // We want to take care of our input and make sure we're working with
        // chars here and not just individual bytes. We also want to not use
        // consuming methods of the iterator, so this will get a little weird.
        loop {
            let (start, '<') = self.chars.next()? else { continue };

            let mut found = false;

            for sigil in T::SIGILS {
                if self.chars.as_str().starts_with(sigil) {
                    found = true;

                    for _ in 0..sigil.chars().count() {
                        self.chars.next();
                    }
                }
            }

            if !found {
                continue;
            }

            let Some((end, _)) = self.chars.find(|c| c.1 == '>') else {
                continue;
            };

            let buf = self.buf.get(start..=end)?;

            if let Ok(id) = T::parse(buf) {
                return Some((id, start, end));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::timestamp::{Timestamp, TimestampStyle};

    use super::{
        super::{MentionType, ParseMention},
        MentionIter,
    };
    use static_assertions::{assert_impl_all, assert_obj_safe};
    use std::fmt::Debug;
    use twilight_model::id::{
        marker::{ChannelMarker, EmojiMarker, RoleMarker, UserMarker},
        Id,
    };

    assert_impl_all!(MentionIter<'_, Id<ChannelMarker>>: Clone, Debug, Iterator, Send, Sync);
    assert_impl_all!(MentionIter<'_, Id<EmojiMarker>>: Clone, Debug, Iterator, Send, Sync);
    assert_impl_all!(MentionIter<'_, MentionType>: Clone, Debug, Iterator, Send, Sync);
    assert_impl_all!(MentionIter<'_, Id<RoleMarker>>: Clone, Debug, Iterator, Send, Sync);
    assert_impl_all!(MentionIter<'_, Id<UserMarker>>: Clone, Debug, Iterator, Send, Sync);
    assert_obj_safe!(
        MentionIter<'_, Id<ChannelMarker>>,
        MentionIter<'_, Id<EmojiMarker>>,
        MentionIter<'_, MentionType>,
        MentionIter<'_, Id<RoleMarker>>,
        MentionIter<'_, Id<UserMarker>>,
    );

    #[test]
    fn iter_channel_id() {
        let mut iter = Id::<ChannelMarker>::iter("<#123>");
        assert_eq!(Id::new(123), iter.next().unwrap().0);
        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_multiple_ids() {
        let buf = "one <@123>two<#456><@789> ----";
        let mut iter = Id::<UserMarker>::iter(buf);
        assert_eq!(Id::new(123), iter.next().unwrap().0);
        let (mention, start, end) = iter.next().unwrap();
        assert_eq!(Id::new(789), mention);
        assert_eq!(19, start);
        assert_eq!(24, end);
        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_emoji_ids() {
        let mut iter = Id::<EmojiMarker>::iter("some <:name:123> emojis <:emoji:456>");
        assert_eq!(Id::new(123), iter.next().unwrap().0);
        assert_eq!(Id::new(456), iter.next().unwrap().0);
        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_mention_type() {
        let mut iter = MentionType::iter("<#12><:name:34><@&56><@78>");
        assert_eq!(MentionType::Channel(Id::new(12)), iter.next().unwrap().0);
        assert_eq!(MentionType::Emoji(Id::new(34)), iter.next().unwrap().0);
        assert_eq!(MentionType::Role(Id::new(56)), iter.next().unwrap().0);
        assert_eq!(MentionType::User(Id::new(78)), iter.next().unwrap().0);
        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_mention_type_with_timestamp() {
        let mut iter = MentionType::iter("<#12> <t:34> <t:56:d>");
        assert_eq!(MentionType::Channel(Id::new(12)), iter.next().unwrap().0);
        assert_eq!(
            MentionType::Timestamp(Timestamp::new(34, None)),
            iter.next().unwrap().0
        );
        assert_eq!(
            MentionType::Timestamp(Timestamp::new(56, Some(TimestampStyle::ShortDate))),
            iter.next().unwrap().0
        );
        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_role_ids() {
        let mut iter = Id::<RoleMarker>::iter("some <@&123> roles <@&456>");
        assert_eq!(Id::new(123), iter.next().unwrap().0);
        assert_eq!(Id::new(456), iter.next().unwrap().0);
        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_timestamps() {
        let mut iter = Timestamp::iter("some <t:123> roles <t:456:t>");
        assert_eq!(Timestamp::new(123, None), iter.next().unwrap().0);
        assert_eq!(
            Timestamp::new(456, Some(TimestampStyle::ShortTime)),
            iter.next().unwrap().0
        );
        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_user_ids() {
        let mut iter = Id::<UserMarker>::iter("some <@123>users<@456>");
        assert_eq!(Id::new(123), iter.next().unwrap().0);
        assert_eq!(Id::new(456), iter.next().unwrap().0);
        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_no_id() {
        let mention = "this is not <# actually a mention";
        let mut iter = Id::<ChannelMarker>::iter(mention);

        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_ignores_other_types() {
        let mention = "<#123> <:name:456> <@&789>";
        let mut iter = Id::<UserMarker>::iter(mention);

        assert!(iter.next().is_none());
    }

    #[test]
    fn iter_as_str() {
        let buf = "a buf";
        let mut iter = Id::<RoleMarker>::iter(buf);
        assert_eq!(buf, iter.as_str());
        // Advancing still returns the complete buffer.
        assert!(iter.next().is_none());
        assert_eq!(buf, iter.as_str());
    }
}

//! Active gateway session details.

use serde::{Deserialize, Serialize};
use std::mem;

/// Gateway session information for a shard's active connection.
///
/// A session is a stateful identifier on Discord's end for running a [shard].
/// It is used for maintaining an authenticated Websocket connection based on
/// an [identifier]. While a session is only connected to one shard, one shard
/// can have more than one session: if a shard shuts down its connection and
/// starts a new session, then the previous session will be kept alive for a
/// short time.
///
/// # Reusing sessions
///
/// Sessions are able to be reused across connections to Discord. If an
/// application's process needs to be restarted, then this session
/// information—which can be (de)serialized via serde—can be stored, the
/// application restarted, and then used again via [`ConfigBuilder::session`].
///
/// If the delay between disconnecting from the gateway and reconnecting isn't
/// too long and Discord hasn't invalidated the session, then the session will
/// be reused by Discord. As a result, any events that were "missed" while
/// restarting and reconnecting will be played back, meaning the application
/// won't have missed any events. If the delay has been too long, then a new
/// session will be initialized, resulting in those events being missed.
///
/// [`ConfigBuilder::session`]: crate::ConfigBuilder::session
/// [identifier]: Self::id
/// [shard]: crate::Shard
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Session {
    /// ID of the gateway session.
    id: Box<str>,
    /// Sequence of the most recently received gateway event.
    ///
    /// The first sequence of a session is always 1.
    sequence: u64,
}

impl Session {
    /// Create new configuration for resuming a gateway session.
    ///
    /// Can be provided to [`ConfigBuilder::session`].
    ///
    /// [`ConfigBuilder::session`]: crate::ConfigBuilder::session
    pub fn new(sequence: u64, session_id: String) -> Self {
        Self {
            sequence,
            id: session_id.into_boxed_str(),
        }
    }

    /// ID of the session being resumed.
    ///
    /// The ID of the session is different from the [ID of the shard]; shards are
    /// identified by an index, and when authenticated with the gateway the shard
    /// is given a unique identifier for the gateway session.
    ///
    /// Session IDs are obtained by shards via sending an [`Identify`] command
    /// with the shard's authentication details, and in return the session ID is
    /// provided via the [`Ready`] event.
    ///
    /// [`Identify`]: twilight_model::gateway::payload::outgoing::Identify
    /// [`Ready`]: twilight_model::gateway::payload::incoming::Ready
    /// [ID of the shard]: crate::ShardId
    pub const fn id(&self) -> &str {
        &self.id
    }

    /// Current sequence of the connection.
    ///
    /// Number of the events that have been received during this session. A
    /// larger number typically correlates that the shard has been connected
    /// with this session for a longer time, while a smaller number typically
    /// correlates to meaning that it's been connected with this session for a
    /// shorter duration of time.
    ///
    /// As a shard is connected to the gateway and receives events this sequence
    /// will be updated in real time when obtaining the [session of a shard].
    ///
    /// [session of a shard]: crate::Shard::session
    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Set the sequence, returning the previous sequence.
    pub(crate) fn set_sequence(&mut self, sequence: u64) -> u64 {
        mem::replace(&mut self.sequence, sequence)
    }
}

#[cfg(test)]
mod tests {
    use super::Session;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::fmt::Debug;

    assert_impl_all!(
        Session: Clone,
        Debug,
        Deserialize<'static>,
        Eq,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    /// Test that sessions deserialize and serialize the same way.
    #[test]
    fn serde() {
        const SEQUENCE: u64 = 56_132;
        const SESSION_ID: &str = "thisisanid";

        let value = Session::new(SEQUENCE, SESSION_ID.to_owned());

        serde_test::assert_tokens(
            &value,
            &[
                Token::Struct {
                    name: "Session",
                    len: 2,
                },
                Token::Str("id"),
                Token::Str(SESSION_ID),
                Token::Str("sequence"),
                Token::U64(SEQUENCE),
                Token::StructEnd,
            ],
        );
    }

    /// Test that session getters return the provided values.
    #[test]
    fn session() {
        const SESSIONS: [(u64, &str); 2] = [(1, "a"), (2, "b")];

        for (sequence, session_id) in SESSIONS {
            let session = Session::new(sequence, session_id.to_owned());
            assert_eq!(session.sequence(), sequence);
            assert_eq!(session.id(), session_id);
        }
    }

    /// Test that setting the sequence actually updates the sequence and returns
    /// the previous sequence.
    #[test]
    fn set_sequence() {
        const SEQUENCE_INITIAL: u64 = 1;
        const SEQUENCE_NEXT: u64 = SEQUENCE_INITIAL + 1;
        const SEQUENCE_SKIPPED: u64 = SEQUENCE_NEXT + 3;

        let mut session = Session::new(SEQUENCE_INITIAL, String::new());
        let old = session.set_sequence(SEQUENCE_NEXT);
        assert_eq!(old, SEQUENCE_INITIAL);

        // although we don't expect to skip sequences the setter should still
        // handle them as usual
        let skipped_old = session.set_sequence(SEQUENCE_SKIPPED);
        assert_eq!(skipped_old, SEQUENCE_NEXT);
    }
}

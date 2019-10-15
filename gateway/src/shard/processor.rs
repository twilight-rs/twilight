use super::{
    config::Config,
    connect,
    error::{Error, Result},
    event::Event,
    session::Session,
    socket_forwarder::SocketForwarder,
    stage::Stage,
};
use crate::{
    event::{DispatchEvent, GatewayEvent},
    listener::Listeners,
};
use dawn_model::gateway::payload::{
    identify::{Identify, IdentifyInfo, IdentifyProperties},
    resume::Resume,
};
use flate2::Decompress;
use futures_channel::mpsc::UnboundedReceiver;
use futures_util::stream::StreamExt;
use log::{debug, trace, warn};
use serde::Serialize;
use std::{env::consts::OS, mem, ops::Deref, sync::Arc};
use tokio_tungstenite::tungstenite::Message;

use std::error::Error as StdError;

/// Runs in the background and processes incoming events, and then broadcasts
/// to all listeners.
pub struct ShardProcessor {
    pub config: Arc<Config>,
    pub listeners: Arc<Listeners<Event>>,
    pub properties: IdentifyProperties,
    pub rx: UnboundedReceiver<Message>,
    pub session: Arc<Session>,
    inflater: Decompress,
    event_buffer: Vec<u8>,
    msg_buffer: Vec<u8>,
}

impl ShardProcessor {
    pub async fn new(config: Arc<Config>) -> Result<Self> {
        let properties = IdentifyProperties::new("dawn.rs", "dawn.rs", OS, "", "");

        let url = "wss://gateway.discord.gg?compress=zlib-stream";

        let stream = connect::connect(url).await?;
        let (mut forwarder, rx, tx) = SocketForwarder::new(stream);
        tokio_executor::spawn(async move {
            forwarder.run().await;
        });

        Ok(Self {
            config,
            listeners: Arc::new(Listeners::default()),
            properties,
            rx,
            session: Arc::new(Session::new(tx)),
            inflater: Decompress::new(true),
            event_buffer: Vec::new(),
            // Allocates around 1GB, but is in pratice not allocated,
            // as most of it is never written to so it will not allocate
            // all of it on systems with lazy allocation, like for example
            // GNU/Linux.
            msg_buffer: Vec::with_capacity(2_usize.pow(30)),
        })
    }

    pub async fn run(mut self) {
        let mut remove_listeners = Vec::new();

        loop {
            let gateway_event = match self.next_event().await {
                Ok(ev) => ev,
                // Reconnect as this error is often fatal!
                Err(crate::shard::Error::Decompressing {
                    ..
                }) => {
                    warn!("[gateway] Decompressing error, clears buffers and reconnect!");

                    // Clear buffers
                    self.event_buffer.clear();
                    self.msg_buffer.clear();

                    // Reset inflater context
                    self.inflater.reset(true);

                    self.reconnect().await;
                    continue;
                },
                Err(err) => {
                    warn!("Decompressing failed: {:?}", err.source());
                    continue;
                },
            };

            // The only reason for an error is if the sender couldn't send a
            // message or if the session didn't exist when it should, so do a
            // reconnect if this fails.
            if self.process(&gateway_event).await.is_err() {
                debug!("Error processing event; reconnecting");

                self.reconnect().await;

                continue;
            }

            let event = Event::from(gateway_event);

            let mut listeners = self.listeners.listeners.lock().await;

            for (id, listener) in listeners.iter() {
                let event_type = event.event_type();

                if !listener.events.contains(event_type) {
                    trace!(
                        "[ShardProcessor] Listener {} doesn't want event type {:?}",
                        id,
                        event_type,
                    );

                    continue;
                }

                // Since this is unbounded, this is always because the receiver
                // dropped.
                if listener.tx.unbounded_send(event.clone()).is_err() {
                    remove_listeners.push(*id);
                }
            }

            for id in &remove_listeners {
                debug!("[ShardProcessor] Removing listener {}", id);

                listeners.remove(id);
            }

            remove_listeners.clear();
        }
    }

    /// Identifies with the gateway to create a new session.
    async fn identify(&mut self) -> Result<()> {
        self.session.set_stage(Stage::Identifying);

        let identify = Identify::new(IdentifyInfo {
            compression: false,
            guild_subscriptions: true,
            large_threshold: 250,
            properties: self.properties.clone(),
            shard: Some(self.config.shard()),
            presence: None,
            token: self.config.token().to_owned(),
            v: 6,
        });

        self.send(identify).await
    }

    async fn process(&mut self, event: &GatewayEvent) -> Result<()> {
        use GatewayEvent::*;

        match event {
            Dispatch(seq, dispatch) => {
                self.session.set_seq(*seq);

                match dispatch.deref() {
                    DispatchEvent::Ready(ready) => {
                        self.session.set_stage(Stage::Connected);
                        self.session.set_id(&ready.session_id).await;
                    },
                    DispatchEvent::Resumed => {
                        self.session.set_stage(Stage::Connected);
                        self.session.heartbeats.receive().await;
                    },
                    _ => {},
                }
            },
            Heartbeat(seq) => {
                if *seq > self.session.seq() + 1 {
                    self.resume().await?;
                }

                if self.session.heartbeat().is_err() {
                    warn!("Error sending heartbeat; reconnecting");

                    self.reconnect().await;
                }
            },
            Hello(interval) => {
                self.session.set_stage(Stage::Identifying);

                if *interval > 0 {
                    self.session.set_heartbeat_interval(*interval);
                    self.session.start_heartbeater().await;
                }

                self.identify().await?;
            },
            HeartbeatAck => {
                self.session.heartbeats.receive().await;
            },
            InvalidateSession(true) => {
                self.resume().await?;
            },
            InvalidateSession(false) => {
                self.reconnect().await;
            },
            Reconnect => {
                self.reconnect().await;
            },
        }

        Ok(())
    }

    async fn reconnect(&mut self) {
        loop {
            self.config.queue.request().await;

            let shard = match Self::new(Arc::clone(&self.config.clone())).await {
                Ok(shard) => shard,
                Err(why) => {
                    warn!("Error reconnecting: {:?}", why);

                    continue;
                },
            };

            mem::replace(self, shard);
        }
    }

    async fn resume(&mut self) -> Result<()> {
        self.session.set_stage(Stage::Resuming);

        let id = if let Some(id) = self.session.id().await {
            id
        } else {
            self.reconnect().await;

            return Ok(());
        };

        let payload = Resume::new(self.session.seq(), id, self.config.token());

        self.send(payload).await?;

        Ok(())
    }

    pub async fn send(&mut self, payload: impl Serialize) -> Result<()> {
        match self.session.send(payload) {
            Ok(()) => Ok(()),
            Err(Error::PayloadSerialization {
                source,
            }) => {
                log::warn!("Failed to serialize message to send: {:?}", source);

                Err(Error::PayloadSerialization {
                    source,
                })
            },
            Err(Error::SendingMessage {
                source,
            }) => {
                log::warn!("Failed to send message: {:?}", source);
                log::info!("Reconnecting");

                self.reconnect().await;

                Ok(())
            },
            Err(other) => Err(other),
        }
    }

    async fn next_event(&mut self) -> GatewayEvent {
        const ZLIB_SUFFIX: [u8; 4] = [0x00, 0x00, 0xff, 0xff];

        loop {
            // Returns None when the socket forwarder has ended, meaning the
            // connection was dropped.
            let bin = if let Some(msg) = self.rx.next().await {
                if let tokio_tungstenite::tungstenite::Message::Binary(bin) = msg {
                    bin
                } else {
                    panic!("Ehmmm this should not happen!");
                }
            } else {
                self.reconnect().await;

                continue;
            };

            // Extend new message to event buffer.
            self.event_buffer.extend_from_slice(&bin[..]);
            
            let length = self.event_buffer.len();
            if length >= 4 {
                if self.event_buffer[(length - 4)..] == ZLIB_SUFFIX {
                    let event = loop {
                        self.inflater
                            .decompress_vec(
                                &self.event_buffer,
                                &mut self.msg_buffer,
                                flate2::FlushDecompress::Sync,
                            )
                            .map_err(|source| crate::shard::Error::Decompressing {
                                source,
                            })?;

                        match serde_json::from_slice(&self.msg_buffer.clone()) {
                            Ok(ev) => break ev,
                            Err(err) => {
                                trace!("error: {:?}", err);
                                // Is maybe not enought to catch all errors but that is
                                // currently not a problem as we just make sure to have
                                // large enough buffer at all time.
                                if err.is_eof() {
                                    let cap = self.msg_buffer.capacity();
                                    trace!(
                                        "msg_buffer: {}",
                                        if let Ok(s) = String::from_utf8(self.msg_buffer.clone()) {
                                            s
                                        } else {
                                            String::from("Invalid string!")
                                        }
                                    );
                                    &self.msg_buffer.reserve(cap);
                                    trace!("Buffer resized to: {}", self.msg_buffer.capacity());

                                    continue;
                                } else {
                                    trace!(
                                        "msg_buffer: {}",
                                        if let Ok(s) = String::from_utf8(self.msg_buffer.clone()) {
                                            s
                                        } else {
                                            String::from("Invalid string!")
                                        }
                                    );
                                    return Err(crate::shard::Error::PayloadSerialization {
                                        source: err,
                                    });
                                }
                            },
                        };
                    };
                    trace!(
                        "in:out: {}:{}",
                        self.event_buffer.len(),
                        self.msg_buffer.len()
                    );
                    self.event_buffer.clear();
                    self.msg_buffer.clear();
                    break Ok(event);
                }
            }
        }
    }
}

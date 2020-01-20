use super::{
    super::{
        config::Config,
        error::{Error, Result},
        event::Event,
        stage::Stage,
    },
    connect,
    emit,
    inflater::Inflater,
    session::Session,
    socket_forwarder::SocketForwarder,
};

use crate::{
    event::{DispatchEvent, GatewayEvent},
    listener::Listeners,
};
use dawn_model::gateway::payload::{
    identify::{Identify, IdentifyInfo, IdentifyProperties},
    resume::Resume,
};
use futures::{channel::mpsc::UnboundedReceiver, stream::StreamExt};
#[allow(unused_imports)]
use log::{debug, info, trace, warn};
use serde::Serialize;
use std::{env::consts::OS, ops::Deref, sync::Arc};
use tokio_tungstenite::tungstenite::Message;

#[cfg(feature = "metrics")]
use metrics::counter;

use std::error::Error as StdError;

/// Runs in the background and processes incoming events, and then broadcasts
/// to all listeners.
pub struct ShardProcessor {
    pub config: Arc<Config>,
    pub listeners: Listeners<Event>,
    pub properties: IdentifyProperties,
    pub rx: UnboundedReceiver<Message>,
    pub session: Arc<Session>,
    inflater: Inflater,
    url: String,
}

impl ShardProcessor {
    pub async fn new(config: Arc<Config>) -> Result<Self> {
        debug!("[ShardProcessor {:?}] Queueing", config.shard());
        config.queue.request().await;
        debug!("[ShardProcessor {:?}] Finished queue", config.shard());

        let properties = IdentifyProperties::new("dawn.rs", "dawn.rs", OS, "", "");

        let mut url = config
            .http_client()
            .gateway()
            .await
            .map_err(|source| Error::GettingGatewayUrl {
                source,
            })?
            .url;
        url.push_str("?v=6&compress=zlib-stream");

        let stream = connect::connect(&url).await?;
        let (forwarder, rx, tx) = SocketForwarder::new(stream);
        tokio::spawn(async move {
            forwarder.run().await;
        });

        let shard = config.shard();

        Ok(Self {
            config,
            listeners: Listeners::default(),
            properties,
            rx,
            session: Arc::new(Session::new(tx)),
            inflater: Inflater::new(shard),
            url,
        })
    }

    pub async fn run(mut self) {
        loop {
            let gateway_event = match self.next_event().await {
                Ok(ev) => ev,
                // Reconnect as this error is often fatal!
                Err(Error::Decompressing {
                    source,
                }) => {
                    warn!(
                        "[gateway] Decompressing error, clears buffers and reconnect! {:?}",
                        source
                    );

                    // Inflater gets reset in the reconnect call.
                    self.reconnect().await;
                    continue;
                },
                Err(err) => {
                    warn!("Error receiveing gateway event: {:?}", err.source());
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

            emit::event(self.listeners.clone(), Event::from(gateway_event));
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
                #[cfg(feature = "metrics")]
                counter!("GatewayEvent", 1, "GatewayEvent" => "Dispatch");
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
                #[cfg(feature = "metrics")]
                counter!("GatewayEvent", 1, "GatewayEvent" => "Heartbeat");
                if *seq > self.session.seq() + 1 {
                    self.resume().await?;
                }

                if let Err(err) = self.session.heartbeat() {
                    warn!("Error sending heartbeat; reconnecting: {}", err);

                    self.reconnect().await;
                }
            },
            Hello(interval) => {
                #[cfg(feature = "metrics")]
                counter!("GatewayEvent", 1, "GatewayEvent" => "Hello");
                debug!("[EVENT] Hello({})", interval);
                self.session.set_stage(Stage::Identifying);

                if *interval > 0 {
                    self.session.set_heartbeat_interval(*interval);
                    self.session.start_heartbeater().await;
                }

                self.identify().await?;
            },
            HeartbeatAck => {
                #[cfg(feature = "metrics")]
                counter!("GatewayEvent", 1, "GatewayEvent" => "HeartbeatAck");
                self.session.heartbeats.receive().await;
            },
            InvalidateSession(true) => {
                #[cfg(feature = "metrics")]
                counter!("GatewayEvent", 1, "GatewayEvent" => "InvalidateSessionTrue");
                debug!("[EVENT] InvalidateSession(true)");
                self.resume().await?;
            },
            InvalidateSession(false) => {
                #[cfg(feature = "metrics")]
                counter!("GatewayEvent", 1, "GatewayEvent" => "InvalidateSessionFalse");
                debug!("[EVENT] InvalidateSession(false)");
                self.reconnect().await;
            },
            Reconnect => {
                #[cfg(feature = "metrics")]
                counter!("GatewayEvent", 1, "GatewayEvent" => "Reconnect");
                debug!("[EVENT] Reconnect");
                self.reconnect().await;
            },
        }

        Ok(())
    }

    async fn reconnect(&mut self) {
        warn!("[reconnect] Reconnection started!");
        loop {
            self.config.queue.request().await;

            let new_stream = match connect::connect(&self.url).await {
                Ok(s) => s,
                Err(why) => {
                    warn!("Error reconnecting: {:?}", why);
                    continue;
                },
            };

            let (new_forwarder, new_rx, new_tx) = SocketForwarder::new(new_stream);
            tokio::spawn(async move {
                new_forwarder.run().await;
            });

            self.rx = new_rx;
            self.session = Arc::new(Session::new(new_tx));
            self.inflater.reset();

            break;
        }
    }

    async fn resume(&mut self) -> Result<()> {
        warn!("[resume] Resume started!");
        self.session.set_stage(Stage::Resuming);

        let id = if let Some(id) = self.session.id().await {
            id
        } else {
            self.reconnect().await;

            return Ok(());
        };
        self.inflater.reset();
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

    async fn next_event(&mut self) -> Result<GatewayEvent> {
        loop {
            // Returns None when the socket forwarder has ended, meaning the
            // connection was dropped.
            let msg = if let Some(msg) = self.rx.next().await {
                msg
            } else {
                if let Err(why) = self.resume().await {
                    warn!("Resume failed with {}, reconnecting", why);
                    self.reconnect().await;
                }
                continue;
            };

            match msg {
                Message::Binary(bin) => {
                    self.inflater.extend(&bin[..]);
                    let decompressed_msg =
                        self.inflater.msg().map_err(|source| Error::Decompressing {
                            source,
                        })?;
                    let msg_or_error = match decompressed_msg {
                        Some(json) => {
                            emit::bytes(self.listeners.clone(), json).await;

                            match serde_json::from_slice(json).map_err(|source| {
                                Error::PayloadSerialization {
                                    source,
                                }
                            }) {
                                Ok(ser) => Ok(ser),
                                Err(err) => {
                                    debug!("Broken JSON: {:?}", std::str::from_utf8(json));
                                    Err(err)
                                },
                            }
                        },
                        None => continue,
                    };
                    self.inflater.clear();
                    break msg_or_error;
                },
                Message::Close(_) => self.reconnect().await,
                Message::Ping(_) | Message::Pong(_) => {},
                Message::Text(text) => {
                    trace!("Text payload: {}", text);
                    emit::bytes(self.listeners.clone(), text.as_bytes()).await;

                    break match serde_json::from_str(&text).map_err(|source| {
                        Error::PayloadSerialization {
                            source,
                        }
                    }) {
                        Ok(ser) => Ok(ser),
                        Err(err) => {
                            debug!("Broken JSON: {:?}", &text);
                            Err(err)
                        },
                    };
                },
            }
        }
    }
}

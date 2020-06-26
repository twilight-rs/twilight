use super::{
    super::{
        config::ShardConfig,
        error::{Error, Result},
        stage::Stage,
    },
    connect, emit,
    inflater::Inflater,
    session::Session,
    socket_forwarder::SocketForwarder,
};

use crate::listener::Listeners;
use twilight_model::gateway::{
    event::{
        shard::{Connected, Connecting, Disconnected, Identifying, Reconnecting, Resuming},
        DispatchEvent, Event, GatewayEvent,
    },
    payload::{
        identify::{Identify, IdentifyInfo, IdentifyProperties},
        resume::Resume,
    },
};

use futures_channel::mpsc::UnboundedReceiver;
use futures_util::stream::StreamExt;
#[allow(unused_imports)]
use log::{debug, info, trace, warn};
use serde::Serialize;
use std::{env::consts::OS, ops::Deref, sync::Arc};
use tokio::sync::watch::{
    channel as watch_channel, Receiver as WatchReceiver, Sender as WatchSender,
};
use tokio_tungstenite::tungstenite::{
    protocol::{frame::coding::CloseCode, CloseFrame},
    Message,
};

use std::borrow::Cow;

#[cfg(feature = "metrics")]
use metrics::counter;

use std::error::Error as StdError;
use std::sync::atomic::Ordering;
use twilight_model::gateway::payload::update_status::UpdateStatusInfo;

/// Runs in the background and processes incoming events, and then broadcasts
/// to all listeners.
#[derive(Debug)]
pub struct ShardProcessor {
    pub config: Arc<ShardConfig>,
    pub listeners: Listeners<Event>,
    pub properties: IdentifyProperties,
    pub rx: UnboundedReceiver<Message>,
    pub session: Arc<Session>,
    inflater: Inflater,
    url: String,
    resume: Option<(u64, String)>,
    wtx: WatchSender<Arc<Session>>,
}

impl ShardProcessor {
    pub async fn new(
        config: Arc<ShardConfig>,
        mut url: String,
        listeners: Listeners<Event>,
    ) -> Result<(Self, WatchReceiver<Arc<Session>>)> {
        //if we got resume info we don't need to wait
        let shard_id = config.shard();
        let resumable = config.sequence.is_some() && config.session_id.is_some();
        if !resumable {
            debug!("Shard {:?} is not resumable", shard_id);
            debug!("[ShardProcessor {:?}] Queueing", shard_id);
            config.queue.request(shard_id).await;
            debug!("[ShardProcessor {:?}] Finished queue", config.shard());
        }

        let properties = IdentifyProperties::new("twilight.rs", "twilight.rs", OS, "", "");

        url.push_str("?v=6&compress=zlib-stream");

        emit::event(
            listeners.clone(),
            Event::ShardConnecting(Connecting {
                gateway: url.clone(),
                shard_id: config.shard()[0],
            }),
        );
        let stream = connect::connect(&url).await?;
        let (forwarder, rx, tx) = SocketForwarder::new(stream);
        tokio::spawn(async move {
            forwarder.run().await;
        });

        let session = Arc::new(Session::new(tx));
        if resumable {
            session
                .id
                .lock()
                .await
                .replace(config.session_id.clone().unwrap());
            session
                .seq
                .store(config.sequence.unwrap(), Ordering::Relaxed)
        }

        let (wtx, wrx) = watch_channel(Arc::clone(&session));

        let mut processor = Self {
            config,
            listeners,
            properties,
            rx,
            session,
            inflater: Inflater::new(shard_id),
            url,
            resume: None,
            wtx,
        };

        if resumable {
            debug!("Shard {:?} resuming", shard_id);
            processor.resume().await?;
        }

        Ok((processor, wrx))
    }

    pub async fn run(mut self) {
        loop {
            let gateway_event = match self.next_event().await {
                Ok(ev) => ev,
                // The authorization is invalid, so we should just quit.
                Err(Error::AuthorizationInvalid { shard_id, .. }) => {
                    warn!(
                        "The authorization for shard {} is invalid, quitting",
                        shard_id
                    );
                    self.listeners.remove_all().await;

                    return;
                }
                // Reconnect as this error is often fatal!
                Err(Error::Decompressing { source }) => {
                    warn!(
                        "[gateway] Decompressing error, clears buffers and reconnect! {:?}",
                        source
                    );

                    // Inflater gets reset in the reconnect call.
                    self.reconnect(true).await;
                    continue;
                }
                Err(Error::IntentsDisallowed { shard_id, .. }) => {
                    warn!(
                        "At least one of the provided intents for shard {} are disallowed",
                        shard_id
                    );
                    self.listeners.remove_all().await;
                    return;
                }
                Err(Error::IntentsInvalid { shard_id, .. }) => {
                    warn!(
                        "At least one of the provided intents for shard {} are invalid",
                        shard_id
                    );
                    self.listeners.remove_all().await;
                    return;
                }
                Err(err) => {
                    warn!("Error receiveing gateway event: {:?}", err.source());
                    continue;
                }
            };

            // The only reason for an error is if the sender couldn't send a
            // message or if the session didn't exist when it should, so do a
            // reconnect if this fails.
            if self.process(&gateway_event).await.is_err() {
                debug!("Error processing event; reconnecting");

                self.reconnect(true).await;

                continue;
            }

            emit::event(self.listeners.clone(), Event::from(gateway_event));
        }
    }

    /// Identifies with the gateway to create a new session.
    async fn identify(&mut self) -> Result<()> {
        self.session.set_stage(Stage::Identifying);

        let intents = self.config.intents().copied();

        let identify = Identify::new(IdentifyInfo {
            compression: false,
            guild_subscriptions: true,
            intents,
            large_threshold: 250,
            properties: self.properties.clone(),
            shard: Some(self.config.shard()),
            presence: self.config.presence().cloned(),
            token: self.config.token().to_owned(),
            v: 6,
        });
        emit::event(
            self.listeners.clone(),
            Event::ShardIdentifying(Identifying {
                shard_id: self.config.shard()[0],
                shard_total: self.config.shard()[1],
            }),
        );

        self.send(identify).await
    }

    async fn process(&mut self, event: &GatewayEvent) -> Result<()> {
        use GatewayEvent::{
            Dispatch, Heartbeat, HeartbeatAck, Hello, InvalidateSession, Reconnect,
        };

        match event {
            Dispatch(seq, dispatch) => {
                #[cfg(feature = "metrics")]
                counter!("GatewayEvent", 1, "GatewayEvent" => "Dispatch");
                self.session.set_seq(*seq);

                // this lint is wrong and generates invalid code
                #[allow(clippy::explicit_deref_methods)]
                match dispatch.deref() {
                    DispatchEvent::Ready(ready) => {
                        self.session.set_stage(Stage::Connected);
                        self.session.set_id(&ready.session_id).await;

                        emit::event(
                            self.listeners.clone(),
                            Event::ShardConnected(Connected {
                                heartbeat_interval: self.session.heartbeat_interval(),
                                shard_id: self.config.shard()[0],
                            }),
                        );
                    }
                    DispatchEvent::Resumed => {
                        self.session.set_stage(Stage::Connected);
                        emit::event(
                            self.listeners.clone(),
                            Event::ShardConnected(Connected {
                                heartbeat_interval: self.session.heartbeat_interval(),
                                shard_id: self.config.shard()[0],
                            }),
                        );
                        self.session.heartbeats.receive().await;
                    }
                    _ => {}
                }
            }
            Heartbeat(seq) => {
                #[cfg(feature = "metrics")]
                counter!("GatewayEvent", 1, "GatewayEvent" => "Heartbeat");
                if *seq > self.session.seq() + 1 {
                    self.resume().await?;
                }

                if let Err(err) = self.session.heartbeat() {
                    warn!("Error sending heartbeat; reconnecting: {}", err);

                    self.reconnect(true).await;
                }
            }
            Hello(interval) => {
                #[cfg(feature = "metrics")]
                counter!("GatewayEvent", 1, "GatewayEvent" => "Hello");
                debug!("[EVENT] Hello({})", interval);

                if self.session.stage() == Stage::Resuming && self.resume.is_some() {
                    // Safe to unwrap so here as we have just checked that
                    // it is some.
                    let (seq, id) = self.resume.take().unwrap();
                    warn!("Resuming with ({}, {})!", seq, id);
                    let payload = Resume::new(seq, &id, self.config.token());

                    // Set id so it is correct for next resume.
                    self.session.set_id(id).await;

                    if *interval > 0 {
                        self.session.set_heartbeat_interval(*interval);
                        self.session.start_heartbeater().await;
                    }

                    self.send(payload).await?;
                } else {
                    self.session.set_stage(Stage::Identifying);

                    if *interval > 0 {
                        self.session.set_heartbeat_interval(*interval);
                        self.session.start_heartbeater().await;
                    }

                    self.identify().await?;
                }
            }
            HeartbeatAck => {
                #[cfg(feature = "metrics")]
                counter!("GatewayEvent", 1, "GatewayEvent" => "HeartbeatAck");
                self.session.heartbeats.receive().await;
            }
            InvalidateSession(true) => {
                #[cfg(feature = "metrics")]
                counter!("GatewayEvent", 1, "GatewayEvent" => "InvalidateSessionTrue");
                debug!("[EVENT] InvalidateSession(true)");
                self.resume().await?;
            }
            InvalidateSession(false) => {
                #[cfg(feature = "metrics")]
                counter!("GatewayEvent", 1, "GatewayEvent" => "InvalidateSessionFalse");
                debug!("[EVENT] InvalidateSession(false)");
                self.reconnect(true).await;
            }
            Reconnect => {
                #[cfg(feature = "metrics")]
                counter!("GatewayEvent", 1, "GatewayEvent" => "Reconnect");
                debug!("[EVENT] Reconnect");
                let frame = CloseFrame {
                    code: CloseCode::Restart,
                    reason: Cow::Borrowed("Reconnecting"),
                };
                self.close(Some(frame)).await?;
                self.resume().await?;
            }
        }

        Ok(())
    }

    async fn reconnect(&mut self, full_reconnect: bool) {
        info!("[reconnect] Reconnection started!");
        loop {
            // Await allowance if doing a full reconnect
            if full_reconnect {
                let shard = self.config.shard();
                self.config.queue.request(shard).await;
            }

            if full_reconnect {
                emit::event(
                    self.listeners.clone(),
                    Event::ShardReconnecting(Reconnecting {
                        shard_id: self.config.shard()[0],
                    }),
                );
            } else {
                emit::event(
                    self.listeners.clone(),
                    Event::ShardResuming(Resuming {
                        seq: self.session.seq(),
                        shard_id: self.config.shard()[0],
                    }),
                );
            }

            let new_stream = match connect::connect(&self.url).await {
                Ok(s) => s,
                Err(why) => {
                    warn!("Error reconnecting: {:?}", why);
                    continue;
                }
            };

            let (new_forwarder, new_rx, new_tx) = SocketForwarder::new(new_stream);
            tokio::spawn(async move {
                new_forwarder.run().await;
            });

            self.rx = new_rx;
            self.session = Arc::new(Session::new(new_tx));
            match self.wtx.broadcast(Arc::clone(&self.session)) {
                Ok(_) => (),
                Err(why) => {
                    warn!(
                        "Broadcast of new session failed, \
                         This should not happen, please open \
                         a issue on the repo. {}",
                        why
                    );
                    warn!(
                        "After this many of the commands on the \
                         shard will no longer work."
                    );
                }
            };

            if !full_reconnect {
                self.session.set_stage(Stage::Resuming);
            }

            self.inflater.reset();

            break;
        }

        emit::event(
            self.listeners.clone(),
            Event::ShardConnecting(Connecting {
                gateway: self.url.clone(),
                shard_id: self.config.shard()[0],
            }),
        );
    }

    async fn resume(&mut self) -> Result<()> {
        info!("[resume] Resume started!");
        self.session.set_stage(Stage::Resuming);
        self.session.stop_heartbeater().await;

        let seq = self.session.seq();

        let id = if let Some(id) = self.session.id().await {
            id
        } else {
            warn!("Was not able to get the id, reconnecting.");
            self.reconnect(true).await;
            return Ok(());
        };

        self.resume = Some((seq, id));

        self.reconnect(false).await;

        Ok(())
    }

    pub async fn send(&mut self, payload: impl Serialize) -> Result<()> {
        match self.session.send(payload) {
            Ok(()) => Ok(()),
            Err(Error::PayloadSerialization { source }) => {
                warn!("Failed to serialize message to send: {:?}", source);

                Err(Error::PayloadSerialization { source })
            }
            Err(Error::SendingMessage { source }) => {
                warn!("Failed to send message: {:?}", source);
                info!("Reconnecting");

                self.reconnect(true).await;

                Ok(())
            }
            Err(other) => Err(other),
        }
    }

    async fn close(&mut self, close_frame: Option<CloseFrame<'static>>) -> Result<()> {
        self.session.close(close_frame)?;
        Ok(())
    }

    /// # Errors
    ///
    /// Returns [`Error::AuthorizationInvalid`] if the provided authorization
    /// is invalid.
    ///
    /// [`Error::AuthorizationInvalid`]: ../../error/enum.Error.html#variant.AuthorizationInvalid
    async fn next_event(&mut self) -> Result<GatewayEvent> {
        loop {
            // Returns None when the socket forwarder has ended, meaning the
            // connection was dropped.
            let msg = if let Some(msg) = self.rx.next().await {
                msg
            } else {
                if let Err(why) = self.resume().await {
                    warn!("Resume failed with {}, reconnecting", why);
                    self.reconnect(true).await;
                }
                continue;
            };

            match msg {
                Message::Binary(bin) => {
                    self.inflater.extend(&bin[..]);
                    let decompressed_msg = self
                        .inflater
                        .msg()
                        .map_err(|source| Error::Decompressing { source })?;
                    let msg_or_error = match decompressed_msg {
                        Some(json) => {
                            emit::bytes(self.listeners.clone(), json).await;

                            match crate::json_from_slice(json)
                                .map_err(|source| Error::PayloadSerialization { source })
                            {
                                Ok(ser) => Ok(ser),
                                Err(err) => {
                                    debug!("Broken JSON: {:?}", std::str::from_utf8(json));
                                    Err(err)
                                }
                            }
                        }
                        None => continue,
                    };
                    self.inflater.clear();
                    break msg_or_error;
                }
                Message::Close(close_frame) => {
                    log::warn!("Got close code: {:?}.", close_frame);
                    emit::event(
                        self.listeners.clone(),
                        Event::ShardDisconnected(Disconnected {
                            code: close_frame.as_ref().map(|frame| frame.code.into()),
                            reason: close_frame
                                .as_ref()
                                .map(|frame| frame.reason.clone().into()),
                            shard_id: self.config.shard()[0],
                        }),
                    );

                    if let Some(close_frame) = close_frame {
                        match close_frame.code {
                            CloseCode::Library(4004) => {
                                return Err(Error::AuthorizationInvalid {
                                    shard_id: self.config.shard()[0],
                                    token: self.config.token().to_owned(),
                                });
                            }
                            CloseCode::Library(4013) => {
                                return Err(Error::IntentsInvalid {
                                    intents: self.config.intents().copied(),
                                    shard_id: self.config.shard()[0],
                                });
                            }
                            CloseCode::Library(4014) => {
                                return Err(Error::IntentsDisallowed {
                                    intents: self.config.intents().copied(),
                                    shard_id: self.config.shard()[0],
                                });
                            }
                            _ => {}
                        }
                    }

                    self.resume().await?;
                }
                Message::Ping(_) | Message::Pong(_) => {}
                Message::Text(mut text) => {
                    trace!("Text payload: {}", text);

                    emit::bytes(self.listeners.clone(), text.as_bytes()).await;

                    break match crate::json_from_str(&mut text)
                        .map_err(|source| Error::PayloadSerialization { source })
                    {
                        Ok(ser) => Ok(ser),
                        Err(err) => {
                            debug!("Broken JSON: {:?}", &text);
                            Err(err)
                        }
                    };
                }
            }
        }
    }
}

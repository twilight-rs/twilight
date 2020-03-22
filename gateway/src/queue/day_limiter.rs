use crate::shard::error::{Error, Result};

use tokio::time::delay_until;

use std::time::Duration;

use log::warn;
use tokio::{sync::Mutex, time::Instant};

#[derive(Debug)]
pub(crate) struct DayLimiter(Mutex<DayLimiterInner>);

#[derive(Debug)]
pub(crate) struct DayLimiterInner {
    pub http: dawn_http::Client,
    pub next_reset: Instant,
    pub total: u64,
    pub current: u64,
}

impl DayLimiter {
    pub async fn new(http: &dawn_http::Client) -> Result<Self> {
        let info = http
            .gateway()
            .authed()
            .await
            .map_err(|e| Error::GettingGatewayUrl {
                source: e,
            })?;
        let next_reset = Instant::now() + Duration::from_millis(info.session_start_limit.remaining);
        let total = info.session_start_limit.total;
        let remaining = info.session_start_limit.remaining;
        assert!(total >= remaining);
        let current = total - remaining;
        Ok(DayLimiter(Mutex::new(DayLimiterInner {
            http: http.clone(),
            next_reset,
            total: info.session_start_limit.total,
            current,
        })))
    }

    pub async fn get(&self) {
        let mut lock = self.0.lock().await;
        if lock.current < lock.total {
            lock.current += 1;
        } else {
            let wait = lock.next_reset;
            delay_until(wait).await;
            if let Ok(info) = lock.http.gateway().authed().await {
                let next_reset =
                    Instant::now() + Duration::from_millis(info.session_start_limit.remaining);
                let total = info.session_start_limit.total;
                let remaining = info.session_start_limit.remaining;
                assert!(total >= remaining);
                let current = total - remaining;
                lock.next_reset = next_reset;
                lock.total = total;
                lock.current = current + 1;
            } else {
                warn!("Unable to get new session limits, skipping it. (This may cause bad things)")
            }
        }
    }
}

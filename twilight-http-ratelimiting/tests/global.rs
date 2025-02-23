use tokio::time::{advance, Instant};
use twilight_http_ratelimiting::{Path, RateLimiter, GLOBAL_LIMIT_PERIOD};

const PATH: Path = Path::ApplicationsMe;
const NOT_LIMITED_PATH: Path = Path::InteractionCallback(1);

#[tokio::test(start_paused = true)]
async fn global_limit() {
    let rate_limiter = RateLimiter::new(1);
    let now = Instant::now();

    drop(rate_limiter.acquire(PATH).await);
    assert!(now.elapsed() < GLOBAL_LIMIT_PERIOD, "did not run instantly");

    rate_limiter.acquire(PATH).await.complete(None);
    assert!(now.elapsed() < GLOBAL_LIMIT_PERIOD, "did not run instantly");

    rate_limiter.acquire(NOT_LIMITED_PATH).await.complete(None);
    assert!(now.elapsed() < GLOBAL_LIMIT_PERIOD, "did not run instantly");

    drop(rate_limiter.acquire(PATH).await);
    assert!(
        now.elapsed() >= GLOBAL_LIMIT_PERIOD,
        "misstimed global refill"
    );
}

#[tokio::test(start_paused = true)]
async fn global_reset_on_cancel() {
    let rate_limiter = RateLimiter::new(1);

    let permit = rate_limiter.acquire(PATH).await;

    advance(GLOBAL_LIMIT_PERIOD / 2).await;

    drop(permit);

    rate_limiter.acquire(PATH).await.complete(None);
    let now = Instant::now();

    drop(rate_limiter.acquire(PATH).await);
    assert!(
        now.elapsed() >= GLOBAL_LIMIT_PERIOD,
        "misstimed global refill"
    );
}

#[tokio::test(start_paused = true)]
async fn global_reset_preemptive() {
    let rate_limiter = RateLimiter::new(2);

    rate_limiter.acquire(PATH).await.complete(None);

    advance(GLOBAL_LIMIT_PERIOD).await;

    rate_limiter.acquire(PATH).await.complete(None);
    rate_limiter.acquire(PATH).await.complete(None);

    let now = Instant::now();
    drop(rate_limiter.acquire(PATH).await);

    assert!(
        now.elapsed() >= GLOBAL_LIMIT_PERIOD,
        "misstimed global refill"
    );
}

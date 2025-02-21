use tokio::{
    task,
    time::{Duration, Instant},
};
use twilight_http_ratelimiting::{Path, RateLimitHeaders, RateLimiter, GLOBAL_LIMIT_PERIOD};

const PATH: Path = Path::ApplicationsMe;

#[tokio::test]
async fn acquire_serial() {
    let rate_limiter = RateLimiter::default();

    let permit_fut1 = rate_limiter.acquire(PATH);
    let mut permit_fut2 = rate_limiter.acquire(PATH);

    tokio::select! {
        biased;
        _ = &mut permit_fut2 => panic!("not acquired in order"),
        permit = permit_fut1 => {
            permit.complete(None);
            _ = permit_fut2.await;
        }
    }
}

#[tokio::test]
async fn acquire_if() {
    let rate_limiter = RateLimiter::default();

    assert!(rate_limiter.acquire_if(PATH, |_| false).await.is_none());
    assert!(rate_limiter.acquire_if(PATH, |_| true).await.is_some());
}

#[tokio::test]
async fn bucket() {
    let rate_limiter = RateLimiter::default();

    let limit = 2;
    let remaining = 1;
    let reset_at = Instant::now() + Duration::from_secs(1);
    let headers = RateLimitHeaders {
        bucket: vec![0, 1, 2, 3],
        limit,
        remaining,
        reset_at,
    };

    rate_limiter.acquire(PATH).await.complete(Some(headers));
    task::yield_now().await;

    let bucket = rate_limiter.bucket(PATH).await.unwrap();
    assert_eq!(bucket.limit, limit);
    assert_eq!(bucket.remaining, remaining);
    assert!(bucket.reset_at.saturating_duration_since(reset_at) < Duration::from_millis(1));
}

#[tokio::test(start_paused = true)]
async fn global() {
    let rate_limiter = RateLimiter::new(1);
    let now = Instant::now();

    rate_limiter.acquire(PATH).await.complete(None);

    assert!(now.elapsed() < GLOBAL_LIMIT_PERIOD, "did not run instantly");

    rate_limiter.acquire(PATH).await.complete(None);

    assert!(
        now.elapsed() >= GLOBAL_LIMIT_PERIOD,
        "misstimed global refill"
    );
}

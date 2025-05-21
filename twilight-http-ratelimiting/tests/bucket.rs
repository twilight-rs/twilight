use tokio::time::{self, Duration, Instant};
use twilight_http_ratelimiting::{Path, RateLimitHeaders, RateLimiter};

const BUCKET: fn() -> Vec<u8> = || vec![1, 2, 3, 4];
const BUCKET2: fn() -> Vec<u8> = || vec![2, 3, 4, 5];
const PATH: Path = Path::ChannelsId(1);
const PATH2: Path = Path::ChannelsIdFollowers(1);
const RESET_AFTER: Duration = Duration::from_secs(1);

#[tokio::test(start_paused = true)]
async fn bucket_limit() {
    let rate_limiter = RateLimiter::default();

    let now = Instant::now();
    let permit_fut1 = rate_limiter.acquire(PATH);
    let mut permit_fut2 = rate_limiter.acquire(PATH);

    tokio::select! {
        biased;
        _ = &mut permit_fut2 => panic!("not acquired in order"),
        permit = permit_fut1 => permit.complete(Some(RateLimitHeaders {
            bucket: BUCKET(),
            limit: 10,
            remaining: 0,
            reset_at: now.into_std() + RESET_AFTER,
        })),
    }

    time::advance(RESET_AFTER / 2).await;

    assert!(rate_limiter.bucket(PATH).await.is_some());

    time::advance(RESET_AFTER / 2).await;

    assert!(rate_limiter.bucket(PATH).await.is_none());
    _ = permit_fut2.await;
    assert!(now.elapsed() >= RESET_AFTER);
}

#[tokio::test(start_paused = true)]
async fn bucket_sublimit() {
    let rate_limiter = RateLimiter::default();

    let reset_at = Instant::now().into_std() + RESET_AFTER;
    rate_limiter
        .acquire(PATH)
        .await
        .complete(Some(RateLimitHeaders {
            bucket: BUCKET(),
            limit: 3,
            remaining: 2,
            reset_at,
        }));

    let mut bucket = rate_limiter.bucket(PATH).await.unwrap();
    assert_eq!(bucket.limit, 3);
    assert_eq!(bucket.remaining, 2);

    rate_limiter
        .acquire(PATH)
        .await
        .complete(Some(RateLimitHeaders {
            bucket: BUCKET2(),
            limit: 10,
            remaining: 9,
            reset_at,
        }));

    let bucket2 = rate_limiter.bucket(PATH).await.unwrap();
    assert_eq!(bucket2.limit, 10);
    assert_eq!(bucket2.remaining, 9);

    rate_limiter
        .acquire(PATH)
        .await
        .complete(Some(RateLimitHeaders {
            bucket: BUCKET(),
            limit: 3,
            remaining: 1,
            reset_at,
        }));

    bucket = rate_limiter.bucket(PATH).await.unwrap();
    assert_eq!(bucket.limit, 3);
    assert_eq!(bucket.remaining, 1);
}

#[tokio::test(start_paused = true)]
async fn bucket_shared() {
    let rate_limiter = RateLimiter::default();

    let reset_at = Instant::now().into_std() + RESET_AFTER;
    rate_limiter
        .acquire(PATH)
        .await
        .complete(Some(RateLimitHeaders {
            bucket: BUCKET(),
            limit: 3,
            remaining: 2,
            reset_at,
        }));

    let mut path = rate_limiter.bucket(PATH).await.unwrap();
    assert_eq!(path.limit, 3);
    assert_eq!(path.remaining, 2);

    rate_limiter
        .acquire(PATH2)
        .await
        .complete(Some(RateLimitHeaders {
            bucket: BUCKET(),
            limit: 3,
            remaining: 1,
            reset_at,
        }));

    let mut path2 = rate_limiter.bucket(PATH2).await.unwrap();
    assert_eq!(path2.limit, 3);
    assert_eq!(path2.remaining, 1);

    path = rate_limiter.bucket(PATH).await.unwrap();
    assert_eq!(path.limit, 3);
    assert_eq!(path.remaining, 1);

    rate_limiter
        .acquire(PATH)
        .await
        .complete(Some(RateLimitHeaders {
            bucket: BUCKET2(),
            limit: 5,
            remaining: 4,
            reset_at,
        }));

    path = rate_limiter.bucket(PATH).await.unwrap();
    assert_eq!(path.limit, 5);
    assert_eq!(path.remaining, 4);

    path2 = rate_limiter.bucket(PATH2).await.unwrap();
    assert_eq!(path2.limit, 3);
    assert_eq!(path2.remaining, 1);
}

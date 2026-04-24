//! Rate limiter: per-IP token bucket for search (60 req/min) and uploads (10 req/min).

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct RateLimiter {
    inner: Arc<Mutex<RateLimitState>>,
}

#[derive(Debug)]
struct RateLimitState {
    search_buckets: HashMap<String, TokenBucket>,
    upload_buckets: HashMap<String, TokenBucket>,
}

#[derive(Debug, Clone)]
struct TokenBucket {
    tokens: f32,
    last_refill: Instant,
}

impl RateLimiter {
    pub fn new() -> Self {
        RateLimiter {
            inner: Arc::new(Mutex::new(RateLimitState {
                search_buckets: HashMap::new(),
                upload_buckets: HashMap::new(),
            })),
        }
    }

    /// Check if an IP can perform a search (60 req/min = 1 token/sec).
    pub fn check_search(&self, ip: &str) -> bool {
        let mut state = self.inner.lock().unwrap();
        self.check_token_bucket(
            &mut state.search_buckets,
            ip,
            1.0, // 1 token per request
            60.0, // capacity = 60 tokens (1 per second × 60 seconds)
            Duration::from_secs(60),
        )
    }

    /// Check if an IP can upload (10 req/min = 0.167 tokens/sec).
    pub fn check_upload(&self, ip: &str) -> bool {
        let mut state = self.inner.lock().unwrap();
        self.check_token_bucket(
            &mut state.upload_buckets,
            ip,
            1.0, // 1 token per request
            10.0, // capacity = 10 tokens
            Duration::from_secs(60),
        )
    }

    fn check_token_bucket(
        &self,
        buckets: &mut HashMap<String, TokenBucket>,
        key: &str,
        cost: f32,
        capacity: f32,
        window: Duration,
    ) -> bool {
        let now = Instant::now();
        let bucket = buckets.entry(key.to_string()).or_insert_with(|| TokenBucket {
            tokens: capacity,
            last_refill: now,
        });

        // Refill based on elapsed time
        let elapsed = now.duration_since(bucket.last_refill).as_secs_f32();
        let refill_rate = capacity / window.as_secs_f32();
        bucket.tokens = (bucket.tokens + elapsed * refill_rate).min(capacity);
        bucket.last_refill = now;

        // Try to consume token
        if bucket.tokens >= cost {
            bucket.tokens -= cost;
            true
        } else {
            false
        }
    }
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_rate_limit_allows_initial_burst() {
        let limiter = RateLimiter::new();
        for _ in 0..60 {
            assert!(limiter.check_search("127.0.0.1"), "should allow 60 initial requests");
        }
        assert!(!limiter.check_search("127.0.0.1"), "should reject 61st request");
    }

    #[test]
    fn upload_rate_limit_allows_initial_burst() {
        let limiter = RateLimiter::new();
        for _ in 0..10 {
            assert!(limiter.check_upload("127.0.0.1"), "should allow 10 initial requests");
        }
        assert!(!limiter.check_upload("127.0.0.1"), "should reject 11th request");
    }

    #[test]
    fn different_ips_have_separate_limits() {
        let limiter = RateLimiter::new();
        for _ in 0..60 {
            assert!(limiter.check_search("127.0.0.1"));
        }
        assert!(limiter.check_search("127.0.0.2"), "different IP should have own bucket");
    }
}

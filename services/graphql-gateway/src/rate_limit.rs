//! Per-client rate limiting: 100 requests per minute.
//!
//! Tracks client IP (from X-Forwarded-For or socket addr) in a rolling window.

use std::collections::HashMap;
use std::sync::Mutex;
use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Clone, Copy)]
struct RequestRecord {
    timestamp: DateTime<Utc>,
}

/// Rate limiter: 100 req/min per client IP.
pub struct RateLimiter {
    state: Mutex<HashMap<String, Vec<RequestRecord>>>,
}

impl RateLimiter {
    /// Create a new rate limiter.
    pub fn new() -> Self {
        Self {
            state: Mutex::new(HashMap::new()),
        }
    }

    /// Check if a client has exceeded the rate limit (100/min).
    /// Returns true if allowed, false if rate-limited.
    pub fn check(&self, client_ip: &str) -> bool {
        let mut state = self.state.lock().unwrap();
        let now = Utc::now();
        let cutoff = now - Duration::minutes(1);

        let records = state.entry(client_ip.to_string()).or_insert_with(Vec::new);

        // Remove old records outside the 1-minute window.
        records.retain(|r| r.timestamp > cutoff);

        // Allow if under the limit.
        if records.len() < 100 {
            records.push(RequestRecord { timestamp: now });
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
    fn rate_limit_under_threshold() {
        let limiter = RateLimiter::new();
        for _ in 0..50 {
            assert!(limiter.check("192.168.1.1"));
        }
    }

    #[test]
    fn rate_limit_at_threshold() {
        let limiter = RateLimiter::new();
        for _ in 0..100 {
            assert!(limiter.check("192.168.1.1"));
        }
        // 101st request should be rejected.
        assert!(!limiter.check("192.168.1.1"));
    }

    #[test]
    fn rate_limit_per_client() {
        let limiter = RateLimiter::new();
        for _ in 0..50 {
            assert!(limiter.check("192.168.1.1"));
            assert!(limiter.check("192.168.1.2"));
        }
    }
}

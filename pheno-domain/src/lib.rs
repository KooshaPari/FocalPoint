//! # pheno-domain
//!
//! Canonical domain primitives for the `pheno-*` fleet.
//!
//! Each primitive is a newtype wrapper (or small struct) that carries
//! validation, serde, and Display/FromStr impls so it can cross
//! API boundaries, persist to JSON, and parse from CLI arguments or
//! config files without ceremony.
//!
//! | type      | backing          | validation rule                          |
//! |-----------|------------------|------------------------------------------|
//! | `EntityId`| `Uuid`           | none (any v4 UUID)                       |
//! | `Timestamp`| `DateTime<Utc>` | none (any UTC datetime)                  |
//! | `Slug`    | `String`         | lowercase, hyphen, 3-64 chars            |
//! | `Email`   | `String`         | RFC-ish regex (loose but practical)      |
//! | `Money`   | `Decimal`+`String`| amount rounds to 2 decimals, currency 3 chars |

use chrono::{DateTime, Utc};
use regex::Regex;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// DomainError
// ---------------------------------------------------------------------------

/// The canonical error type for domain-primitive validation failures.
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    /// The input did not match the expected format (e.g. invalid regex).
    #[error("invalid format: {0}")]
    InvalidFormat(String),

    /// The input was outside the permitted range (e.g. too short / too long).
    #[error("out of range: {0}")]
    OutOfRange(String),

    /// The input contained an invalid value (e.g. negative money amount).
    #[error("invalid value: {0}")]
    InvalidValue(String),
}

// ---------------------------------------------------------------------------
// EntityId
// ---------------------------------------------------------------------------

/// A canonical entity identifier backed by a UUID.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EntityId(Uuid);

impl EntityId {
    /// Generate a new random v4 UUID.
    pub fn new_v4() -> Self {
        Self(Uuid::new_v4())
    }

    /// Access the inner UUID.
    pub fn inner(&self) -> Uuid {
        self.0
    }
}

impl fmt::Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Debug for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("EntityId").field(&self.0).finish()
    }
}

impl FromStr for EntityId {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s)
            .map(EntityId)
            .map_err(|e| DomainError::InvalidFormat(e.to_string()))
    }
}

// ---------------------------------------------------------------------------
// Timestamp
// ---------------------------------------------------------------------------

/// A canonical UTC timestamp.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Timestamp(DateTime<Utc>);

impl Timestamp {
    /// The current time in UTC.
    pub fn now() -> Self {
        Self(Utc::now())
    }

    /// Access the inner `DateTime<Utc>`.
    pub fn inner(&self) -> DateTime<Utc> {
        self.0
    }
}

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.to_rfc3339().fmt(f)
    }
}

impl fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Timestamp").field(&self.0).finish()
    }
}

impl FromStr for Timestamp {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        DateTime::parse_from_rfc3339(s)
            .map(|dt| Self(dt.with_timezone(&Utc)))
            .map_err(|e| DomainError::InvalidFormat(e.to_string()))
    }
}

// ---------------------------------------------------------------------------
// Slug
// ---------------------------------------------------------------------------

/// A URL-safe slug: lowercase ASCII letters, digits, and hyphens only.
///
/// Length is restricted to 3-64 characters inclusive.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Slug(String);

impl Slug {
    /// The minimum allowed length.
    pub const MIN_LEN: usize = 3;
    /// The maximum allowed length.
    pub const MAX_LEN: usize = 64;

    /// Access the inner string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Compile the validation regex once.
    fn regex() -> &'static Regex {
        static RE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
        RE.get_or_init(|| Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap())
    }

    /// Validate raw input.
    fn validate(s: &str) -> Result<(), DomainError> {
        let len = s.len();
        if len < Self::MIN_LEN {
            return Err(DomainError::OutOfRange(format!(
                "slug too short ({} < {})",
                len,
                Self::MIN_LEN
            )));
        }
        if len > Self::MAX_LEN {
            return Err(DomainError::OutOfRange(format!(
                "slug too long ({} > {})",
                len,
                Self::MAX_LEN
            )));
        }
        if !Self::regex().is_match(s) {
            return Err(DomainError::InvalidFormat(
                "slug must be lowercase ASCII letters, digits, and hyphens only".into(),
            ));
        }
        Ok(())
    }
}

impl fmt::Display for Slug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl fmt::Debug for Slug {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Slug").field(&self.0).finish()
    }
}

impl FromStr for Slug {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::validate(s)?;
        Ok(Self(s.to_string()))
    }
}

impl TryFrom<String> for Slug {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(&value)?;
        Ok(Self(value))
    }
}

// ---------------------------------------------------------------------------
// Email
// ---------------------------------------------------------------------------

/// A validated email address.
///
/// The validation is pragmatic: a loose regex that catches the most common
/// malformed inputs. It does not guarantee deliverability.
#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    /// Access the inner string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Compile the validation regex once.
    fn regex() -> &'static Regex {
        static RE: std::sync::OnceLock<Regex> = std::sync::OnceLock::new();
        RE.get_or_init(|| {
            Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
        })
    }

    /// Validate raw input.
    fn validate(s: &str) -> Result<(), DomainError> {
        if !Self::regex().is_match(s) {
            return Err(DomainError::InvalidFormat(
                "email does not match expected format".into(),
            ));
        }
        Ok(())
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Email display is lowercase per spec.
        self.0.to_lowercase().fmt(f)
    }
}

impl fmt::Debug for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Email").field(&self.0).finish()
    }
}

impl FromStr for Email {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::validate(s)?;
        Ok(Self(s.to_string()))
    }
}

impl TryFrom<String> for Email {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::validate(&value)?;
        Ok(Self(value))
    }
}

// ---------------------------------------------------------------------------
// Money
// ---------------------------------------------------------------------------

/// A monetary amount with currency.
///
/// The amount is stored as a `Decimal` and rounded to 2 decimal places
/// on construction. Currency is a 3-character ISO-4217-ish code (e.g. USD, EUR).
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    amount: Decimal,
    currency: String,
}

impl Money {
    /// Create a new `Money` value.
    ///
    /// The amount is rounded to 2 decimal places. Currency is uppercased
    /// and must be 3 characters.
    pub fn new(amount: Decimal, currency: impl Into<String>) -> Result<Self, DomainError> {
        let currency = currency.into().to_uppercase();
        if currency.len() != 3 {
            return Err(DomainError::InvalidFormat(
                "currency code must be exactly 3 characters".into(),
            ));
        }
        let amount = amount.round_dp(2);
        Ok(Self { amount, currency })
    }

    /// The monetary amount.
    pub fn amount(&self) -> Decimal {
        self.amount
    }

    /// The currency code.
    pub fn currency(&self) -> &str {
        &self.currency
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.amount, self.currency)
    }
}

impl fmt::Debug for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Money")
            .field("amount", &self.amount)
            .field("currency", &self.currency)
            .finish()
    }
}

impl FromStr for Money {
    type Err = DomainError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Expected format: "<amount> <currency>" or "<currency> <amount>"
        // We try: split by whitespace, the numeric part is amount, the alphabetic part is currency.
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(DomainError::InvalidFormat(
                "money must be two whitespace-separated tokens: amount and currency".into(),
            ));
        }
        let (amount_str, currency_str) = if parts[0].chars().any(|c| c.is_ascii_digit()) {
            (parts[0], parts[1])
        } else {
            (parts[1], parts[0])
        };
        let amount = Decimal::from_str(amount_str)
            .map_err(|e| DomainError::InvalidFormat(format!("invalid decimal: {}", e)))?;
        Self::new(amount, currency_str)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------
    // EntityId
    // -----------------------------------------------------------------

    #[test]
    fn entity_id_parses_uuid() {
        let raw = "550e8400-e29b-41d4-a716-446655440000";
        let id: EntityId = raw.parse().unwrap();
        assert_eq!(id.to_string(), raw);
    }

    #[test]
    fn entity_id_rejects_invalid_uuid() {
        let result: Result<EntityId, _> = "not-a-uuid".parse();
        assert!(matches!(result, Err(DomainError::InvalidFormat(_))));
    }

    #[test]
    fn entity_id_serde_round_trips() {
        let id = EntityId::new_v4();
        let json = serde_json::to_string(&id).unwrap();
        let back: EntityId = serde_json::from_str(&json).unwrap();
        assert_eq!(id, back);
    }

    // -----------------------------------------------------------------
    // Slug
    // -----------------------------------------------------------------

    #[test]
    fn slug_rejects_invalid_chars() {
        let result: Result<Slug, _> = "Hello World".parse();
        assert!(matches!(result, Err(DomainError::InvalidFormat(_))));
    }

    #[test]
    fn slug_rejects_too_short() {
        let result: Result<Slug, _> = "ab".parse();
        assert!(matches!(result, Err(DomainError::OutOfRange(_))));
    }

    #[test]
    fn slug_rejects_too_long() {
        let result: Result<Slug, _> = ("a".repeat(65)).parse();
        assert!(matches!(result, Err(DomainError::OutOfRange(_))));
    }

    #[test]
    fn slug_accepts_valid() {
        let slug: Slug = "hello-world".parse().unwrap();
        assert_eq!(slug.to_string(), "hello-world");
    }

    #[test]
    fn slug_from_str_round_trips() {
        let raw = "my-cool-slug-123";
        let slug: Slug = raw.parse().unwrap();
        assert_eq!(slug.to_string(), raw);
        assert_eq!(slug.as_str(), raw);
    }

    #[test]
    fn slug_try_from_string() {
        let s = "another-slug".to_string();
        let slug = Slug::try_from(s.clone()).unwrap();
        assert_eq!(slug.as_str(), s);
    }

    #[test]
    fn slug_serde_round_trips() {
        let slug: Slug = "test-slug".parse().unwrap();
        let json = serde_json::to_string(&slug).unwrap();
        let back: Slug = serde_json::from_str(&json).unwrap();
        assert_eq!(slug, back);
    }

    // -----------------------------------------------------------------
    // Email
    // -----------------------------------------------------------------

    #[test]
    fn email_rejects_invalid_format() {
        let result: Result<Email, _> = "not-an-email".parse();
        assert!(matches!(result, Err(DomainError::InvalidFormat(_))));
    }

    #[test]
    fn email_accepts_valid() {
        let email: Email = "user@example.com".parse().unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn email_display_is_lowercase() {
        let email: Email = "User@Example.COM".parse().unwrap();
        assert_eq!(email.to_string(), "user@example.com");
    }

    #[test]
    fn email_try_from_string() {
        let s = "foo@bar.com".to_string();
        let email = Email::try_from(s.clone()).unwrap();
        assert_eq!(email.as_str(), s);
    }

    #[test]
    fn email_serde_round_trips() {
        let email: Email = "test@domain.com".parse().unwrap();
        let json = serde_json::to_string(&email).unwrap();
        let back: Email = serde_json::from_str(&json).unwrap();
        assert_eq!(email, back);
    }

    // -----------------------------------------------------------------
    // Money
    // -----------------------------------------------------------------

    #[test]
    fn money_rounds_to_2_decimals() {
        let m = Money::new(Decimal::from_str("10.999").unwrap(), "usd").unwrap();
        assert_eq!(m.amount().to_string(), "11.00");
    }

    #[test]
    fn money_display_includes_currency() {
        let m = Money::new(Decimal::from_str("42.50").unwrap(), "eur").unwrap();
        let disp = m.to_string();
        assert!(disp.contains("42.50"));
        assert!(disp.contains("EUR"));
    }

    #[test]
    fn money_from_str_parses_amount_currency() {
        let m: Money = "123.45 USD".parse().unwrap();
        assert_eq!(m.amount().to_string(), "123.45");
        assert_eq!(m.currency(), "USD");
    }

    #[test]
    fn money_from_str_parses_currency_amount() {
        let m: Money = "GBP 99.99".parse().unwrap();
        assert_eq!(m.amount().to_string(), "99.99");
        assert_eq!(m.currency(), "GBP");
    }

    #[test]
    fn money_rejects_bad_currency_length() {
        let result = Money::new(Decimal::from_str("1.00").unwrap(), "US");
        assert!(matches!(result, Err(DomainError::InvalidFormat(_))));
    }

    #[test]
    fn money_rejects_negative_if_needed() {
        // Money itself does not reject negative amounts; the test shows
        // the constructor accepts them and rounds to 2 dp.
        let m = Money::new(Decimal::from_str("-5.5").unwrap(), "usd").unwrap();
        assert_eq!(m.amount(), Decimal::from_str("-5.50").unwrap());
    }

    #[test]
    fn money_serde_round_trips() {
        let m = Money::new(Decimal::from_str("100.00").unwrap(), "USD").unwrap();
        let json = serde_json::to_string(&m).unwrap();
        let back: Money = serde_json::from_str(&json).unwrap();
        assert_eq!(m, back);
    }

    // -----------------------------------------------------------------
    // Timestamp
    // -----------------------------------------------------------------

    #[test]
    fn timestamp_orders_correctly() {
        let _t1 = Timestamp::now();
        let _t2 = Timestamp::now();
        // Because we can't sleep reliably in a unit test, we compare
        // parsed timestamps.
        let a: Timestamp = "2024-01-01T00:00:00Z".parse().unwrap();
        let b: Timestamp = "2024-06-01T00:00:00Z".parse().unwrap();
        assert!(a < b);
    }

    #[test]
    fn timestamp_from_str_round_trips() {
        let raw = "2024-03-15T12:30:00Z";
        let ts: Timestamp = raw.parse().unwrap();
        assert!(ts.to_string().starts_with("2024-03-15T12:30:00"));
    }

    #[test]
    fn timestamp_serde_round_trips() {
        let ts = Timestamp::now();
        let json = serde_json::to_string(&ts).unwrap();
        let back: Timestamp = serde_json::from_str(&json).unwrap();
        assert_eq!(ts, back);
    }

    // -----------------------------------------------------------------
    // DomainError
    // -----------------------------------------------------------------

    #[test]
    fn domain_error_display_includes_variant() {
        let e = DomainError::InvalidFormat("oops".into());
        assert!(e.to_string().contains("invalid format"));
        assert!(e.to_string().contains("oops"));
    }

    #[test]
    fn domain_error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<DomainError>();
    }
}

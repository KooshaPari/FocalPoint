//! Connector builder macros — eliminates copy-pasted builder boilerplate across
//! all connector crates (Fitbit, Strava, GitHub, GCal, Canvas, Linear, Notion,
//! Readwise).

/// Pattern A: OAuth2 with client_id/client_secret (e.g. Fitbit, Strava).
///
/// Generates `ConnectorBuilder` struct and common builder methods
/// (`new`, `account_id`, `token_store`, `oauth`, `http`).
/// The caller writes the `build` method in a separate `impl` block.
#[macro_export]
macro_rules! connector_builder_common_oauth2_client_id {
    (
        builder: $builder:ident,
        token_store: $token_store:ty,
        oauth: $oauth:ty,
    ) => {
        pub struct $builder {
            #[allow(dead_code)]
            client_id: String,
            #[allow(dead_code)]
            client_secret: String,
            account_id: uuid::Uuid,
            token_store: Option<std::sync::Arc<$token_store>>,
            oauth: Option<std::sync::Arc<$oauth>>,
            http: Option<reqwest::Client>,
        }

        impl $builder {
            pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
                Self {
                    client_id: client_id.into(),
                    client_secret: client_secret.into(),
                    account_id: uuid::Uuid::nil(),
                    token_store: None,
                    oauth: None,
                    http: None,
                }
            }

            pub fn account_id(mut self, id: uuid::Uuid) -> Self {
                self.account_id = id;
                self
            }

            pub fn token_store(mut self, s: std::sync::Arc<$token_store>) -> Self {
                self.token_store = Some(s);
                self
            }

            pub fn oauth(mut self, o: std::sync::Arc<$oauth>) -> Self {
                self.oauth = Some(o);
                self
            }

            pub fn http(mut self, h: reqwest::Client) -> Self {
                self.http = Some(h);
                self
            }
        }
    };
}

/// Pattern B: OAuth2 with base_url (e.g. GitHub PAT, GCal, Canvas).
///
/// Generates `ConnectorBuilder` struct and common builder methods
/// (`new`, `base_url`, `account_id`, `token_store`, `oauth`, `http`, `scopes`).
/// The caller writes the `build` method in a separate `impl` block.
#[macro_export]
macro_rules! connector_builder_common_oauth2_base_url {
    (
        builder: $builder:ident,
        token_store: $token_store:ty,
        oauth: $oauth:ty,
    ) => {
        pub struct $builder {
            base_url: String,
            account_id: uuid::Uuid,
            token_store: Option<std::sync::Arc<$token_store>>,
            oauth: Option<std::sync::Arc<$oauth>>,
            http: Option<reqwest::Client>,
            scopes: Option<Vec<String>>,
        }

        impl $builder {
            pub fn new(base_url: impl Into<String>) -> Self {
                Self {
                    base_url: base_url.into(),
                    account_id: uuid::Uuid::nil(),
                    token_store: None,
                    oauth: None,
                    http: None,
                    scopes: None,
                }
            }

            pub fn base_url(mut self, url: impl Into<String>) -> Self {
                self.base_url = url.into();
                self
            }

            pub fn account_id(mut self, id: uuid::Uuid) -> Self {
                self.account_id = id;
                self
            }

            pub fn token_store(mut self, s: std::sync::Arc<$token_store>) -> Self {
                self.token_store = Some(s);
                self
            }

            pub fn oauth(mut self, o: std::sync::Arc<$oauth>) -> Self {
                self.oauth = Some(o);
                self
            }

            pub fn http(mut self, h: reqwest::Client) -> Self {
                self.http = Some(h);
                self
            }

            pub fn scopes(mut self, scopes: Vec<String>) -> Self {
                self.scopes = Some(scopes);
                self
            }
        }
    };
}

/// Pattern C: API-key / no-auth-required connectors (e.g. Linear, Notion, Readwise).
///
/// Generates `ConnectorBuilder` struct and common builder methods
/// (`new`, `account_id`, `token_store`, `http`).
/// The caller writes the `build` method in a separate `impl` block.
#[macro_export]
macro_rules! connector_builder_common_api_key {
    (
        builder: $builder:ident,
        token_store: $token_store:ty,
    ) => {
        pub struct $builder {
            account_id: uuid::Uuid,
            token_store: Option<std::sync::Arc<$token_store>>,
            http: Option<reqwest::Client>,
        }

        impl Default for $builder {
            fn default() -> Self {
                Self {
                    account_id: uuid::Uuid::nil(),
                    token_store: None,
                    http: None,
                }
            }
        }

        impl $builder {
            pub fn new() -> Self {
                Self::default()
            }

            pub fn account_id(mut self, id: uuid::Uuid) -> Self {
                self.account_id = id;
                self
            }

            pub fn token_store(mut self, s: std::sync::Arc<$token_store>) -> Self {
                self.token_store = Some(s);
                self
            }

            pub fn http(mut self, h: reqwest::Client) -> Self {
                self.http = Some(h);
                self
            }
        }
    };
}

/// Backward-compatible alias — Pattern A (OAuth2 + client_id/client_secret).
pub use connector_builder_common_oauth2_client_id as connector_builder;

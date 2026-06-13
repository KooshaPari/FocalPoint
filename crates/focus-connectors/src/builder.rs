//! Connector builder macro — eliminates copy-pasted builder boilerplate across
//! all connector crates (Fitbit, Strava, GitHub, GCal, Canvas, Linear, Notion,
//! Readwise).

/// Generates the common `ConnectorBuilder` struct + methods for any connector.
///
/// Each connector defines:
/// - `ConnectorName` — the pascal-case connector name (e.g. `FitbitConnector`).
/// - `ConnectorBuilder` — the builder struct name (e.g. `FitbitConnectorBuilder`).
/// - `TokenStoreType` — the connector-specific token-store trait (e.g. `dyn TokenStore`).
/// - `OAuthType` — the connector-specific OAuth type (e.g. `FitbitOAuth2`).
/// - `build_impl` — the `build(self) -> ConnectorName` method body.
///
/// # Example
/// ```ignore
/// connector_builder! {
///     connector: FitbitConnector,
///     builder: FitbitConnectorBuilder,
///     token_store: dyn TokenStore,
///     oauth: FitbitOAuth2,
///     build_impl: {
///         let http = self.http.unwrap_or_default();
///         let store = self
///             .token_store
///             .unwrap_or_else(|| Arc::new(KeychainTokenStore::new()));
///         let client = FitbitClient::new(http);
///         FitbitConnector {
///             manifest: default_manifest(),
///             account_id: self.account_id,
///             token_store: store,
///             oauth: self.oauth,
///             client: Mutex::new(client),
///         }
///     }
/// }
/// ```
#[macro_export]
macro_rules! connector_builder {
    (
        connector: $connector:ident,
        builder: $builder:ident,
        token_store: $token_store:ty,
        oauth: $oauth:ty,
        build_impl: $build_impl:block
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
            pub fn new(
                client_id: impl Into<String>,
                client_secret: impl Into<String>,
            ) -> Self {
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

            pub fn token_store(
                mut self,
                s: std::sync::Arc<$token_store>,
            ) -> Self {
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

            pub fn build(self) -> $connector {
                $build_impl
            }
        }
    };
}

/// Re-export for convenience so connectors can `use focus_connectors::connector_builder`.
pub use connector_builder;

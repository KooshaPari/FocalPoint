//! FocalPoint Transpilers — connector-like trait pattern for transpilation pipelines.

use std::collections::HashMap;

use thiserror::Error;

pub use focus_errors::FocusError;
pub use focus_result::Result;

// ---------------------------------------------------------------------------
// TranspilerError — per-crate error enum mapped into FocusError.
// ---------------------------------------------------------------------------

#[derive(Debug, Error)]
pub enum TranspilerError {
    #[error("transpiler not found: {0}")]
    NotFound(String),
    #[error("transpiler already registered: {0}")]
    AlreadyRegistered(String),
    #[error("connection failed: {0}")]
    ConnectionFailed(String),
    #[error("send failed: {0}")]
    SendFailed(String),
    #[error("receive failed: {0}")]
    ReceiveFailed(String),
    #[error("invalid config: {0}")]
    InvalidConfig(String),
}

impl From<TranspilerError> for FocusError {
    fn from(err: TranspilerError) -> Self {
        match err {
            TranspilerError::NotFound(msg) => FocusError::NotFound(msg),
            TranspilerError::AlreadyRegistered(msg) => FocusError::Conflict(msg),
            TranspilerError::ConnectionFailed(msg) => FocusError::Network(msg),
            TranspilerError::SendFailed(msg) => FocusError::Network(msg),
            TranspilerError::ReceiveFailed(msg) => FocusError::Network(msg),
            TranspilerError::InvalidConfig(msg) => FocusError::Schema(msg),
        }
    }
}

// ---------------------------------------------------------------------------
// Config — transpiler endpoint configuration.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Config {
    pub transpiler_id: String,
    pub endpoint: String,
    pub auth_token: String,
    pub headers: HashMap<String, String>,
}

// ---------------------------------------------------------------------------
// TranspileData — opaque payload.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct TranspileData(pub Vec<u8>);

// ---------------------------------------------------------------------------
// Connection — active transpiler session.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Connection {
    pub transpiler_id: String,
    pub config: Config,
    pub connected_at: chrono::DateTime<chrono::Utc>,
}

// ---------------------------------------------------------------------------
// Transpiler — connector-like trait for transpilation pipelines.
// ---------------------------------------------------------------------------

pub trait Transpiler {
    fn connect(&mut self, config: &Config) -> Result<Connection>;
    fn disconnect(&mut self, conn: &mut Connection) -> Result<()>;
    fn send(&mut self, conn: &mut Connection, data: TranspileData) -> Result<()>;
    fn receive(&mut self, conn: &mut Connection) -> Result<TranspileData>;
    fn is_connected(&self, conn: &Connection) -> bool;
}

// ---------------------------------------------------------------------------
// TranspilerRegistry — catalog of transpilers with batch operations.
// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct TranspilerRegistry {
    transpilers: HashMap<String, Box<dyn Transpiler>>,
}

impl TranspilerRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, id: &str, transpiler: Box<dyn Transpiler>) -> Result<()> {
        if self.transpilers.contains_key(id) {
            return Err(TranspilerError::AlreadyRegistered(id.to_string()).into());
        }
        self.transpilers.insert(id.to_string(), transpiler);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&dyn Transpiler> {
        self.transpilers.get(id).map(|b| b.as_ref())
    }

    pub fn list(&self) -> Vec<&str> {
        self.transpilers.keys().map(|k| k.as_str()).collect()
    }

    pub fn remove(&mut self, id: &str) -> Result<()> {
        if self.transpilers.remove(id).is_none() {
            return Err(TranspilerError::NotFound(id.to_string()).into());
        }
        Ok(())
    }

    pub fn connect_all(&mut self, config: &Config) -> Result<Vec<Connection>> {
        let mut connections = Vec::new();
        for (id, transpiler) in &mut self.transpilers {
            let mut cfg = config.clone();
            cfg.transpiler_id = id.clone();
            let conn = transpiler.connect(&cfg)?;
            connections.push(conn);
        }
        Ok(connections)
    }

    pub fn disconnect_all(&mut self, conns: &mut [Connection]) -> Result<()> {
        for conn in conns {
            if let Some(transpiler) = self.transpilers.get_mut(&conn.transpiler_id) {
                transpiler.disconnect(conn)?;
            }
        }
        Ok(())
    }

    pub fn send_to_all(
        &mut self,
        conns: &mut [Connection],
        data: TranspileData,
    ) -> Result<()> {
        for conn in conns {
            if let Some(transpiler) = self.transpilers.get_mut(&conn.transpiler_id) {
                transpiler.send(conn, data.clone())?;
            }
        }
        Ok(())
    }

    pub fn receive_from_all(
        &mut self,
        conns: &mut [Connection],
    ) -> Result<Vec<TranspileData>> {
        let mut results = Vec::new();
        for conn in conns {
            if let Some(transpiler) = self.transpilers.get_mut(&conn.transpiler_id) {
                let data = transpiler.receive(conn)?;
                results.push(data);
            }
        }
        Ok(results)
    }

    pub fn is_any_connected(&self, conns: &[Connection]) -> bool {
        conns.iter().any(|conn| {
            self.transpilers
                .get(&conn.transpiler_id)
                .map(|t| t.is_connected(conn))
                .unwrap_or(false)
        })
    }

    pub fn is_empty(&self) -> bool {
        self.transpilers.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockTranspiler {
        connected: bool,
    }

    impl Transpiler for MockTranspiler {
        fn connect(&mut self, config: &Config) -> Result<Connection> {
            self.connected = true;
            Ok(Connection {
                transpiler_id: config.transpiler_id.clone(),
                config: config.clone(),
                connected_at: chrono::Utc::now(),
            })
        }

        fn disconnect(&mut self, _conn: &mut Connection) -> Result<()> {
            self.connected = false;
            Ok(())
        }

        fn send(&mut self, _conn: &mut Connection, _data: TranspileData) -> Result<()> {
            Ok(())
        }

        fn receive(&mut self, _conn: &mut Connection) -> Result<TranspileData> {
            Ok(TranspileData(vec![0x01, 0x02, 0x03]))
        }

        fn is_connected(&self, _conn: &Connection) -> bool {
            self.connected
        }
    }

    fn mk_config(id: &str) -> Config {
        Config {
            transpiler_id: id.to_string(),
            endpoint: "http://localhost:8080".to_string(),
            auth_token: "secret".to_string(),
            headers: HashMap::new(),
        }
    }

    #[test]
    fn registry_register_and_list() {
        let mut reg = TranspilerRegistry::new();
        reg.register("mock", Box::new(MockTranspiler { connected: false }))
            .unwrap();
        let ids = reg.list();
        assert_eq!(ids, vec!["mock"]);
    }

    #[test]
    fn registry_register_duplicate_errors() {
        let mut reg = TranspilerRegistry::new();
        reg.register("mock", Box::new(MockTranspiler { connected: false }))
            .unwrap();
        let err = reg
            .register("mock", Box::new(MockTranspiler { connected: false }))
            .unwrap_err();
        assert!(matches!(err, FocusError::Conflict(_)));
    }

    #[test]
    fn registry_connect_all() {
        let mut reg = TranspilerRegistry::new();
        reg.register("a", Box::new(MockTranspiler { connected: false }))
            .unwrap();
        reg.register("b", Box::new(MockTranspiler { connected: false }))
            .unwrap();
        let conns = reg.connect_all(&mk_config("ignored")).unwrap();
        assert_eq!(conns.len(), 2);
        assert!(reg.is_any_connected(&conns));
    }

    #[test]
    fn registry_send_and_receive_all() {
        let mut reg = TranspilerRegistry::new();
        reg.register("a", Box::new(MockTranspiler { connected: false }))
            .unwrap();
        let mut conns = reg.connect_all(&mk_config("a")).unwrap();
        reg.send_to_all(&mut conns, TranspileData(vec![0xAB]))
            .unwrap();
        let received = reg.receive_from_all(&mut conns).unwrap();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].0, vec![0x01, 0x02, 0x03]);
    }

    #[test]
    fn registry_disconnect_all() {
        let mut reg = TranspilerRegistry::new();
        reg.register("a", Box::new(MockTranspiler { connected: false }))
            .unwrap();
        let mut conns = reg.connect_all(&mk_config("a")).unwrap();
        assert!(reg.is_any_connected(&conns));
        reg.disconnect_all(&mut conns).unwrap();
        assert!(!reg.is_any_connected(&conns));
    }

    #[test]
    fn registry_remove_ok() {
        let mut reg = TranspilerRegistry::new();
        reg.register("a", Box::new(MockTranspiler { connected: false }))
            .unwrap();
        reg.remove("a").unwrap();
        assert!(reg.is_empty());
    }

    #[test]
    fn registry_remove_missing_errors() {
        let mut reg = TranspilerRegistry::new();
        let err = reg.remove("missing").unwrap_err();
        assert!(matches!(err, FocusError::NotFound(_)));
    }

    #[test]
    fn transpiler_error_maps_to_focus_error() {
        let e = TranspilerError::ConnectionFailed("timeout".to_string());
        let fe: FocusError = e.into();
        assert!(matches!(fe, FocusError::Network(_)));
    }
}

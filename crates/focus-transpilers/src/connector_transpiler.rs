//! Transpiler: focus_connectors::ConnectorManifest ↔ focus_ir::ConnectorIr

use anyhow::{anyhow, Result};
use focus_connectors::{
    AuthStrategy, ConnectorCapability, ConnectorListing, ConnectorManifest, SyncMode, VerificationTier,
};
use focus_ir::{
    AuthStrategyIr, ConnectorCapabilityIr, ConnectorIr, Document, DocKind, Body,
};

/// Convert focus_connectors::ConnectorManifest to focus_ir::ConnectorIr.
pub fn manifest_to_ir(manifest: &ConnectorManifest) -> ConnectorIr {
    ConnectorIr {
        id: manifest.id.clone(),
        version: manifest.version.clone(),
        display_name: manifest.display_name.clone(),
        auth_strategy: auth_strategy_to_ir(&manifest.auth_strategy),
        sync_mode: sync_mode_to_ir(&manifest.sync_mode),
        capabilities: manifest
            .capabilities
            .iter()
            .map(|cap| ConnectorCapabilityIr {
                name: cap.name.clone(),
                params_schema: cap.params_schema.clone(),
            })
            .collect(),
        entity_types: manifest.entity_types.clone(),
        event_types: manifest.event_types.clone(),
        tier: format!("{:?}", manifest.tier),
        health_indicators: manifest.health_indicators.clone(),
    }
}

/// Convert focus_ir::ConnectorIr back to focus_connectors::ConnectorManifest.
pub fn ir_to_manifest(ir: &ConnectorIr) -> Result<ConnectorManifest> {
    Ok(ConnectorManifest {
        id: ir.id.clone(),
        version: ir.version.clone(),
        display_name: ir.display_name.clone(),
        auth_strategy: ir_to_auth_strategy(&ir.auth_strategy)?,
        sync_mode: ir_to_sync_mode(&ir.sync_mode)?,
        capabilities: ir
            .capabilities
            .iter()
            .map(|cap| ConnectorCapability {
                name: cap.name.clone(),
                params_schema: cap.params_schema.clone(),
            })
            .collect(),
        entity_types: ir.entity_types.clone(),
        event_types: ir.event_types.clone(),
        tier: parse_tier(&ir.tier)?,
        health_indicators: ir.health_indicators.clone(),
    })
}

fn auth_strategy_to_ir(strategy: &AuthStrategy) -> AuthStrategyIr {
    match strategy {
        AuthStrategy::OAuth2 { scopes } => AuthStrategyIr::OAuth2 {
            scopes: scopes.clone(),
        },
        AuthStrategy::ApiKey => AuthStrategyIr::ApiKey,
        AuthStrategy::DeviceBrokered => AuthStrategyIr::DeviceBrokered,
        AuthStrategy::None => AuthStrategyIr::None,
    }
}

fn ir_to_auth_strategy(ir: &AuthStrategyIr) -> Result<AuthStrategy> {
    match ir {
        AuthStrategyIr::OAuth2 { scopes } => Ok(AuthStrategy::OAuth2 {
            scopes: scopes.clone(),
        }),
        AuthStrategyIr::ApiKey => Ok(AuthStrategy::ApiKey),
        AuthStrategyIr::DeviceBrokered => Ok(AuthStrategy::DeviceBrokered),
        AuthStrategyIr::None => Ok(AuthStrategy::None),
    }
}

fn sync_mode_to_ir(mode: &SyncMode) -> focus_ir::SyncModeIr {
    match mode {
        SyncMode::Polling { cadence_seconds } => focus_ir::SyncModeIr::Polling {
            cadence_seconds: *cadence_seconds,
        },
        SyncMode::Webhook => focus_ir::SyncModeIr::Webhook,
        SyncMode::Hybrid => focus_ir::SyncModeIr::Hybrid,
    }
}

fn ir_to_sync_mode(ir: &focus_ir::SyncModeIr) -> Result<SyncMode> {
    match ir {
        focus_ir::SyncModeIr::Polling { cadence_seconds } => {
            Ok(SyncMode::Polling {
                cadence_seconds: *cadence_seconds,
            })
        }
        focus_ir::SyncModeIr::Webhook => Ok(SyncMode::Webhook),
        focus_ir::SyncModeIr::Hybrid => Ok(SyncMode::Hybrid),
    }
}

fn parse_tier(s: &str) -> Result<VerificationTier> {
    match s {
        "Official" => Ok(VerificationTier::Official),
        "Verified" => Ok(VerificationTier::Verified),
        "MCPBridged" => Ok(VerificationTier::MCPBridged),
        "Private" => Ok(VerificationTier::Private),
        _ => Err(anyhow!("Unknown tier: {}", s)),
    }
}

/// Convert ConnectorManifest to an IR Document.
pub fn manifest_to_document(manifest: &ConnectorManifest) -> Document {
    let ir = manifest_to_ir(manifest);
    Document {
        version: 1,
        kind: DocKind::Connector,
        id: manifest.id.clone(),
        name: manifest.display_name.clone(),
        body: Body::Connector(ir),
    }
}

/// Convert an IR Document back to ConnectorManifest.
pub fn document_to_manifest(doc: &Document) -> Result<ConnectorManifest> {
    match &doc.body {
        Body::Connector(ir) => ir_to_manifest(ir),
        _ => Err(anyhow!("Expected Connector body, got {:?}", doc.kind)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_to_ir_round_trip() {
        let manifest = ConnectorManifest {
            id: "conn-test".into(),
            version: "1.0.0".into(),
            display_name: "Test Connector".into(),
            auth_strategy: AuthStrategy::OAuth2 {
                scopes: vec!["read".into(), "write".into()],
            },
            sync_mode: SyncMode::Polling { cadence_seconds: 3600 },
            capabilities: vec![ConnectorCapability {
                name: "test_cap".into(),
                params_schema: serde_json::json!({}),
            }],
            entity_types: vec!["test_entity".into()],
            event_types: vec!["test_event".into()],
            tier: VerificationTier::Verified,
            health_indicators: vec!["test_health".into()],
        };

        let ir = manifest_to_ir(&manifest);
        let restored = ir_to_manifest(&ir).expect("Round-trip");

        assert_eq!(manifest.id, restored.id);
        assert_eq!(manifest.version, restored.version);
        assert_eq!(manifest.display_name, restored.display_name);
        assert_eq!(manifest.entity_types.len(), restored.entity_types.len());
    }

    #[test]
    fn test_document_round_trip() {
        let manifest = ConnectorManifest {
            id: "conn-doc-test".into(),
            version: "2.0.0".into(),
            display_name: "Doc Test".into(),
            auth_strategy: AuthStrategy::ApiKey,
            sync_mode: SyncMode::Webhook,
            capabilities: vec![],
            entity_types: vec![],
            event_types: vec![],
            tier: VerificationTier::Official,
            health_indicators: vec![],
        };

        let doc = manifest_to_document(&manifest);
        let restored = document_to_manifest(&doc).expect("Document round-trip");

        assert_eq!(manifest.id, restored.id);
        assert_eq!(manifest.version, restored.version);
    }
}

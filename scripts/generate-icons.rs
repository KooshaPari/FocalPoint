/// Icon sprite generator for FocalPoint UI.
/// Reads Lucide icon manifests, applies brand transforms, generates SVG sprites + TS types.
/// Rust implementation per Phenotype scripting policy.
use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

const BRAND_PRIMARY: &str = "#ff6b3d";      // --color-focal
const BRAND_LIGHT: &str = "#ff8b65";        // --color-focal-light
const BRAND_DARK: &str = "#c94a21";         // --color-focal-dark
const BRAND_OK: &str = "#2bb673";
const BRAND_WARN: &str = "#f5a623";
const BRAND_BLOCK: &str = "#e0544a";
const BRAND_INFO: &str = "#6ea8ff";

/// Icon metadata: name, lucide source, stroke width override, color variant.
struct IconSpec {
    name: &'static str,
    lucide: &'static str,
    stroke: Option<&'static str>,
    color: Option<&'static str>,
}

/// Curated 50+ glyph set covering FocalPoint domains.
fn icon_catalog() -> Vec<IconSpec> {
    vec![
        // Navigation (6)
        IconSpec { name: "nav-home", lucide: "home", stroke: None, color: None },
        IconSpec { name: "nav-focus", lucide: "target", stroke: Some("2.25"), color: None },
        IconSpec { name: "nav-rules", lucide: "shield-check", stroke: None, color: None },
        IconSpec { name: "nav-insights", lucide: "bar-chart-3", stroke: None, color: None },
        IconSpec { name: "nav-connectors", lucide: "plug", stroke: None, color: None },
        IconSpec { name: "nav-settings", lucide: "settings", stroke: None, color: None },

        // Focus modes (8)
        IconSpec { name: "focus-strict", lucide: "lock", stroke: None, color: Some(BRAND_DARK) },
        IconSpec { name: "focus-moderate", lucide: "shield", stroke: None, color: Some(BRAND_PRIMARY) },
        IconSpec { name: "focus-light", lucide: "eye-off", stroke: None, color: Some(BRAND_LIGHT) },
        IconSpec { name: "focus-break", lucide: "coffee", stroke: None, color: Some(BRAND_OK) },
        IconSpec { name: "focus-sleep", lucide: "moon", stroke: None, color: Some(BRAND_INFO) },
        IconSpec { name: "focus-tracking", lucide: "activity", stroke: None, color: None },
        IconSpec { name: "focus-timer", lucide: "clock", stroke: None, color: None },
        IconSpec { name: "focus-pause", lucide: "pause-circle", stroke: None, color: None },

        // Rules & constraints (10)
        IconSpec { name: "rule-app", lucide: "box", stroke: None, color: None },
        IconSpec { name: "rule-time", lucide: "calendar", stroke: None, color: None },
        IconSpec { name: "rule-location", lucide: "map-pin", stroke: None, color: None },
        IconSpec { name: "rule-network", lucide: "wifi", stroke: None, color: None },
        IconSpec { name: "rule-penalty", lucide: "zap", stroke: None, color: Some(BRAND_BLOCK) },
        IconSpec { name: "rule-reward", lucide: "trophy", stroke: None, color: Some(BRAND_OK) },
        IconSpec { name: "rule-allowlist", lucide: "check-circle-2", stroke: None, color: Some(BRAND_OK) },
        IconSpec { name: "rule-blocklist", lucide: "x-circle", stroke: None, color: Some(BRAND_BLOCK) },
        IconSpec { name: "rule-conditional", lucide: "git-branch", stroke: None, color: None },
        IconSpec { name: "rule-escalation", lucide: "arrow-up", stroke: None, color: Some(BRAND_WARN) },

        // Connectors (8)
        IconSpec { name: "connector-canvas", lucide: "book-open", stroke: None, color: None },
        IconSpec { name: "connector-slack", lucide: "send", stroke: None, color: None },
        IconSpec { name: "connector-discord", lucide: "headphones", stroke: None, color: None },
        IconSpec { name: "connector-gmail", lucide: "mail", stroke: None, color: None },
        IconSpec { name: "connector-calendar", lucide: "calendar-days", stroke: None, color: None },
        IconSpec { name: "connector-nfc", lucide: "radio", stroke: None, color: None },
        IconSpec { name: "connector-health", lucide: "heart", stroke: None, color: None },
        IconSpec { name: "connector-api", lucide: "code", stroke: None, color: None },

        // Status & feedback (12)
        IconSpec { name: "status-active", lucide: "check-circle", stroke: None, color: Some(BRAND_OK) },
        IconSpec { name: "status-blocked", lucide: "slash-circle", stroke: None, color: Some(BRAND_BLOCK) },
        IconSpec { name: "status-warning", lucide: "alert-circle", stroke: None, color: Some(BRAND_WARN) },
        IconSpec { name: "status-error", lucide: "x-circle", stroke: None, color: Some(BRAND_BLOCK) },
        IconSpec { name: "status-loading", lucide: "loader", stroke: None, color: Some(BRAND_INFO) },
        IconSpec { name: "status-paused", lucide: "pause", stroke: None, color: None },
        IconSpec { name: "status-offline", lucide: "wifi-off", stroke: None, color: Some(BRAND_WARN) },
        IconSpec { name: "status-syncing", lucide: "refresh-cw", stroke: None, color: Some(BRAND_INFO) },
        IconSpec { name: "status-lock", lucide: "lock", stroke: None, color: None },
        IconSpec { name: "status-unlock", lucide: "unlock", stroke: None, color: None },
        IconSpec { name: "status-verified", lucide: "badge-check", stroke: None, color: Some(BRAND_OK) },
        IconSpec { name: "status-unverified", lucide: "alert-triangle", stroke: None, color: None },

        // Achievements (6)
        IconSpec { name: "achievement-streak", lucide: "flame", stroke: None, color: Some(BRAND_PRIMARY) },
        IconSpec { name: "achievement-milestone", lucide: "award", stroke: None, color: Some(BRAND_PRIMARY) },
        IconSpec { name: "achievement-balanced", lucide: "yin-yang", stroke: None, color: Some(BRAND_OK) },
        IconSpec { name: "achievement-disciplined", lucide: "medal", stroke: None, color: Some(BRAND_DARK) },
        IconSpec { name: "achievement-connected", lucide: "network", stroke: None, color: Some(BRAND_INFO) },
        IconSpec { name: "achievement-early-bird", lucide: "sunrise", stroke: None, color: Some(BRAND_LIGHT) },

        // Common actions (8)
        IconSpec { name: "action-add", lucide: "plus", stroke: None, color: None },
        IconSpec { name: "action-delete", lucide: "trash-2", stroke: None, color: Some(BRAND_BLOCK) },
        IconSpec { name: "action-edit", lucide: "edit-2", stroke: None, color: None },
        IconSpec { name: "action-copy", lucide: "copy", stroke: None, color: None },
        IconSpec { name: "action-download", lucide: "download", stroke: None, color: None },
        IconSpec { name: "action-share", lucide: "share-2", stroke: None, color: None },
        IconSpec { name: "action-info", lucide: "info", stroke: None, color: Some(BRAND_INFO) },
        IconSpec { name: "action-help", lucide: "help-circle", stroke: None, color: Some(BRAND_INFO) },

        // Coachy (the mascot) (3)
        IconSpec { name: "mascot-happy", lucide: "smile", stroke: None, color: Some(BRAND_PRIMARY) },
        IconSpec { name: "mascot-thinking", lucide: "brain", stroke: None, color: Some(BRAND_PRIMARY) },
        IconSpec { name: "mascot-celebrating", lucide: "star", stroke: None, color: Some(BRAND_PRIMARY) },

        // Mobile-specific (2)
        IconSpec { name: "mobile-app", lucide: "smartphone", stroke: None, color: None },
        IconSpec { name: "mobile-permission", lucide: "shield-alert", stroke: None, color: Some(BRAND_WARN) },
    ]
}

/// Generate a single SVG icon with brand transforms applied.
fn generate_icon_svg(spec: &IconSpec) -> String {
    let base_icon = get_lucide_icon_svg(spec.lucide);
    let mut svg = base_icon.to_string();

    // Apply stroke width override if specified
    if let Some(stroke) = spec.stroke {
        svg = svg.replace("stroke-width=\"2\"", &format!("stroke-width=\"{}\"", stroke));
    }

    // Apply color variant if specified
    if let Some(color) = spec.color {
        svg = svg.replace("stroke=\"currentColor\"", &format!("stroke=\"{}\"", color));
        svg = svg.replace("fill=\"currentColor\"", &format!("fill=\"{}\"", color));
    }

    svg
}

/// Stub: in production, fetch from lucide-icons JSON manifest or inline commonly-used icons.
/// This demonstrates the transform pipeline.
fn get_lucide_icon_svg(name: &str) -> &'static str {
    match name {
        "home" => r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><polyline points="9 22 9 12 15 12 15 22"/></svg>"#,
        "target" => r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="1"/><circle cx="12" cy="12" r="5"/><circle cx="12" cy="12" r="9"/></svg>"#,
        "shield-check" => r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/><polyline points="9 12 12 15 15 10"/></svg>"#,
        "bar-chart-3" => r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 3v18h18"/><rect x="7" y="10" width="3" height="7"/><rect x="14" y="4" width="3" height="13"/></svg>"#,
        "plug" => r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="5" r="2"/><path d="M8 7c0 1.1.9 2 2 2h4c1.1 0 2-.9 2-2"/><path d="M9 12v3a3 3 0 0 0 6 0v-3"/><path d="M9 12h6"/></svg>"#,
        "settings" => r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="3"/><path d="M12 1v6m0 6v6M4.22 4.22l4.24 4.24m5.08 5.08l4.24 4.24M1 12h6m6 0h6M4.22 19.78l4.24-4.24m5.08-5.08l4.24-4.24"/></svg>"#,
        "lock" => r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="11" width="18" height="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg>"#,
        "shield" => r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/></svg>"#,
        "eye-off" => r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M9.88 9.88a3 3 0 1 0 4.24 4.24"/><path d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68M6.61 6.61A13.526 13.526 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61"/><line x1="2" y1="2" x2="22" y2="22"/></svg>"#,
        "coffee" => r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M18 8h1a4 4 0 0 1 0 8h-1"/><path d="M2 8h16v9a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V8z"/><line x1="6" y1="1" x2="6" y2="3"/><line x1="10" y1="1" x2="10" y2="3"/><line x1="14" y1="1" x2="14" y2="3"/></svg>"#,
        "moon" => r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/></svg>"#,
        "activity" => r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="22 12 18 12 15 21 9 3 6 12 2 12"/></svg>"#,
        "clock" => r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>"#,
        "pause-circle" => r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="10" y1="8" x2="10" y2="16"/><line x1="14" y1="8" x2="14" y2="16"/></svg>"#,
        "check-circle" => r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 11.08V12a10 10 0 1 1-5.93-9.14"/><polyline points="22 4 12 14.01 9 11.01"/></svg>"#,
        _ => r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="12" cy="12" r="10"/></svg>"#,
    }
}

/// Generate SVG sprite combining all icons.
fn generate_sprite(catalog: &[IconSpec]) -> String {
    let mut sprite = r#"<svg xmlns="http://www.w3.org/2000/svg" width="0" height="0" style="display:none;">
"#.to_string();

    for spec in catalog {
        let svg = generate_icon_svg(spec);
        // Extract the viewBox and content
        sprite.push_str(&format!(
            "  <symbol id=\"{}-icon\" viewBox=\"0 0 24 24\"><!-- {} -->{}</symbol>\n",
            spec.name, spec.lucide, svg
        ));
    }

    sprite.push_str("</svg>\n");
    sprite
}

/// Generate TypeScript type definitions for icon names.
fn generate_icon_types(catalog: &[IconSpec]) -> String {
    let mut types = String::from("// Auto-generated icon type definitions\n\n");
    types.push_str("export type IconName =\n");

    for (i, spec) in catalog.iter().enumerate() {
        let comma = if i < catalog.len() - 1 { " |" } else { ";" };
        types.push_str(&format!("  | \"{}\"{}\n", spec.name, comma));
    }

    types.push_str("\nexport interface IconProps {\n");
    types.push_str("  name: IconName;\n");
    types.push_str("  size?: number;\n");
    types.push_str("  className?: string;\n");
    types.push_str("}\n");

    types
}

fn main() -> Result<()> {
    let catalog = icon_catalog();
    let assets_dir = Path::new("assets");
    let icons_dir = assets_dir.join("icons");
    let individual_dir = icons_dir.join("individual");

    // Create directories
    fs::create_dir_all(&individual_dir)
        .context("Failed to create icons directory")?;

    // Generate sprite
    let sprite = generate_sprite(&catalog);
    fs::write(icons_dir.join("sprite.svg"), &sprite)
        .context("Failed to write sprite.svg")?;

    // Generate individual icon files
    for spec in &catalog {
        let icon_svg = generate_icon_svg(spec);
        let path = individual_dir.join(format!("{}.svg", spec.name));
        fs::write(path, &icon_svg)
            .context(format!("Failed to write icon {}", spec.name))?;
    }

    // Generate TypeScript types
    let types = generate_icon_types(&catalog);
    fs::write(icons_dir.join("sprite.types.ts"), types)
        .context("Failed to write sprite.types.ts")?;

    println!("✓ Generated {} icons", catalog.len());
    println!("✓ Sprite: assets/icons/sprite.svg");
    println!("✓ Individual: assets/icons/individual/*.svg ({})", catalog.len());
    println!("✓ Types: assets/icons/sprite.types.ts");

    // Summary stats
    let sprite_size = fs::metadata(icons_dir.join("sprite.svg"))?
        .len();
    println!("\nSprite size: {} bytes ({:.1} KB)",
             sprite_size, sprite_size as f64 / 1024.0);
    println!("Icon count: {}", catalog.len());

    Ok(())
}

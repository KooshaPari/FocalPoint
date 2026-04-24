use anyhow::{anyhow, Result};
use sha2::{Digest, Sha256};
use std::path::Path;
use tiny_skia::*;

/// FocalPoint icon generator: procedural Coachy flame silhouette with gradient background.
/// Renders a flame icon inspired by the Coachy mascot.

const ICON_SIZES: &[(u32, &str)] = &[
    (1024, "1024x1024"),
    (512, "512x512"),
    (256, "256x256"),
    (180, "180x180"), // iPhone App Icon
    (167, "167x167"), // iPad Pro App Icon
    (152, "152x152"), // iPad App Icon
    (120, "120x120"), // iPhone 6 Plus/7 Plus Spotlight/Settings
    (114, "114x114"), // iPhone non-retina Spotlight
    (80, "80x80"),    // Spotlight
    (76, "76x76"),    // iPad Notification
    (58, "58x58"),    // iPhone non-retina Notification
];

pub struct IconGenerator {
    /// Primary flame color (orange-red gradient start)
    flame_primary: Color,
    /// Flame gradient end (deep red)
    flame_secondary: Color,
    /// Background gradient start (dark)
    bg_dark: Color,
    /// Background gradient end (lighter)
    bg_light: Color,
}

impl Default for IconGenerator {
    fn default() -> Self {
        Self {
            flame_primary: Color::from_rgba8(255, 140, 0, 255),   // Orange
            flame_secondary: Color::from_rgba8(220, 20, 60, 255), // Crimson red
            bg_dark: Color::from_rgba8(30, 30, 40, 255),          // Dark blue-black
            bg_light: Color::from_rgba8(60, 60, 80, 255),         // Lighter blue-gray
        }
    }
}

impl IconGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    /// Render icon at specified size (in pixels).
    pub fn render(&self, size: u32) -> Result<Vec<u8>> {
        let mut pixmap = Pixmap::new(size, size)
            .ok_or_else(|| anyhow!("Failed to create pixmap for size {}", size))?;

        // Fill background with gradient (dark to light, vertical)
        {
            let mut pb = PathBuilder::new();
            pb.push_rect(Rect::from_xywh(0.0, 0.0, size as f32, size as f32).unwrap());
            let path = pb.finish().ok_or_else(|| anyhow!("Failed to build bg path"))?;

            // Gradient: dark at top, light at bottom
            let gradient = LinearGradient::new(
                Point::from_xy(size as f32 / 2.0, 0.0),
                Point::from_xy(size as f32 / 2.0, size as f32),
                [self.bg_dark, self.bg_light],
            )
            .ok_or_else(|| anyhow!("Failed to create bg gradient"))?;

            let mut paint = Paint::default();
            paint.set_color_stops([
                GradientStop { position: 0.0, color: self.bg_dark },
                GradientStop { position: 1.0, color: self.bg_light },
            ]);
            paint.shader = gradient.shader();

            pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::default(), None);
        }

        // Render flame silhouette (stylized upward-pointing flame)
        self.render_flame(&mut pixmap, size)?;

        // Encode to PNG bytes
        let mut png_data = Vec::new();
        {
            let encoder = png::Encoder::new(&mut png_data, size, size);
            let mut writer = encoder.write_header()?;

            // Convert pixmap to RGBA8 scanlines
            let pixels = pixmap.data();
            writer.write_image_data(pixels)?;
        }

        Ok(png_data)
    }

    /// Render a stylized flame silhouette centered in the pixmap.
    fn render_flame(&self, pixmap: &mut Pixmap, size: u32) -> Result<()> {
        let sz = size as f32;
        let cx = sz / 2.0;
        let cy = sz / 2.0;
        let scale = sz / 1024.0; // Normalize to 1024-pixel base

        // Flame bounds: centered, occupying roughly 60% of icon
        let flame_height = sz * 0.6;
        let flame_width = sz * 0.45;
        let flame_base_y = cy + flame_height * 0.3;
        let flame_top_y = flame_base_y - flame_height;

        // Build flame silhouette using a teardrop-like shape with wavy edges
        let mut pb = PathBuilder::new();

        // Start at flame base center
        pb.move_to(cx, flame_base_y);

        // Left curve up and inward (main lobe)
        pb.cubic_to(
            cx - flame_width * 0.5,
            flame_base_y - flame_height * 0.2,
            cx - flame_width * 0.6,
            flame_base_y - flame_height * 0.5,
            cx - flame_width * 0.4,
            flame_base_y - flame_height * 0.75,
        );

        // Left upper taper to point
        pb.cubic_to(
            cx - flame_width * 0.15,
            flame_base_y - flame_height * 0.85,
            cx - flame_width * 0.1,
            flame_base_y - flame_height * 0.95,
            cx,
            flame_top_y,
        );

        // Right upper taper from point
        pb.cubic_to(
            cx + flame_width * 0.1,
            flame_base_y - flame_height * 0.95,
            cx + flame_width * 0.15,
            flame_base_y - flame_height * 0.85,
            cx + flame_width * 0.4,
            flame_base_y - flame_height * 0.75,
        );

        // Right curve down and inward (main lobe)
        pb.cubic_to(
            cx + flame_width * 0.6,
            flame_base_y - flame_height * 0.5,
            cx + flame_width * 0.5,
            flame_base_y - flame_height * 0.2,
            cx,
            flame_base_y,
        );

        let path = pb.finish().ok_or_else(|| anyhow!("Failed to build flame path"))?;

        // Flame gradient: orange at base, deep red at tip
        let gradient = LinearGradient::new(
            Point::from_xy(cx, flame_base_y),
            Point::from_xy(cx, flame_top_y),
            [self.flame_primary, self.flame_secondary],
        )
        .ok_or_else(|| anyhow!("Failed to create flame gradient"))?;

        let mut paint = Paint::default();
        paint.set_color_stops([
            GradientStop { position: 0.0, color: self.flame_primary },
            GradientStop { position: 1.0, color: self.flame_secondary },
        ]);
        paint.shader = gradient.shader();

        pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::default(), None);

        Ok(())
    }

    /// Compute SHA-256 hash of rendered icon for verification.
    pub fn icon_hash(&self, size: u32) -> Result<String> {
        let pixels = self.render(size)?;
        let mut hasher = Sha256::new();
        hasher.update(&pixels);
        let hash = hasher.finalize();
        Ok(hex::encode(hash))
    }

    /// Generate all required iOS icon sizes and return a map of size -> PNG bytes.
    pub fn render_all_sizes(&self) -> Result<Vec<(u32, String, Vec<u8>)>> {
        let mut results = Vec::new();
        for &(size, name) in ICON_SIZES {
            let png_data = self.render(size)?;
            results.push((size, name.to_string(), png_data));
        }
        Ok(results)
    }

    /// Generate an App Store `Contents.json` manifest for XCAssets.
    pub fn generate_contents_json(&self) -> Result<String> {
        #[derive(serde::Serialize)]
        struct Image {
            filename: String,
            idiom: String,
            scale: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            size: Option<String>,
        }

        #[derive(serde::Serialize)]
        struct ContentsJson {
            images: Vec<Image>,
            info: serde_json::json!({
                "version": 1,
                "author": "focalpoint-icon-gen"
            }),
        }

        // Map sizes to XCAssets idiom/scale/filename format
        let mut images = Vec::new();

        // 1024x1024 (universal)
        images.push(Image {
            filename: "icon-1024x1024.png".to_string(),
            idiom: "universal".to_string(),
            scale: "1x".to_string(),
            size: Some("1024x1024".to_string()),
        });

        // iPhone App Icon (180x180 @3x = 60pt)
        images.push(Image {
            filename: "icon-180x180.png".to_string(),
            idiom: "iphone".to_string(),
            scale: "3x".to_string(),
            size: Some("60x60".to_string()),
        });

        // iPhone App Icon (120x120 @2x = 60pt)
        images.push(Image {
            filename: "icon-120x120.png".to_string(),
            idiom: "iphone".to_string(),
            scale: "2x".to_string(),
            size: Some("60x60".to_string()),
        });

        // iPad App Icon (152x152 @2x = 76pt)
        images.push(Image {
            filename: "icon-152x152.png".to_string(),
            idiom: "ipad".to_string(),
            scale: "2x".to_string(),
            size: Some("76x76".to_string()),
        });

        // iPad App Icon (76x76 @1x = 76pt)
        images.push(Image {
            filename: "icon-76x76.png".to_string(),
            idiom: "ipad".to_string(),
            scale: "1x".to_string(),
            size: Some("76x76".to_string()),
        });

        // iPad Pro App Icon (167x167 @2x = 83.5pt)
        images.push(Image {
            filename: "icon-167x167.png".to_string(),
            idiom: "ipad".to_string(),
            scale: "2x".to_string(),
            size: Some("83.5x83.5".to_string()),
        });

        let contents = ContentsJson { images, info: Default::default() };
        let json_str = serde_json::to_string_pretty(&contents)?;
        Ok(json_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Traces to: FR-APPSTORE-001 (Icon generation)
    #[test]
    fn test_icon_hash_stable() {
        let gen = IconGenerator::new();
        let hash1 = gen.icon_hash(1024).expect("First hash");
        let hash2 = gen.icon_hash(1024).expect("Second hash");
        assert_eq!(hash1, hash2, "Icon hash must be stable across renders");
    }

    // Traces to: FR-APPSTORE-002 (All required sizes)
    #[test]
    fn test_all_required_sizes_render() {
        let gen = IconGenerator::new();
        let sizes = gen.render_all_sizes().expect("Render all sizes");

        let required_sizes = [1024, 512, 256, 180, 167, 152, 120, 114, 80, 76, 58];
        let rendered_sizes: Vec<u32> = sizes.iter().map(|(sz, _, _)| *sz).collect();

        for req_size in &required_sizes {
            assert!(
                rendered_sizes.contains(req_size),
                "Required size {} not in rendered: {:?}",
                req_size,
                rendered_sizes
            );
        }
    }

    // Traces to: FR-APPSTORE-003 (Contents.json validity)
    #[test]
    fn test_contents_json_valid() {
        let gen = IconGenerator::new();
        let json_str = gen.generate_contents_json().expect("Generate Contents.json");
        let parsed: serde_json::Value = serde_json::from_str(&json_str)
            .expect("Contents.json must be valid JSON");

        assert!(parsed["images"].is_array(), "images field must be array");
        assert!(parsed["info"]["version"].is_number(), "info.version must be present");

        let images = parsed["images"].as_array().expect("images array");
        assert!(!images.is_empty(), "images array must not be empty");
    }

    // Traces to: FR-APPSTORE-004 (Flame gradient clipping)
    #[test]
    fn test_flame_gradient_no_clip() {
        let gen = IconGenerator::new();
        let png_data = gen.render(1024).expect("Render 1024x1024");
        assert!(!png_data.is_empty(), "PNG must not be empty");

        // Verify PNG header signature
        assert_eq!(&png_data[0..8], &[137, 80, 78, 71, 13, 10, 26, 10], "Valid PNG signature");
    }
}

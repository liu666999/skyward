//! Liquid-glass material tokens for Skyward's software renderer.
//!
//! This module only describes the material. Backdrop sampling and refraction
//! are implemented separately so a theme can fall back to a flat surface on
//! devices where the extra passes would cost too much.

use crate::color::Rgba;

/// Requested quality for glass surfaces.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GlassLevel {
    Off,
    Subtle,
    Standard,
    Rich,
}

/// Parameters for a glass surface. Blur and displacement are in physical
/// pixels after the caller applies display density.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlassMaterial {
    pub tint: Rgba,
    pub blur_radius: f32,
    pub refraction_px: f32,
    pub saturation: f32,
    pub edge_highlight: Rgba,
    pub edge_width_px: f32,
    pub shadow: Rgba,
    pub shadow_blur_px: f32,
}

/// Resolve a quality level into bounded material parameters.
///
/// scale is the display's pixels-per-dp ratio. The values are intentionally
/// capped: recovery screens are full-screen, memory-constrained environments.
pub fn material(level: GlassLevel, dark: bool, scale: f32) -> Option<GlassMaterial> {
    if level == GlassLevel::Off {
        return None;
    }

    let scale = if scale.is_finite() { scale.clamp(0.75, 5.0) } else { 1.0 };
    let (tint, blur_dp, refraction_dp, saturation, edge_alpha, shadow_alpha) = match (level, dark) {
        (GlassLevel::Subtle, false) => (Rgba::rgb(250, 252, 255).with_alpha(176), 3.0, 0.35, 1.02, 96, 20),
        (GlassLevel::Subtle, true) => (Rgba::rgb(35, 42, 56).with_alpha(188), 3.0, 0.25, 1.04, 72, 56),
        (GlassLevel::Standard, false) => (Rgba::rgb(245, 249, 255).with_alpha(150), 7.0, 0.8, 1.08, 132, 28),
        (GlassLevel::Standard, true) => (Rgba::rgb(30, 39, 57).with_alpha(166), 7.0, 0.65, 1.10, 112, 72),
        (GlassLevel::Rich, false) => (Rgba::rgb(241, 247, 255).with_alpha(128), 11.0, 1.25, 1.14, 168, 36),
        (GlassLevel::Rich, true) => (Rgba::rgb(25, 34, 52).with_alpha(146), 11.0, 1.0, 1.16, 144, 88),
        (GlassLevel::Off, _) => return None,
    };

    Some(GlassMaterial {
        tint,
        blur_radius: (blur_dp * scale).min(24.0),
        refraction_px: (refraction_dp * scale).min(6.0),
        saturation,
        edge_highlight: Rgba::rgb(255, 255, 255).with_alpha(edge_alpha),
        edge_width_px: (0.7 * scale).clamp(0.5, 3.0),
        shadow: Rgba::rgb(0, 0, 0).with_alpha(shadow_alpha),
        shadow_blur_px: (4.0 * scale).min(16.0),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn off_disables_glass_without_allocating_a_material() {
        assert!(material(GlassLevel::Off, false, 3.0).is_none());
    }

    #[test]
    fn glass_cost_is_bounded_at_high_density() {
        let rich = material(GlassLevel::Rich, true, 8.0).unwrap();
        assert!(rich.blur_radius <= 24.0);
        assert!(rich.refraction_px <= 6.0);
        assert!(rich.shadow_blur_px <= 16.0);
    }

    #[test]
    fn invalid_density_uses_a_safe_default() {
        let a = material(GlassLevel::Standard, false, f32::NAN).unwrap();
        let b = material(GlassLevel::Standard, false, 1.0).unwrap();
        assert_eq!(a, b);
    }
}

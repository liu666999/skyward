//! Material 3 Expressive-inspired design tokens for Skyward.
//!
//! These tokens are renderer-independent: they describe semantic roles and
//! motion/shape scales without bringing AndroidX, Kotlin, or Compose into the
//! recovery binary. The colours are Skyward's blue-seed palette, not a claim
//! of byte-for-byte parity with Google's dynamic-colour implementation.

use crate::color::Rgba;

/// Semantic colour roles used by Material 3 surfaces and components.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ColorScheme {
    pub primary: Rgba,
    pub on_primary: Rgba,
    pub primary_container: Rgba,
    pub on_primary_container: Rgba,
    pub secondary: Rgba,
    pub on_secondary: Rgba,
    pub secondary_container: Rgba,
    pub on_secondary_container: Rgba,
    pub tertiary: Rgba,
    pub on_tertiary: Rgba,
    pub tertiary_container: Rgba,
    pub on_tertiary_container: Rgba,
    pub error: Rgba,
    pub on_error: Rgba,
    pub error_container: Rgba,
    pub on_error_container: Rgba,
    pub background: Rgba,
    pub on_background: Rgba,
    pub surface: Rgba,
    pub on_surface: Rgba,
    pub surface_variant: Rgba,
    pub on_surface_variant: Rgba,
    pub outline: Rgba,
    pub outline_variant: Rgba,
    pub shadow: Rgba,
    pub scrim: Rgba,
    pub inverse_surface: Rgba,
    pub inverse_on_surface: Rgba,
    pub inverse_primary: Rgba,
    pub surface_dim: Rgba,
    pub surface_bright: Rgba,
    pub surface_container_lowest: Rgba,
    pub surface_container_low: Rgba,
    pub surface_container: Rgba,
    pub surface_container_high: Rgba,
    pub surface_container_highest: Rgba,
}

impl ColorScheme {
    /// Skyward's light blue-seed scheme, organised by Material 3 colour roles.
    pub const fn light() -> Self {
        Self {
            primary: Rgba::rgb(65, 95, 145),
            on_primary: Rgba::rgb(255, 255, 255),
            primary_container: Rgba::rgb(214, 227, 255),
            on_primary_container: Rgba::rgb(0, 27, 72),
            secondary: Rgba::rgb(86, 95, 113),
            on_secondary: Rgba::rgb(255, 255, 255),
            secondary_container: Rgba::rgb(218, 226, 249),
            on_secondary_container: Rgba::rgb(19, 28, 43),
            tertiary: Rgba::rgb(112, 85, 117),
            on_tertiary: Rgba::rgb(255, 255, 255),
            tertiary_container: Rgba::rgb(250, 216, 248),
            on_tertiary_container: Rgba::rgb(40, 19, 45),
            error: Rgba::rgb(186, 26, 26),
            on_error: Rgba::rgb(255, 255, 255),
            error_container: Rgba::rgb(255, 218, 214),
            on_error_container: Rgba::rgb(65, 0, 2),
            background: Rgba::rgb(249, 249, 255),
            on_background: Rgba::rgb(25, 28, 32),
            surface: Rgba::rgb(249, 249, 255),
            on_surface: Rgba::rgb(25, 28, 32),
            surface_variant: Rgba::rgb(225, 226, 236),
            on_surface_variant: Rgba::rgb(68, 71, 78),
            outline: Rgba::rgb(117, 119, 127),
            outline_variant: Rgba::rgb(197, 198, 208),
            shadow: Rgba::rgb(0, 0, 0),
            scrim: Rgba::rgb(0, 0, 0),
            inverse_surface: Rgba::rgb(46, 48, 53),
            inverse_on_surface: Rgba::rgb(240, 240, 247),
            inverse_primary: Rgba::rgb(171, 199, 255),
            surface_dim: Rgba::rgb(217, 217, 224),
            surface_bright: Rgba::rgb(249, 249, 255),
            surface_container_lowest: Rgba::rgb(255, 255, 255),
            surface_container_low: Rgba::rgb(243, 243, 250),
            surface_container: Rgba::rgb(237, 237, 244),
            surface_container_high: Rgba::rgb(231, 232, 239),
            surface_container_highest: Rgba::rgb(226, 226, 233),
        }
    }

    /// Dark counterpart of Skyward's blue-seed scheme.
    pub const fn dark() -> Self {
        Self {
            primary: Rgba::rgb(171, 199, 255),
            on_primary: Rgba::rgb(10, 47, 91),
            primary_container: Rgba::rgb(40, 71, 117),
            on_primary_container: Rgba::rgb(214, 227, 255),
            secondary: Rgba::rgb(190, 198, 220),
            on_secondary: Rgba::rgb(40, 49, 65),
            secondary_container: Rgba::rgb(62, 71, 88),
            on_secondary_container: Rgba::rgb(218, 226, 249),
            tertiary: Rgba::rgb(223, 187, 222),
            on_tertiary: Rgba::rgb(63, 41, 68),
            tertiary_container: Rgba::rgb(87, 63, 92),
            on_tertiary_container: Rgba::rgb(250, 216, 248),
            error: Rgba::rgb(255, 180, 171),
            on_error: Rgba::rgb(105, 0, 5),
            error_container: Rgba::rgb(147, 0, 10),
            on_error_container: Rgba::rgb(255, 218, 214),
            background: Rgba::rgb(17, 19, 24),
            on_background: Rgba::rgb(226, 226, 233),
            surface: Rgba::rgb(17, 19, 24),
            on_surface: Rgba::rgb(226, 226, 233),
            surface_variant: Rgba::rgb(68, 71, 78),
            on_surface_variant: Rgba::rgb(197, 198, 208),
            outline: Rgba::rgb(143, 145, 153),
            outline_variant: Rgba::rgb(68, 71, 78),
            shadow: Rgba::rgb(0, 0, 0),
            scrim: Rgba::rgb(0, 0, 0),
            inverse_surface: Rgba::rgb(226, 226, 233),
            inverse_on_surface: Rgba::rgb(46, 48, 53),
            inverse_primary: Rgba::rgb(65, 95, 145),
            surface_dim: Rgba::rgb(17, 19, 24),
            surface_bright: Rgba::rgb(55, 57, 63),
            surface_container_lowest: Rgba::rgb(12, 14, 19),
            surface_container_low: Rgba::rgb(25, 28, 32),
            surface_container: Rgba::rgb(29, 32, 36),
            surface_container_high: Rgba::rgb(39, 42, 48),
            surface_container_highest: Rgba::rgb(50, 53, 59),
        }
    }
}

/// Corner sizes in dp. Components choose a role rather than hard-coding a radius.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ShapeScale {
    pub none: f32,
    pub extra_small: f32,
    pub small: f32,
    pub medium: f32,
    pub large: f32,
    pub extra_large: f32,
    pub full: f32,
}

impl ShapeScale {
    /// Existing Skyward geometry, expressed as named shape roles.
    pub const STOCK: Self = Self {
        none: 0.0,
        extra_small: 4.0,
        small: 8.0,
        medium: 12.0,
        large: 16.0,
        extra_large: 20.0,
        full: 999.0,
    };

    pub const EXPRESSIVE: Self = Self {
        none: 0.0,
        extra_small: 4.0,
        small: 8.0,
        medium: 12.0,
        large: 16.0,
        extra_large: 28.0,
        full: 999.0,
    };
}

/// Named typography role. Sizes and line heights are in sp.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypeStyle {
    pub size_sp: f32,
    pub line_height_sp: f32,
    pub letter_spacing_sp: f32,
    pub weight: u16,
}

/// Material type scale, kept independent from any particular font file.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TypeScale {
    pub display_large: TypeStyle,
    pub display_medium: TypeStyle,
    pub display_small: TypeStyle,
    pub headline_large: TypeStyle,
    pub headline_medium: TypeStyle,
    pub headline_small: TypeStyle,
    pub title_large: TypeStyle,
    pub title_medium: TypeStyle,
    pub title_small: TypeStyle,
    pub body_large: TypeStyle,
    pub body_medium: TypeStyle,
    pub body_small: TypeStyle,
    pub label_large: TypeStyle,
    pub label_medium: TypeStyle,
    pub label_small: TypeStyle,
}

impl TypeScale {
    pub const EXPRESSIVE: Self = Self {
        display_large: TypeStyle { size_sp: 57.0, line_height_sp: 64.0, letter_spacing_sp: -0.25, weight: 400 },
        display_medium: TypeStyle { size_sp: 45.0, line_height_sp: 52.0, letter_spacing_sp: 0.0, weight: 400 },
        display_small: TypeStyle { size_sp: 36.0, line_height_sp: 44.0, letter_spacing_sp: 0.0, weight: 400 },
        headline_large: TypeStyle { size_sp: 32.0, line_height_sp: 40.0, letter_spacing_sp: 0.0, weight: 400 },
        headline_medium: TypeStyle { size_sp: 28.0, line_height_sp: 36.0, letter_spacing_sp: 0.0, weight: 400 },
        headline_small: TypeStyle { size_sp: 24.0, line_height_sp: 32.0, letter_spacing_sp: 0.0, weight: 400 },
        title_large: TypeStyle { size_sp: 22.0, line_height_sp: 28.0, letter_spacing_sp: 0.0, weight: 500 },
        title_medium: TypeStyle { size_sp: 16.0, line_height_sp: 24.0, letter_spacing_sp: 0.15, weight: 500 },
        title_small: TypeStyle { size_sp: 14.0, line_height_sp: 20.0, letter_spacing_sp: 0.1, weight: 500 },
        body_large: TypeStyle { size_sp: 16.0, line_height_sp: 24.0, letter_spacing_sp: 0.5, weight: 400 },
        body_medium: TypeStyle { size_sp: 14.0, line_height_sp: 20.0, letter_spacing_sp: 0.25, weight: 400 },
        body_small: TypeStyle { size_sp: 12.0, line_height_sp: 16.0, letter_spacing_sp: 0.4, weight: 400 },
        label_large: TypeStyle { size_sp: 14.0, line_height_sp: 20.0, letter_spacing_sp: 0.1, weight: 600 },
        label_medium: TypeStyle { size_sp: 12.0, line_height_sp: 16.0, letter_spacing_sp: 0.5, weight: 600 },
        label_small: TypeStyle { size_sp: 11.0, line_height_sp: 16.0, letter_spacing_sp: 0.5, weight: 600 },
    };
}

/// Timing values in milliseconds. The spring periods are seconds.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MotionScheme {
    pub short: u64,
    pub medium: u64,
    pub long: u64,
    pub expressive: u64,
    pub spatial_spring_period: f32,
    pub effects_spring_period: f32,
}

impl MotionScheme {
    pub const EXPRESSIVE: Self = Self {
        short: 150,
        medium: 300,
        long: 500,
        expressive: 700,
        spatial_spring_period: 0.45,
        effects_spring_period: 0.30,
    };
}

/// The complete renderer-neutral M3 Expressive-inspired theme.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Theme {
    pub colors: ColorScheme,
    pub shapes: ShapeScale,
    pub typography: TypeScale,
    pub motion: MotionScheme,
    pub dark: bool,
}

impl Theme {
    pub const fn light() -> Self {
        Self { colors: ColorScheme::light(), shapes: ShapeScale::EXPRESSIVE, typography: TypeScale::EXPRESSIVE, motion: MotionScheme::EXPRESSIVE, dark: false }
    }

    pub const fn dark() -> Self {
        Self { colors: ColorScheme::dark(), shapes: ShapeScale::EXPRESSIVE, typography: TypeScale::EXPRESSIVE, motion: MotionScheme::EXPRESSIVE, dark: true }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_and_dark_schemes_keep_semantic_roles_distinct() {
        let light = Theme::light();
        let dark = Theme::dark();
        assert!(!light.dark);
        assert!(dark.dark);
        assert_ne!(light.colors.surface, dark.colors.surface);
        assert_eq!(light.colors.on_primary, Rgba::rgb(255, 255, 255));
        assert_eq!(dark.colors.on_primary, Rgba::rgb(10, 47, 91));
    }

    #[test]
    fn expressive_tokens_have_ordered_corner_sizes() {
        let s = ShapeScale::EXPRESSIVE;
        assert!(s.none <= s.extra_small);
        assert!(s.extra_small <= s.small);
        assert!(s.small <= s.medium);
        assert!(s.medium <= s.large);
        assert!(s.large <= s.extra_large);
        assert!(s.extra_large <= s.full);
    }

    fn luminance(c: Rgba) -> f32 {
        fn linear(v: u8) -> f32 {
            let s = v as f32 / 255.0;
            if s <= 0.04045 { s / 12.92 } else { ((s + 0.055) / 1.055).powf(2.4) }
        }
        0.2126 * linear(c.r) + 0.7152 * linear(c.g) + 0.0722 * linear(c.b)
    }

    fn contrast(a: Rgba, b: Rgba) -> f32 {
        let (hi, lo) = {
            let x = luminance(a);
            let y = luminance(b);
            (x.max(y), x.min(y))
        };
        (hi + 0.05) / (lo + 0.05)
    }

    #[test]
    fn primary_text_and_controls_keep_readable_contrast_in_both_schemes() {
        for colors in [ColorScheme::light(), ColorScheme::dark()] {
            assert!(contrast(colors.on_surface, colors.surface) >= 7.0);
            assert!(contrast(colors.on_primary, colors.primary) >= 4.5);
            assert!(contrast(colors.on_primary_container, colors.primary_container) >= 4.5);
            assert!(contrast(colors.on_error, colors.error) >= 4.5);
        }
    }

    #[test]
    fn typography_has_valid_line_heights() {
        let t = TypeScale::EXPRESSIVE;
        for style in [t.display_large, t.display_medium, t.display_small, t.headline_large, t.headline_medium, t.headline_small, t.title_large, t.title_medium, t.title_small, t.body_large, t.body_medium, t.body_small, t.label_large, t.label_medium, t.label_small] {
            assert!(style.size_sp > 0.0);
            assert!(style.line_height_sp >= style.size_sp);
        }
    }
}

use crate::color::{over, Rgba};
use crate::glow::{self, Glow};
use crate::expressive::{self, ColorScheme, MotionScheme, ShapeScale, TypeScale};

#[derive(Clone, Copy)]
pub struct Palette {
    pub bg: Rgba,
    pub card: Rgba,
    pub text: Rgba,
    pub subtitle: Rgba,
    pub value: Rgba,
    pub section: Rgba,
    pub chevron: Rgba,
    pub accent: Rgba,
    /// Foreground ink placed directly on the accent role.
    pub on_accent: Rgba,
    /// Inverse surface used by transient messages such as toasts.
    pub inverse_surface: Rgba,
    pub inverse_text: Rgba,
    /// Gesture edge treatment and keyboard semantic roles.
    pub backdrop: Rgba,
    pub backdrop_ink: Rgba,
    pub rim: Rgba,
    pub chip_fill: Rgba,
    pub chip_rim: Rgba,
    pub keyboard_panel: Rgba,
    pub keyboard_key: Rgba,
    pub keyboard_side: Rgba,
    pub keyboard_ink: Rgba,
    pub keyboard_shadow: Rgba,
    pub track: Rgba,
    pub switch_off: Rgba,
    pub search: Rgba,
    pub search_icon: Rgba,
    pub placeholder: Rgba,
    pub badge: Rgba,
    pub avatar: Rgba,
    pub avatar_glyph: Rgba,
    pub status: Rgba,
    pub battery: BatteryInk,
    pub mono_icon: Rgba,
    pub danger: Rgba,
    pub on_danger: Rgba,
    pub danger_soft: Rgba,
    /// Titles and values on the parameters page and captions inside cards, a
    /// step softer than `text`.
    pub info: Rgba,
    /// Line icons on the parameters page.
    pub outline: Rgba,
    /// Captions under the About page's spec values.
    pub spec_label: Rgba,
    /// Laid over the card behind a row a finger holds down, under the
    /// row's words and icon.
    pub row_press: Rgba,
    /// Laid over the `<`, the search page's round button and the home
    /// page's search bar while a finger holds them down.
    pub button_press: Rgba,
    /// The round button at the end of a WLAN network's row, and the arrow
    /// in it.
    pub round: Rgba,
    pub round_mark: Rgba,
    /// The arrows up and down after the value of a row that opens a list of
    /// choices.
    pub updown: Rgba,
    /// A list of choices open over the page, and the shadow it casts where
    /// the shadow is whole.
    pub popup: Rgba,
    pub shadow: Rgba,
}

/// The status bar battery's inks, partly see-through as the stock status
/// bar's are, so they sit on whatever page is under them.
#[derive(Clone, Copy)]
pub struct BatteryInk {
    /// What the level fills while the cable is in, charging or full.
    pub charging: Rgba,
    /// What a low level fills with the cable out.
    pub low: Rgba,
    /// What any other level fills; the digits are cut out of it.
    pub level: Rgba,
    /// The empty part, cap included.
    pub empty: Rgba,
    pub bolt: Rgba,
}

/// The About page in a theme: the moving field behind it, the version
/// line under the wordmark, and the cards that scroll over the field.
#[derive(Clone, Copy)]
pub struct Hero {
    pub glow: &'static Glow,
    /// The version line: black or white, partly see-through.
    pub version: Rgba,
    pub card: Rgba,
    pub title: Rgba,
    pub value: Rgba,
    pub chevron: Rgba,
}

/// Layout metrics measured on a Xiaomi 17 Ultra's stock settings, in dp,
/// converted at the panel's density.
#[derive(Clone, Copy)]
pub struct Metrics {
    pub w: u32,
    pub h: u32,
    pub px_per_dp: f32,
    pub margin: f32,
    pub card_radius: f32,
    pub card_gap: f32,
    pub section_block: f32,
    pub section_label_top: f32,
    pub row_h: f32,
    pub row_pad_v: f32,
    pub inner: f32,
    pub icon: f32,
    pub icon_gap: f32,
    pub title_px: f32,
    pub title_x: f32,
    pub title_top: f32,
    /// Back chevron: tip x, centre y, width, height and stroke.
    pub back_x: f32,
    pub back_cy: f32,
    pub back_w: f32,
    pub back_h: f32,
    pub back_stroke: f32,
    pub stroke: f32,
    pub content_top: f32,
    pub search_h: f32,
    pub search_text_inset: f32,
    pub after_search: f32,
    pub row_title_px: f32,
    pub sub_px: f32,
    pub sub_gap: f32,
    pub sub_pitch: f32,
    pub label_px: f32,
    pub account_title_px: f32,
    pub account_row_h: f32,
    pub avatar: f32,
    pub avatar_inset: f32,
    pub badge_px: f32,
    pub badge_w: f32,
    pub badge_h: f32,
    pub chevron_w: f32,
    pub chevron_h: f32,
    pub chevron_inset: f32,
    pub chevron_stroke: f32,
    pub value_gap: f32,
    pub switch_w: f32,
    pub switch_h: f32,
    pub knob: f32,
    pub slider_h: f32,
    pub slider_knob: f32,
    pub status_x: f32,
    /// Where the battery ends, from the right edge; with the cable in, the
    /// bolt's box ends at `status_bolt_right` instead and the battery moves
    /// left.
    pub status_right: f32,
    pub status_bolt_right: f32,
    pub status_cy: f32,
    pub status_px: f32,
}

impl Metrics {
    pub fn dp(&self, v: f32) -> f32 {
        v * self.px_per_dp
    }
}

#[derive(Clone, Copy)]
pub struct Theme {
    pub pal: Palette,
    pub hero: Hero,
    pub m: Metrics,
    pub shapes: ShapeScale,
    /// Present only when the M3 Expressive typography scale is selected.
    pub typography: Option<TypeScale>,
    /// Present only when the M3 Expressive motion scheme is selected.
    pub motion: Option<MotionScheme>,
    pub dark: bool,
}

impl Theme {
    /// `px_per_dp` is the panel density over 160, e.g. 3.0 at 480 dpi.
    pub fn light(w: u32, h: u32, px_per_dp: f32) -> Self {
        Theme { pal: LIGHT, hero: LIGHT_HERO, m: metrics(w, h, px_per_dp), shapes: ShapeScale::STOCK, typography: None, motion: None, dark: false }
    }

    pub fn dark(w: u32, h: u32, px_per_dp: f32) -> Self {
        Theme { pal: DARK, hero: DARK_HERO, m: metrics(w, h, px_per_dp), shapes: ShapeScale::STOCK, typography: None, motion: None, dark: true }
    }

    /// Material 3 Expressive palette adapted to Skyward's existing widgets.
    /// Layout, hit targets, status inks and recovery behaviour stay unchanged.
    pub fn expressive(w: u32, h: u32, px_per_dp: f32, dark: bool) -> Self {
        let mut theme = if dark { Self::dark(w, h, px_per_dp) } else { Self::light(w, h, px_per_dp) };
        let c: ColorScheme = if dark { expressive::Theme::dark().colors } else { expressive::Theme::light().colors };
        theme.pal = Palette {
            bg: c.background,
            card: c.surface_container,
            text: c.on_surface,
            subtitle: c.on_surface_variant,
            value: c.on_surface_variant,
            section: c.primary,
            chevron: c.outline,
            accent: c.primary,
            on_accent: c.on_primary,
            inverse_surface: c.inverse_surface,
            inverse_text: c.inverse_on_surface,
            backdrop: c.surface_container_high,
            backdrop_ink: c.on_surface,
            rim: c.primary,
            chip_fill: c.surface_container_highest,
            chip_rim: c.outline_variant,
            keyboard_panel: c.surface_dim,
            keyboard_key: c.surface_container_lowest,
            keyboard_side: c.surface_container_high,
            keyboard_ink: c.on_surface,
            keyboard_shadow: c.shadow.with_alpha(if dark { 100 } else { 32 }),
            track: c.surface_container_high,
            switch_off: c.surface_variant,
            search: c.surface_container_high,
            search_icon: c.on_surface_variant,
            placeholder: c.on_surface_variant.with_alpha(150),
            badge: c.primary_container,
            avatar: c.primary_container,
            avatar_glyph: c.on_primary_container,
            status: c.on_surface,
            battery: BatteryInk { charging: c.primary, low: c.error, level: c.on_surface, empty: c.outline_variant, bolt: c.on_primary },
            mono_icon: c.on_surface_variant,
            danger: c.error,
            on_danger: c.on_error,
            danger_soft: c.error_container,
            info: c.on_surface_variant,
            outline: c.outline,
            spec_label: c.on_surface_variant,
            row_press: c.primary_container.with_alpha(if dark { 76 } else { 54 }),
            button_press: c.primary_container.with_alpha(if dark { 92 } else { 68 }),
            round: c.surface_container_high,
            round_mark: c.on_surface_variant,
            updown: c.surface_variant,
            popup: c.surface_container_high,
            shadow: c.shadow.with_alpha(if dark { 90 } else { 36 }),
            ..theme.pal
        };
        theme.shapes = ShapeScale::EXPRESSIVE;
        theme.typography = Some(TypeScale::EXPRESSIVE);
        theme.motion = Some(MotionScheme::EXPRESSIVE);
        theme.m.card_radius = theme.m.dp(theme.shapes.extra_large);
        // Apply the M3 type roles to existing layout slots without changing
        // row heights or touch targets.
        theme.m.title_px = theme.m.dp(TypeScale::EXPRESSIVE.headline_large.size_sp);
        theme.m.row_title_px = theme.m.dp(TypeScale::EXPRESSIVE.title_medium.size_sp);
        theme.m.account_title_px = theme.m.dp(TypeScale::EXPRESSIVE.title_medium.size_sp);
        theme.m.sub_px = theme.m.dp(TypeScale::EXPRESSIVE.body_medium.size_sp);
        theme.m.label_px = theme.m.dp(TypeScale::EXPRESSIVE.label_medium.size_sp);
        theme.m.badge_px = theme.m.dp(TypeScale::EXPRESSIVE.label_small.size_sp);
        theme.hero.card = if dark { c.surface_container_high.with_alpha(230) } else { c.surface_container_lowest };
        theme.hero.title = c.on_surface;
        theme.hero.value = c.on_surface_variant;
        theme.hero.chevron = c.outline;
        theme
    }

    /// This theme with every card as it shows behind a row held down.
    pub fn pressed(&self) -> Theme {
        let mut t = *self;
        t.pal.card = over(self.pal.row_press, self.pal.card);
        t.hero.card = over(self.pal.row_press, self.hero.card);
        t
    }
}

fn metrics(w: u32, h: u32, px_per_dp: f32) -> Metrics {
    let dp = |v: f32| v * px_per_dp;
    Metrics {
        w,
        h,
        px_per_dp,
        margin: dp(12.0),
        card_radius: dp(20.0),
        card_gap: dp(12.0),
        section_block: dp(45.67),
        section_label_top: dp(23.0),
        row_h: dp(55.67),
        row_pad_v: dp(17.0),
        inner: dp(16.0),
        icon: dp(28.0),
        icon_gap: dp(20.67),
        title_px: dp(32.0),
        title_x: dp(28.6),
        title_top: dp(117.7),
        back_x: dp(26.3),
        back_cy: dp(75.7),
        back_w: dp(11.7),
        back_h: dp(21.0),
        back_stroke: dp(1.8),
        stroke: dp(2.0),
        content_top: dp(167.7),
        search_h: dp(43.7),
        search_text_inset: dp(43.7),
        after_search: dp(12.7),
        row_title_px: dp(16.9),
        sub_px: dp(14.0),
        sub_gap: dp(6.7),
        sub_pitch: dp(18.7),
        label_px: dp(13.2),
        account_title_px: dp(17.1),
        account_row_h: dp(69.7),
        avatar: dp(34.0),
        avatar_inset: dp(12.8),
        badge_px: dp(14.5),
        badge_w: dp(72.9),
        badge_h: dp(28.9),
        chevron_w: dp(7.7),
        chevron_h: dp(13.3),
        chevron_inset: dp(17.3),
        chevron_stroke: dp(1.15),
        value_gap: dp(11.3),
        switch_w: dp(48.7),
        switch_h: dp(28.0),
        knob: dp(19.3),
        slider_h: dp(27.5),
        slider_knob: dp(18.7),
        status_x: dp(37.0),
        status_right: dp(40.0),
        status_bolt_right: dp(36.0),
        status_cy: dp(25.7),
        status_px: dp(14.1),
    }
}

// Sampled from the stock Settings on a Xiaomi 17 Ultra.
const LIGHT: Palette = Palette {
    bg: Rgba::rgb(243, 243, 243),
    card: Rgba::rgb(255, 255, 255),
    text: Rgba::rgb(0, 0, 0),
    subtitle: Rgba::rgb(102, 102, 102),
    value: Rgba::rgb(153, 153, 153),
    section: Rgba::rgb(143, 156, 174),
    chevron: Rgba::rgb(178, 178, 178),
    accent: Rgba::rgb(74, 128, 247),
    on_accent: Rgba::rgb(255, 255, 255),
    inverse_surface: Rgba::rgb(30, 30, 30).with_alpha(235),
    inverse_text: Rgba::rgb(255, 255, 255),
    backdrop: Rgba::rgb(48, 48, 48),
    backdrop_ink: Rgba::rgb(255, 255, 255),
    rim: Rgba::rgb(117, 170, 246),
    chip_fill: Rgba::rgb(248, 248, 248),
    chip_rim: Rgba::rgb(255, 255, 255),
    keyboard_panel: Rgba::rgb(224, 226, 231),
    keyboard_key: Rgba::rgb(255, 255, 255),
    keyboard_side: Rgba::rgb(193, 198, 207),
    keyboard_ink: Rgba::rgb(0, 0, 0),
    keyboard_shadow: Rgba::rgb(138, 141, 147),
    track: Rgba::rgb(240, 240, 240),
    switch_off: Rgba::rgb(229, 229, 229),
    search: Rgba::rgb(229, 229, 229),
    search_icon: Rgba::rgb(161, 161, 161),
    // Black at 30%: 160 over the search field, and darker blue where it
    // crosses the cursor.
    placeholder: Rgba::rgb(0, 0, 0).with_alpha(77),
    badge: Rgba::rgb(235, 242, 254),
    avatar: Rgba::rgb(221, 223, 226),
    avatar_glyph: Rgba::rgb(255, 255, 255),
    status: Rgba::rgb(61, 61, 61),
    // The stock status bar's dark icons, sampled over light pages.
    battery: BatteryInk {
        charging: Rgba::rgb(29, 205, 58),
        low: Rgba::rgb(250, 56, 46),
        level: Rgba::rgb(0, 0, 0).with_alpha(196),
        empty: Rgba::rgb(0, 0, 0).with_alpha(77),
        bolt: Rgba::rgb(0, 0, 0).with_alpha(172),
    },
    mono_icon: Rgba::rgb(179, 179, 179),
    danger: Rgba::rgb(230, 66, 60),
    on_danger: Rgba::rgb(255, 255, 255),
    danger_soft: Rgba::rgb(253, 236, 235),
    info: Rgba::rgb(51, 51, 51),
    outline: Rgba::rgb(102, 102, 102),
    spec_label: Rgba::rgb(127, 127, 127),
    // A white card held down goes to 225; the page under `<` goes from 243
    // to 219, the search field and its round button from 229 to 206.
    row_press: Rgba::rgb(0, 0, 0).with_alpha(30),
    button_press: Rgba::rgb(0, 0, 0).with_alpha(26),
    // Sampled from the stock WLAN page.
    round: Rgba::rgb(242, 242, 242),
    round_mark: Rgba::rgb(169, 169, 169),
    // Sampled from the stock page that adds a network; the shadow as the
    // grey it leaves on the page beside the list is fitted.
    updown: Rgba::rgb(214, 214, 214),
    popup: Rgba::rgb(255, 255, 255),
    shadow: Rgba::rgb(0, 0, 0).with_alpha(27),
};

const DARK: Palette = Palette {
    bg: Rgba::rgb(0, 0, 0),
    card: Rgba::rgb(28, 28, 28),
    text: Rgba::rgb(237, 237, 237),
    subtitle: Rgba::rgb(140, 140, 140),
    value: Rgba::rgb(128, 128, 128),
    section: Rgba::rgb(107, 113, 134),
    chevron: Rgba::rgb(77, 77, 77),
    accent: Rgba::rgb(84, 136, 248),
    on_accent: Rgba::rgb(255, 255, 255),
    inverse_surface: Rgba::rgb(30, 30, 30).with_alpha(235),
    inverse_text: Rgba::rgb(255, 255, 255),
    backdrop: Rgba::rgb(48, 48, 48),
    backdrop_ink: Rgba::rgb(255, 255, 255),
    rim: Rgba::rgb(117, 170, 246),
    chip_fill: Rgba::rgb(38, 38, 38),
    chip_rim: Rgba::rgb(62, 62, 62),
    keyboard_panel: Rgba::rgb(18, 18, 18),
    keyboard_key: Rgba::rgb(58, 58, 58),
    keyboard_side: Rgba::rgb(38, 38, 38),
    keyboard_ink: Rgba::rgb(237, 237, 237),
    keyboard_shadow: Rgba::rgb(0, 0, 0),
    track: Rgba::rgb(46, 46, 46),
    switch_off: Rgba::rgb(58, 58, 58),
    search: Rgba::rgb(28, 28, 28),
    search_icon: Rgba::rgb(110, 110, 110),
    // White at 36%: 110 over the search field.
    placeholder: Rgba::rgb(255, 255, 255).with_alpha(92),
    badge: Rgba::rgb(29, 40, 64),
    avatar: Rgba::rgb(51, 51, 51),
    avatar_glyph: Rgba::rgb(140, 140, 140),
    status: Rgba::rgb(224, 224, 224),
    // The stock status bar's light icons, sampled over black and over the
    // lock screen's wallpaper.
    battery: BatteryInk {
        // As the stock Settings fill it over black and over a dark grey: the
        // green they fill with over light pages too.
        charging: Rgba::rgb(29, 205, 58),
        low: Rgba::rgb(230, 75, 59),
        level: Rgba::rgb(255, 255, 255).with_alpha(232),
        empty: Rgba::rgb(255, 255, 255).with_alpha(94),
        // As the stock Settings draw it over black and over a dark grey.
        bolt: Rgba::rgb(255, 255, 255).with_alpha(207),
    },
    mono_icon: Rgba::rgb(110, 110, 110),
    danger: Rgba::rgb(255, 92, 84),
    on_danger: Rgba::rgb(255, 255, 255),
    danger_soft: Rgba::rgb(52, 24, 24),
    info: Rgba::rgb(214, 214, 214),
    outline: Rgba::rgb(140, 140, 140),
    spec_label: Rgba::rgb(128, 128, 128),
    // White as strong as the black in the light theme.
    row_press: Rgba::rgb(255, 255, 255).with_alpha(30),
    button_press: Rgba::rgb(255, 255, 255).with_alpha(26),
    // As far from the card as the light theme's are from its own.
    round: Rgba::rgb(41, 41, 41),
    round_mark: Rgba::rgb(118, 118, 118),
    // The arrows as much softer than the chevron as in the light theme; the
    // list a step lighter than the cards it opens over, and its shadow
    // darker, for it to show on them.
    updown: Rgba::rgb(54, 54, 54),
    popup: Rgba::rgb(41, 41, 41),
    shadow: Rgba::rgb(0, 0, 0).with_alpha(77),
};

// Sampled from the stock「我的设备」page: in light mode the cards are
// the ordinary settings cards; in dark mode they are white at low opacity
// over the field.
const LIGHT_HERO: Hero = Hero {
    glow: &glow::LIGHT,
    version: Rgba::rgb(0, 0, 0).with_alpha(153),
    card: LIGHT.card,
    title: LIGHT.text,
    value: LIGHT.value,
    chevron: LIGHT.chevron,
};

const DARK_HERO: Hero = Hero {
    glow: &glow::DARK,
    version: Rgba::rgb(255, 255, 255).with_alpha(153),
    card: Rgba::rgb(255, 255, 255).with_alpha(36),
    title: Rgba::rgb(244, 245, 248),
    value: Rgba::rgb(157, 161, 188),
    chevron: Rgba::rgb(118, 122, 160),
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expressive_component_roles_follow_m3_semantics() {
        for dark in [false, true] {
            let theme = Theme::expressive(1200, 2608, 3.0, dark);
            let colors = if dark { ColorScheme::dark() } else { ColorScheme::light() };
            assert_eq!(theme.pal.on_accent, colors.on_primary);
            assert_eq!(theme.pal.on_danger, colors.on_error);
            assert_eq!(theme.pal.backdrop_ink, colors.on_surface);
            assert_eq!(theme.pal.inverse_surface, colors.inverse_surface);
            assert_eq!(theme.pal.inverse_text, colors.inverse_on_surface);
            assert_eq!(theme.pal.backdrop, colors.surface_container_high);
            assert_eq!(theme.pal.rim, colors.primary);
            assert_eq!(theme.pal.chip_fill, colors.surface_container_highest);
            assert_eq!(theme.pal.chip_rim, colors.outline_variant);
            assert_eq!(theme.pal.keyboard_panel, colors.surface_dim);
            assert_eq!(theme.pal.keyboard_key, colors.surface_container_lowest);
            assert_eq!(theme.pal.keyboard_side, colors.surface_container_high);
            assert_eq!(theme.pal.keyboard_ink, colors.on_surface);
        }
    }

    #[test]
    fn stock_component_roles_keep_the_original_light_and_dark_values() {
        let light = Theme::light(1200, 2608, 3.0).pal;
        let dark = Theme::dark(1200, 2608, 3.0).pal;
        assert_eq!(light.on_accent, Rgba::rgb(255, 255, 255));
        assert_eq!(light.on_danger, Rgba::rgb(255, 255, 255));
        assert_eq!(light.inverse_surface, Rgba::rgb(30, 30, 30).with_alpha(235));
        assert_eq!(dark.inverse_surface, Rgba::rgb(30, 30, 30).with_alpha(235));
        assert_eq!(dark.backdrop_ink, Rgba::rgb(255, 255, 255));
        assert_eq!(light.chip_fill, Rgba::rgb(248, 248, 248));
        assert_eq!(light.chip_rim, Rgba::rgb(255, 255, 255));
        assert_eq!(light.keyboard_panel, Rgba::rgb(224, 226, 231));
        assert_eq!(dark.chip_fill, Rgba::rgb(38, 38, 38));
        assert_eq!(dark.chip_rim, Rgba::rgb(62, 62, 62));
        assert_eq!(dark.keyboard_panel, Rgba::rgb(18, 18, 18));
    }

    #[test]
    fn expressive_palette_maps_semantic_roles_and_keeps_device_geometry() {
        let stock = Theme::light(1200, 2608, 3.0);
        let expressive = Theme::expressive(1200, 2608, 3.0, false);
        let roles = ColorScheme::light();
        assert_eq!(expressive.pal.bg, roles.background);
        assert_eq!(expressive.pal.accent, roles.primary);
        assert_eq!(expressive.pal.danger, roles.error);
        assert_eq!(expressive.pal.avatar_glyph, roles.on_primary_container);
        assert_eq!((expressive.m.w, expressive.m.h), (stock.m.w, stock.m.h));
        assert!(expressive.m.card_radius > stock.m.card_radius);
        assert_eq!(stock.shapes, ShapeScale::STOCK);
        assert_eq!(expressive.shapes, ShapeScale::EXPRESSIVE);
        assert!(expressive.typography.is_some());
        assert!(expressive.motion.is_some());
        assert!(stock.typography.is_none());
        let dark = Theme::expressive(1200, 2608, 3.0, true);
        assert!(dark.dark);
        assert_eq!(dark.pal.bg, ColorScheme::dark().background);
    }
}

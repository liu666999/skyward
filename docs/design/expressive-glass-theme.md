# Skyward Expressive Glass Theme

## Author

**是雅而非哑**

## Goal

Add an optional theme that combines the design language of Material 3 Expressive
with a Liquid Glass-inspired material treatment. This is a Rust implementation
for Skyward's existing renderer, not a port of the AndroidX/Compose runtime.

The existing stock-inspired light and dark themes remain the default and remain
available as a safe fallback. The new theme must not change recovery operations,
touch hit-testing, storage, decryption, or boot behaviour.

## Findings from the current renderer

- `skyward-ui` is a standalone Rust crate and owns the drawing pipeline.
- The UI draws into a software RGBA canvas and presents it through the existing
  KMS surface. `skyward-preview` uses the same UI crate to render PNGs on a PC.
- `tiny-skia` is already a dependency; the current `Sprite::blurred` helper
  blurs a coverage mask, not an arbitrary backdrop image.
- `Theme` currently contains the stock-inspired `Palette`, `Hero` and
  device-specific `Metrics`; pages and widgets read those values directly.
- Settings currently persist a light/dark boolean. Theme selection therefore
  needs a backwards-compatible settings migration.

These facts rule out directly embedding a Compose UI or assuming that the
existing renderer can execute Android GPU shaders. The first implementation
should be CPU-renderer-native and region-limited.

## Module boundaries

- `expressive.rs`: semantic Material 3 colour roles, shape scale, type scale,
  and motion timing. It is renderer-independent.
- `glass.rs`: material presets and bounded quality levels. It does not perform
  backdrop sampling itself.
- Existing `theme.rs`: remains the adapter that converts a selected theme into
  the palette and metrics consumed by current pages/widgets.
- Existing `canvas.rs` and `sprite.rs`: candidates for a later clipped
  backdrop-snapshot and blur implementation. Avoid copying the whole screen
  for each card or each animation frame.

## Glass quality levels

- Off: ordinary opaque/translucent surfaces; lowest cost.
- Subtle: tint and a small blur radius.
- Standard: clipped backdrop blur and restrained edge highlight.
- Rich: stronger blur and a small displacement/refraction approximation.

All physical radii and displacement values are bounded by display density.
Rich mode must be opt-in; unsupported or memory-constrained devices fall back
to Subtle or Off. The first release must not promise physically accurate
refraction or GPU shader parity.

## Integration plan

1. **Token foundation** — introduce renderer-neutral colour, shape, typography,
   motion and glass material tokens. Keep the current theme untouched.
2. **Theme adapter** — persist the palette family alongside the existing
   light/dark choice. Old settings files that only contain `dark` continue to
   select the stock palette.
3. **Component adapter** — map semantic roles to existing cards, buttons,
   switches, sliders, dialogs and navigation. Do not change actions or hitboxes.
4. **Glass renderer** — snapshot only the clipped backdrop region, blur it with
   bounded passes, then tint and draw a restrained edge highlight. Cache only
   when the underlying content is unchanged; invalidate on scroll, touch,
   animation, or page transitions.
5. **Preview and verification** — extend `skyward-preview` to render all pages
   in light/dark Expressive and glass quality variants, including pressed,
   scrolling and animated states. Compare output and run workspace tests,
   clippy and the Android target checks.

## Performance and safety constraints

- No Compose, Kotlin runtime, AndroidX runtime, or new Android service.
- No full-screen blur per card and no unbounded per-frame allocations.
- Clip every backdrop operation to the surface bounds.
- Preserve readable text contrast; glass is decoration behind content only.
- Provide a one-step fallback to the existing theme if a frame budget is missed.
- Keep the existing theme and recovery UI usable if glass rendering fails.

## Upstream references and licensing

- Material 3 / AndroidX source: Apache License 2.0.
- AndroidLiquidGlass source: Apache License 2.0.
- Skyward itself is GPL-3.0-or-later.

This work should use upstream source as a behavioural and visual reference,
not copy Compose-dependent implementation wholesale. If source code is copied
or adapted, retain its copyright and Apache-2.0 notices in the relevant files
and document the adaptation. Keep the resulting Skyward changes compatible
with the repository's GPL-3.0-or-later distribution.

## Theme implementation progress

- Settings now offers four independent previews: stock light, stock dark, M3 Expressive light, and M3 Expressive dark.
- The selected palette is stored in the existing JSON settings file. Older files that do not contain the new `expressive` key continue to select the stock palette.
- The M3E adapter maps semantic roles across cards, text, accents, avatars, status/battery inks, pressed states, popups and outlines. Shared controls now also consume explicit `on_accent`, `on_danger`, inverse-surface, gesture-backdrop, navigation-chip and keyboard roles. Named shape roles now also drive compact row tags, keyboard-key corners and choice-popup corners; the stock scale retains their original radii, while card, popup and preview radii use the larger expressive roles. The popup regression test checks the stock 24 dp and M3E 32 dp corner sizes. Row heights and touch targets remain unchanged.
- M3E typography roles now feed page headlines, row titles, account titles, body, labels, badges, preview captions and the About-page hardware-card title/value/caption hierarchy. Hardware-card text can use three cached MiSans variable-font instances at the M3 headline/body roles' 400, 500 and 600 weights. They are initialized lazily only when an M3E hardware card is drawn; stock-only sessions retain the original font-loading path and stock cards continue using their original heading/body faces. The hardware-card mapping retains the original stock sizes. M3E body copy now uses its 20 sp line-height token for multiline spacing, and hardware-card headlines use the 32 sp headline-small line-height; stock line spacing remains unchanged. Its spatial spring drives page transitions, scroll settling and overscroll bounce; its effects spring drives search morphing, edge cues and pressed-row feedback. Search body reveal uses the M3E medium/short timing pair. Stock animation timings remain unchanged.
- Theme preview selection updates both palette family and brightness together; switching from M3E to stock cannot leave the old palette active. Regression tests cover all four presets, four non-overlapping preview hit regions, backwards-compatible settings, semantic palette mapping, stock control-color compatibility, shape-role mapping for tags, keyboard keys and choice popups, and text/control contrast on raised surfaces in both schemes.
- Liquid-glass rendering is intentionally not being expanded in this phase: finish and validate theme selection, palette consistency and previews first.

## Implementation notes

- `theme.rs` exposes `Theme::expressive(w, h, px_per_dp, dark)`, adapting M3E semantic roles into the existing Skyward palette consumed by current widgets. Device geometry, row heights, touch targets and recovery actions remain inherited from the stock theme.
- The Settings page now exposes four complete presets (stock light/dark and M3E light/dark), and the selected palette family is persisted with backward-compatible defaults.
- Glass material parameters remain separate from the active renderer; backdrop sampling, clipped blur, and page/component wiring remain outstanding.

## Current status

The M3E theme adapter, four-way selector, persistent choice, semantic palette mapping for shared controls, named shape/type metadata, compact component and popup shape-role mapping and contrast/regression tests are in place. The theme still needs a full preview pass across every page and a successful workspace CI run. M3E spatial/effects spring periods and short/medium reveal timings are now applied to existing navigation, search morph and press-feedback animations; device power/display timing and gesture thresholds remain unchanged. The current fork branch has no reported GitHub Actions run or commit status, and this editing environment has no Rust toolchain for a local build. Liquid Glass is not yet integrated into the active page components; no completed Liquid Glass implementation is claimed.

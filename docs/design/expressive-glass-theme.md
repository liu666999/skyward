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
2. **Theme adapter** — add a persisted theme identifier while preserving old
   settings files that only contain `dark`. Keep the existing light/dark mode
   semantics and add the new theme as an opt-in choice.
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

## Implementation notes

- `theme.rs` now exposes `Theme::expressive(w, h, px_per_dp, dark)`, adapting M3E semantic roles into the existing Skyward palette consumed by current widgets. Geometry, touch targets, battery inks and recovery actions remain inherited from the stock theme.
- This is a first palette adapter, not yet a selectable persisted theme. Glass material parameters are still separate from the renderer; backdrop sampling, clipped blur, and page/component wiring remain outstanding.

## Current status

The first token-only modules are in place. They are intentionally not wired into
the active UI yet: the theme adapter, persistent selector, component migration,
and backdrop renderer are subsequent steps. No claim of a completed visual
Liquid Glass implementation should be made until those steps are previewed and
tested.

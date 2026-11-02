# opencad-ui

The UI foundation of OpenCAD and
OpenSolve, on [egui](https://github.com/emilk/egui): design tokens (colours
for a light and a dark theme, spacing, radii, elevation, motion), the type
scale and the UI font per platform, an SVG-path icon set, and a widget set
(buttons, toggles, fields with length arithmetic, menus, chips, lists,
tables, trees, tabs, toolbars, dialogs, tool windows, docking pieces),
with English and German strings.

It depends on `egui` only (no eframe) and builds for
`wasm32-unknown-unknown`. The design and the rules for code that uses it
are in [ADR 0019](docs/adr/0019-ui-foundation.md).

## Use

```toml
[dependencies]
opencad-ui = { git = "https://github.com/cadscribe/opencad-ui", tag = "v0.1.0" }
```

```rust
// Once, when the egui context is created:
// fonts and both themes; the closure may add font families
opencad_ui::install(&ctx, |_fonts| {});
```

## Gallery

Every token and widget, the light and the dark theme side by side:

```sh
cargo run --example gallery
# screenshots of every page in both languages, then quit:
OPENCAD_UI_SHOT_DIR=/tmp/shots cargo run --example gallery
```

## Fonts

On macOS the crate uses SF Pro and SF Mono, on Windows Segoe UI (Variable)
and Consolas, all read from the system at runtime and not distributed
here. On Linux and the web it uses [Inter](https://github.com/rsms/inter)
4.1, bundled as `assets/fonts/InterVariable.ttf` (unmodified).

## Licence

The code is licensed under either of [MIT](LICENSE-MIT) or
[Apache-2.0](LICENSE-APACHE), at your option.

The bundled Inter font is © The Inter Project Authors and licensed under
the [SIL Open Font License 1.1](assets/fonts/Inter-OFL.txt); its licence
file ships with the crate, and binaries that bundle it should mention it
in their credits.

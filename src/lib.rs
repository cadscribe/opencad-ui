//! OpenCAD's UI foundation (ADR 0019): design tokens, typography, the
//! icon system and the widget set, on egui only (no app or model types),
//! for native and web builds.
//!
//! * [`tokens`]: colours (light and dark), spacing, radii, elevation,
//!   control sizes, motion;
//! * [`text`]: the type scale, truncation, tabular figures; [`fonts`]:
//!   the UI font (SF Pro, Segoe UI Variable, bundled Inter);
//! * [`icon`]: [`Icon`] (SVG path data drawn as shapes);
//! * [`widgets`]: buttons, toggles, fields, menus, lists, tables, trees,
//!   tabs, toolbars, dialogs, feedback, splitters, docking pieces;
//! * [`gallery`]: every token and widget, in both themes (Help ▸ UI
//!   Gallery, `cargo run -p opencad-ui --example gallery`);
//! * [`window`]: the safe area below the title bar, tool windows (OS
//!   windows natively), floating panels; modal dialogs are in
//!   [`widgets::dialog`];
//! * [`i18n`]: the UI language and the [`messages!`] tables (ADR 0011).
//!
//! Call [`install`] once at start-up. Widgets take their colours from the
//! theme the `Ui` is drawn in ([`theme::colors`]); app code uses tokens
//! and widgets instead of literal colours and sizes.

pub mod fonts;
pub mod gallery;
pub mod i18n;
pub mod icon;
pub mod length;
pub mod state;
pub mod svg;
pub mod text;
pub mod theme;
pub mod tokens;
pub mod widgets;
pub mod window;

pub use icon::Icon;
pub use text::Type;
pub use theme::colors;
pub use tokens::{Colors, Elevation, Tone, motion, radius, size, space};

/// Installs the UI font and the egui styles of both themes. `extend` may
/// add font families of the app (plan texts).
pub fn install(ctx: &egui::Context, extend: impl FnOnce(&mut egui::FontDefinitions)) {
    fonts::install(ctx, extend);
    theme::install(ctx);
}

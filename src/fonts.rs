//! The UI font: the platform's variable system font where it exists,
//! the bundled Inter elsewhere (ADR 0019, "Typography").
//!
//! * macOS: SF Pro (`SFNS.ttf`, variable; weights via the `wght` axis),
//!   SF Mono. Loaded from the system at runtime (Apple's licence does not
//!   allow bundling it).
//! * Windows 11: Segoe UI Variable (`SegUIVar.ttf`, `wght` axis); Windows
//!   10: Segoe UI and Segoe UI Semibold (static); Consolas.
//! * Linux and the web: Inter 4.1 (variable, SIL Open Font License 1.1),
//!   bundled as `assets/fonts/InterVariable.ttf` with its licence file.
//!
//! Every weight is its own font family ([`Weight::family`]); a variable
//! font is registered once per weight with a fixed `wght` coordinate, so
//! egui's own widgets (labels, text edits) can use any weight through a
//! plain [`egui::FontId`]. egui's built-in fonts stay behind every family
//! as fallbacks (symbols, emoji).

use std::sync::Arc;

use egui::{FontData, FontDefinitions, FontFamily};

/// A font weight of the type scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Weight {
    Regular,
    Medium,
    Semibold,
    Bold,
}

impl Weight {
    pub const ALL: [Weight; 4] = [
        Weight::Regular,
        Weight::Medium,
        Weight::Semibold,
        Weight::Bold,
    ];

    /// The font family of this weight. Regular is egui's proportional
    /// family, so unstyled egui text uses the UI font too.
    pub fn family(self) -> FontFamily {
        match self {
            Weight::Regular => FontFamily::Proportional,
            Weight::Medium => FontFamily::Name("ui-medium".into()),
            Weight::Semibold => FontFamily::Name("ui-semibold".into()),
            Weight::Bold => FontFamily::Name("ui-bold".into()),
        }
    }

    fn key(self) -> &'static str {
        match self {
            Weight::Regular => "regular",
            Weight::Medium => "medium",
            Weight::Semibold => "semibold",
            Weight::Bold => "bold",
        }
    }

    /// The `wght` coordinate for SF Pro / Segoe UI Variable / Inter.
    fn wght(self, font: Source) -> f32 {
        match (self, font) {
            // SF Pro's named instances.
            (Weight::Regular, Source::SfPro) => 400.0,
            (Weight::Medium, Source::SfPro) => 510.0,
            (Weight::Semibold, Source::SfPro) => 590.0,
            (Weight::Bold, Source::SfPro) => 700.0,
            (Weight::Regular, _) => 400.0,
            (Weight::Medium, _) => 500.0,
            (Weight::Semibold, _) => 600.0,
            (Weight::Bold, _) => 700.0,
        }
    }
}

/// Where the UI font came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    SfPro,
    SegoeVariable,
    /// Static Segoe UI files (Windows 10).
    SegoeStatic,
    Inter,
    /// egui's built-in fonts (no system font, bundled font disabled).
    EguiDefault,
}

impl Source {
    /// A name for the gallery and diagnostics.
    pub fn name(self) -> &'static str {
        match self {
            Source::SfPro => "SF Pro (system)",
            Source::SegoeVariable => "Segoe UI Variable (system)",
            Source::SegoeStatic => "Segoe UI (system)",
            Source::Inter => "Inter 4.1 (bundled, OFL)",
            Source::EguiDefault => "egui default",
        }
    }
}

/// The bundled Inter (variable). Only compiled into builds that use it.
#[cfg(not(any(target_os = "macos", target_os = "windows")))]
const INTER: &[u8] = include_bytes!("../assets/fonts/InterVariable.ttf");

/// Shared font bytes (a system file or the bundled font).
type Blob = Arc<dyn AsRef<[u8]> + Send + Sync>;

fn variable(bytes: Blob, wght: f32) -> FontData {
    let mut data = FontData::from_blob(bytes, 0);
    data.tweak.coords = egui::epaint::text::VariationCoords::new([(b"wght", wght)]);
    data
}

/// SF Pro's optical size for UI text: its "Text" design, as macOS sets
/// text under 20 pt. The font's default, 28, is the wider "Display" one.
#[cfg(target_os = "macos")]
const SF_TEXT: f32 = 17.0;

/// A weight of SF Pro, in its design for text.
#[cfg(target_os = "macos")]
fn sf_text(bytes: Blob, wght: f32) -> FontData {
    let mut data = FontData::from_blob(bytes, 0);
    data.tweak.coords =
        egui::epaint::text::VariationCoords::new([(b"wght", wght), (b"opsz", SF_TEXT)]);
    data
}

#[cfg(not(target_arch = "wasm32"))]
fn read(path: &str) -> Option<Blob> {
    std::fs::read(path).ok().map(|b| Arc::new(b) as Blob)
}

/// One loaded UI font: per weight the font data.
struct Loaded {
    source: Source,
    weights: Vec<(Weight, FontData)>,
    mono: Option<FontData>,
}

#[allow(unreachable_code)]
fn load() -> Loaded {
    #[cfg(target_os = "macos")]
    if let Some(bytes) = read("/System/Library/Fonts/SFNS.ttf") {
        return Loaded {
            source: Source::SfPro,
            weights: Weight::ALL
                .iter()
                .map(|w| (*w, sf_text(bytes.clone(), w.wght(Source::SfPro))))
                .collect(),
            mono: read("/System/Library/Fonts/SFNSMono.ttf").map(|b| variable(b, 400.0)),
        };
    }
    #[cfg(target_os = "windows")]
    {
        let mono = read("C:\\Windows\\Fonts\\consola.ttf").map(|b| FontData::from_blob(b, 0));
        if let Some(bytes) = read("C:\\Windows\\Fonts\\SegUIVar.ttf") {
            return Loaded {
                source: Source::SegoeVariable,
                weights: Weight::ALL
                    .iter()
                    .map(|w| (*w, variable(bytes.clone(), w.wght(Source::SegoeVariable))))
                    .collect(),
                mono,
            };
        }
        if let Some(regular) = read("C:\\Windows\\Fonts\\segoeui.ttf") {
            let semibold = read("C:\\Windows\\Fonts\\seguisb.ttf").unwrap_or(regular.clone());
            let bold = read("C:\\Windows\\Fonts\\segoeuib.ttf").unwrap_or(semibold.clone());
            return Loaded {
                source: Source::SegoeStatic,
                weights: vec![
                    (Weight::Regular, FontData::from_blob(regular.clone(), 0)),
                    (Weight::Medium, FontData::from_blob(regular, 0)),
                    (Weight::Semibold, FontData::from_blob(semibold, 0)),
                    (Weight::Bold, FontData::from_blob(bold, 0)),
                ],
                mono,
            };
        }
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        let bytes: Blob = Arc::new(INTER);
        return Loaded {
            source: Source::Inter,
            weights: Weight::ALL
                .iter()
                .map(|w| (*w, variable(bytes.clone(), w.wght(Source::Inter))))
                .collect(),
            mono: None,
        };
    }
    Loaded {
        source: Source::EguiDefault,
        weights: Vec::new(),
        mono: None,
    }
}

/// Font definitions: the UI font per weight in front of egui's fonts,
/// and which font was found.
pub fn definitions() -> (FontDefinitions, Source) {
    let mut defs = FontDefinitions::default();
    let fallbacks = defs
        .families
        .get(&FontFamily::Proportional)
        .cloned()
        .unwrap_or_default();
    let loaded = load();
    for (weight, data) in loaded.weights {
        let name = format!("ui-{}", weight.key());
        defs.font_data.insert(name.clone(), Arc::new(data));
        let list = defs.families.entry(weight.family()).or_default();
        list.insert(0, name);
    }
    if let Some(mono) = loaded.mono {
        defs.font_data.insert("ui-mono".into(), Arc::new(mono));
        defs.families
            .entry(FontFamily::Monospace)
            .or_default()
            .insert(0, "ui-mono".into());
    }
    // Weights without a font of their own fall back to the regular UI
    // font, then to egui's fonts.
    let regular = defs
        .families
        .get(&FontFamily::Proportional)
        .cloned()
        .unwrap_or_default();
    for w in Weight::ALL {
        let list = defs.families.entry(w.family()).or_default();
        for name in regular.iter().chain(&fallbacks) {
            if !list.contains(name) {
                list.push(name.clone());
            }
        }
    }
    (defs, loaded.source)
}

fn source_id() -> egui::Id {
    egui::Id::unique("opencad_ui_font_source")
}

/// The font found by [`install`].
pub fn source(ctx: &egui::Context) -> Source {
    ctx.data(|d| d.get_temp(source_id()))
        .unwrap_or(Source::EguiDefault)
}

/// Installs [`definitions`]; `extend` may add families (plan texts).
pub fn install(ctx: &egui::Context, extend: impl FnOnce(&mut FontDefinitions)) {
    let (mut defs, source) = definitions();
    extend(&mut defs);
    ctx.set_fonts(defs);
    ctx.data_mut(|d| d.insert_temp(source_id(), source));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_weight_has_fonts_and_fallbacks() {
        let (defs, _) = definitions();
        for w in Weight::ALL {
            let list = &defs.families[&w.family()];
            assert!(!list.is_empty(), "{w:?}");
            for name in list {
                assert!(defs.font_data.contains_key(name), "{name}");
            }
        }
        // The regular family's fonts back every other weight.
        let regular = &defs.families[&FontFamily::Proportional];
        for w in Weight::ALL {
            for name in regular {
                assert!(defs.families[&w.family()].contains(name));
            }
        }
    }
}

//! Design tokens: colours (light and dark), spacing, radii, elevation,
//! control sizes and motion. Typography is in [`crate::text`].
//!
//! The values follow the approved mockups (Main, Dark, Window, TypePicker,
//! Ribbon, Workspace, Wizard, Chat). App code never writes a colour or a
//! size literal; it takes them from here (ADR 0019).

use egui::{Color32, Shadow};

const fn rgb(v: u32) -> Color32 {
    Color32::from_rgb((v >> 16) as u8, (v >> 8) as u8, v as u8)
}

/// The colours of a status tone: a tinted background, text on it and a
/// solid colour (dots, icons, bars).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToneColors {
    /// Tinted background (chips, notices).
    pub tint: Color32,
    /// Text and icons on [`ToneColors::tint`].
    pub text: Color32,
    /// Solid colour: dots, icons on the surface, progress bars.
    pub solid: Color32,
}

/// A status tone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Tone {
    #[default]
    Neutral,
    Info,
    Success,
    Warning,
    Danger,
}

impl Tone {
    pub const ALL: [Tone; 5] = [
        Tone::Neutral,
        Tone::Info,
        Tone::Success,
        Tone::Warning,
        Tone::Danger,
    ];
}

/// Semantic colours of one theme.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Colors {
    pub dark: bool,
    /// The window background behind panes and panels.
    pub bg: Color32,
    /// Title bar, ribbon, side rail and status bar.
    pub chrome: Color32,
    /// Panels and sidebars behind cards (the palette background).
    pub surface: Color32,
    /// Cards, panes, popovers, menus and dialogs.
    pub surface_raised: Color32,
    /// Inset blocks inside a card, tab strips, sidebars of dialogs.
    pub surface_sunken: Color32,
    /// Background of fields and field-like buttons.
    pub field: Color32,
    /// Background of a focused field.
    pub field_focus: Color32,
    /// Tracks: segmented controls, switch off, progress, scroll bars.
    pub track: Color32,
    /// Tracks of progress indicators (stronger, they sit on cards).
    pub track_strong: Color32,
    /// The raised thumb of a segmented control.
    pub thumb: Color32,
    pub border: Color32,
    /// Hovered field borders, swatch outlines.
    pub border_strong: Color32,
    /// Dashed outline of a field with differing values ("–").
    pub border_mixed: Color32,
    /// Thin lines between rows and sections.
    pub divider: Color32,
    pub text: Color32,
    pub text_secondary: Color32,
    /// Hints, shortcuts, placeholders.
    pub text_tertiary: Color32,
    pub text_disabled: Color32,
    /// Default colour of icons on surfaces.
    pub icon: Color32,
    /// Accent for text, icons and outlines (links, selected icons).
    pub accent: Color32,
    /// Filled accent controls: primary buttons, switch on, checkboxes.
    pub accent_fill: Color32,
    pub accent_fill_hover: Color32,
    pub accent_fill_pressed: Color32,
    /// Text and icons on [`Colors::accent_fill`].
    pub on_accent: Color32,
    /// Filled destructive buttons.
    pub danger_fill: Color32,
    pub danger_fill_hover: Color32,
    /// Text on [`Colors::danger_fill`].
    pub on_danger: Color32,
    /// Tinted accent background: icon squares, anchors, toggled buttons.
    pub accent_tint: Color32,
    /// Text on [`Colors::accent_tint`].
    pub accent_text: Color32,
    /// The highlighted (hovered or keyboard) row of a menu or list; white
    /// text on it.
    pub highlight: Color32,
    /// Text on [`Colors::highlight`].
    pub on_highlight: Color32,
    /// Selected rows of lists, tables and trees (not hovered).
    pub selection: Color32,
    /// Selected text in fields.
    pub text_selection: Color32,
    /// The soft focus ring around a focused control.
    pub focus: Color32,
    /// Translucent overlay of hovered flat controls.
    pub hover: Color32,
    /// Translucent overlay of pressed flat controls.
    pub pressed: Color32,
    /// Translucent fill of floating toolbars and viewport chips.
    pub glass: Color32,
    /// Dimmed backdrop behind modal dialogs.
    pub scrim: Color32,
    /// Base colour of shadows (see [`Elevation`]).
    pub shadow: Color32,
    /// Hairline ring around raised surfaces.
    pub ring: Color32,
    /// Code blocks and terminals (dark in both themes).
    pub code_bg: Color32,
    pub code_text: Color32,
    /// The AI assistant's colour (avatar, presence).
    pub assistant: Color32,
    pub neutral: ToneColors,
    pub info: ToneColors,
    pub success: ToneColors,
    pub warning: ToneColors,
    pub danger: ToneColors,
}

impl Colors {
    pub const LIGHT: Colors = Colors {
        dark: false,
        bg: rgb(0xE4E4E8),
        chrome: rgb(0xECECEF),
        surface: rgb(0xF2F2F4),
        surface_raised: Color32::WHITE,
        surface_sunken: rgb(0xF7F7F9),
        field: rgb(0xFAFAFB),
        field_focus: Color32::WHITE,
        track: rgb(0xF0F0F3),
        track_strong: rgb(0xE6E6EA),
        thumb: Color32::WHITE,
        border: rgb(0xD9D9DE),
        border_strong: rgb(0xCFCFD4),
        border_mixed: rgb(0xB7B7BE),
        divider: rgb(0xEEEEF1),
        text: rgb(0x1D1D1F),
        text_secondary: rgb(0x55555B),
        text_tertiary: rgb(0x6E6E73),
        text_disabled: rgb(0xA8A8AE),
        icon: rgb(0x3A3A3F),
        accent: rgb(0x0A5BC4),
        accent_fill: rgb(0x0A5BC4),
        accent_fill_hover: rgb(0x084A9E),
        accent_fill_pressed: rgb(0x073F87),
        on_accent: Color32::WHITE,
        danger_fill: rgb(0xC8372D),
        danger_fill_hover: rgb(0xAE2B22),
        on_danger: Color32::WHITE,
        accent_tint: rgb(0xEEF3FB),
        accent_text: rgb(0x0A5BC4),
        highlight: rgb(0x0A64D8),
        on_highlight: Color32::WHITE,
        selection: rgb(0xDCE7F8),
        text_selection: rgb(0xD6E5FA),
        focus: rgb(0xD6E5FA),
        hover: Color32::from_black_alpha(12),
        pressed: Color32::from_black_alpha(26),
        glass: Color32::from_rgba_premultiplied(240, 240, 240, 240),
        scrim: Color32::from_black_alpha(46),
        shadow: Color32::BLACK,
        ring: Color32::from_black_alpha(20),
        code_bg: rgb(0x1C1C1F),
        code_text: rgb(0xE8E8ED),
        assistant: rgb(0xF28C28),
        neutral: ToneColors {
            tint: rgb(0xEEEEF1),
            text: rgb(0x55555B),
            solid: rgb(0xA1A1A8),
        },
        info: ToneColors {
            tint: rgb(0xEEF3FB),
            text: rgb(0x0A4FA8),
            solid: rgb(0x0A5BC4),
        },
        success: ToneColors {
            tint: rgb(0xE7F5EC),
            text: rgb(0x1B6E3A),
            solid: rgb(0x2E9E5B),
        },
        warning: ToneColors {
            tint: rgb(0xFDF0DF),
            text: rgb(0x8A4B0A),
            solid: rgb(0xE0851E),
        },
        danger: ToneColors {
            tint: rgb(0xFCE8E6),
            text: rgb(0xA3271B),
            solid: rgb(0xD2603A),
        },
    };

    pub const DARK: Colors = Colors {
        dark: true,
        bg: rgb(0x161618),
        chrome: rgb(0x1E1E20),
        surface: rgb(0x1E1E20),
        surface_raised: rgb(0x2A2A2D),
        surface_sunken: rgb(0x232326),
        field: rgb(0x232326),
        field_focus: rgb(0x1E1E20),
        track: rgb(0x19191B),
        track_strong: rgb(0x3A3A3F),
        thumb: rgb(0x3A3A3E),
        border: rgb(0x3D3D42),
        border_strong: rgb(0x4A4A50),
        border_mixed: rgb(0x5A5A60),
        divider: rgb(0x38383C),
        text: rgb(0xF2F2F5),
        text_secondary: rgb(0xA1A1A8),
        text_tertiary: rgb(0x8E8E94),
        text_disabled: rgb(0x5E5E64),
        icon: rgb(0xC7C7CC),
        accent: rgb(0x6AA7F5),
        accent_fill: rgb(0x6AA7F5),
        accent_fill_hover: rgb(0x8FBDF8),
        accent_fill_pressed: rgb(0x4F93EE),
        on_accent: rgb(0x0E1622),
        danger_fill: rgb(0xF08A7C),
        danger_fill_hover: rgb(0xF4A093),
        on_danger: rgb(0x2A0E0A),
        accent_tint: rgb(0x1F3352),
        accent_text: rgb(0x8FBDF8),
        highlight: rgb(0x0A5FCC),
        on_highlight: Color32::WHITE,
        selection: rgb(0x22406B),
        text_selection: rgb(0x22406B),
        focus: rgb(0x22406B),
        hover: Color32::from_rgba_premultiplied(18, 18, 18, 18),
        pressed: Color32::from_rgba_premultiplied(34, 34, 34, 34),
        glass: Color32::from_rgba_premultiplied(40, 40, 42, 240),
        scrim: Color32::from_black_alpha(110),
        shadow: Color32::BLACK,
        ring: Color32::from_rgba_premultiplied(20, 20, 20, 20),
        code_bg: rgb(0x151517),
        code_text: rgb(0xE8E8ED),
        assistant: rgb(0xF28C28),
        neutral: ToneColors {
            tint: rgb(0x333337),
            text: rgb(0xA1A1A8),
            solid: rgb(0x6E6E75),
        },
        info: ToneColors {
            tint: rgb(0x1F3352),
            text: rgb(0x8FBDF8),
            solid: rgb(0x6AA7F5),
        },
        success: ToneColors {
            tint: rgb(0x1D3B28),
            text: rgb(0x7FD6A0),
            solid: rgb(0x4CC27A),
        },
        warning: ToneColors {
            tint: rgb(0x402C14),
            text: rgb(0xF2B66B),
            solid: rgb(0xE8952E),
        },
        danger: ToneColors {
            tint: rgb(0x45201C),
            text: rgb(0xF4A093),
            solid: rgb(0xE0604A),
        },
    };

    /// The colours of the light or dark theme.
    pub const fn of(dark: bool) -> &'static Colors {
        if dark { &Self::DARK } else { &Self::LIGHT }
    }

    /// The colours of `tone`.
    pub const fn tone(&self, tone: Tone) -> ToneColors {
        match tone {
            Tone::Neutral => self.neutral,
            Tone::Info => self.info,
            Tone::Success => self.success,
            Tone::Warning => self.warning,
            Tone::Danger => self.danger,
        }
    }
}

/// Spacing scale in points. Layout gaps and paddings use these steps
/// only.
pub mod space {
    pub const XXS: f32 = 2.0;
    pub const XS: f32 = 4.0;
    pub const SM: f32 = 6.0;
    pub const MD: f32 = 8.0;
    pub const LG: f32 = 10.0;
    pub const XL: f32 = 12.0;
    pub const XXL: f32 = 14.0;
    pub const X3L: f32 = 16.0;
    pub const X4L: f32 = 20.0;
    pub const X5L: f32 = 24.0;
    pub const X6L: f32 = 32.0;

    /// The whole scale, smallest first (gallery, tests).
    pub const SCALE: [(&str, f32); 11] = [
        ("xxs", XXS),
        ("xs", XS),
        ("sm", SM),
        ("md", MD),
        ("lg", LG),
        ("xl", XL),
        ("xxl", XXL),
        ("3xl", X3L),
        ("4xl", X4L),
        ("5xl", X5L),
        ("6xl", X6L),
    ];
}

/// Corner radii in points.
pub mod radius {
    /// Swatches, small marks.
    pub const XS: u8 = 3;
    /// Menu rows, small icon buttons, list rows.
    pub const ROW: u8 = 5;
    /// Fields, chips with corners, icon buttons, segments.
    pub const FIELD: u8 = 6;
    /// Buttons, segmented tracks.
    pub const CONTROL: u8 = 7;
    /// Tiles, inset blocks, rich buttons, notices.
    pub const TILE: u8 = 8;
    /// Menus and popovers.
    pub const MENU: u8 = 9;
    /// Cards and panes.
    pub const CARD: u8 = 10;
    /// Floating panes, dialogs, large popovers, floating toolbars.
    pub const OVERLAY: u8 = 12;
    /// Fully rounded ends (chips, switches, pills).
    pub const PILL: u8 = 255;

    pub const SCALE: [(&str, u8); 8] = [
        ("xs", XS),
        ("row", ROW),
        ("field", FIELD),
        ("control", CONTROL),
        ("tile", TILE),
        ("menu", MENU),
        ("card", CARD),
        ("overlay", OVERLAY),
    ];
}

/// Control sizes in points.
pub mod size {
    /// Height of fields, small buttons and segmented controls in rows.
    pub const FIELD_H: f32 = 24.0;
    /// Heights of buttons: small, medium (default), large.
    pub const BUTTON_SM: f32 = 24.0;
    pub const BUTTON_MD: f32 = 28.0;
    pub const BUTTON_LG: f32 = 32.0;
    /// Height of a menu row.
    pub const MENU_ROW: f32 = 24.0;
    /// Width of a menu (minimum).
    pub const MENU_W: f32 = 220.0;
    /// Width of a short menu (a few one-word entries: appearance, task
    /// area).
    pub const MENU_W_SM: f32 = 160.0;
    /// Height of a list or tree row (one line).
    pub const ROW_H: f32 = 26.0;
    /// Height of a list row with a secondary line.
    pub const ROW_TWO_LINE_H: f32 = 44.0;
    /// Icon sizes.
    pub const ICON_SM: f32 = 13.0;
    pub const ICON_MD: f32 = 16.0;
    pub const ICON_LG: f32 = 20.0;
    pub const ICON_XL: f32 = 26.0;
    /// Width of the label column of a form row.
    pub const LABEL_W: f32 = 92.0;
    /// Default width of a number field.
    pub const NUMBER_W: f32 = 90.0;
    /// Width of the focus ring.
    pub const FOCUS_RING: f32 = 3.0;
    /// Height of a tab strip of a pane.
    pub const TAB_STRIP_H: f32 = 36.0;
}

/// Elevation levels: shadow and hairline ring of raised surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Elevation {
    /// Flat (no shadow).
    Flat,
    /// Cards on a panel: a barely visible shadow.
    Card,
    /// Panes on the window background: a ring and a soft shadow.
    Pane,
    /// The raised thumb of a segmented control, a selected wizard step.
    Raised,
    /// Menus, popovers, tooltips, floating toolbars.
    Popover,
    /// Dialogs and floating windows.
    Dialog,
}

impl Elevation {
    pub const ALL: [Elevation; 6] = [
        Elevation::Flat,
        Elevation::Card,
        Elevation::Pane,
        Elevation::Raised,
        Elevation::Popover,
        Elevation::Dialog,
    ];

    /// The shadow of this level in a theme.
    pub const fn shadow(self, dark: bool) -> Shadow {
        // Dark surfaces need stronger shadows to read at all.
        let (offset, blur, light, dark_a) = match self {
            Elevation::Flat => return Shadow::NONE,
            Elevation::Card => ([0, 1], 2, 10, 60),
            Elevation::Pane => ([0, 1], 3, 16, 70),
            Elevation::Raised => ([0, 1], 2, 30, 100),
            Elevation::Popover => ([0, 8], 24, 42, 120),
            Elevation::Dialog => ([0, 18], 50, 52, 140),
        };
        Shadow {
            offset,
            blur,
            spread: 0,
            color: Color32::from_black_alpha(if dark { dark_a } else { light }),
        }
    }

    /// Whether this level draws a hairline ring ([`Colors::ring`]).
    pub fn ring(self) -> bool {
        matches!(
            self,
            Elevation::Pane | Elevation::Popover | Elevation::Dialog
        )
    }
}

/// Motion: durations in seconds and easing curves.
pub mod motion {
    /// Hover and press colour changes.
    pub const FAST: f32 = 0.08;
    /// Switches, checkmarks, chevrons, segment thumbs.
    pub const NORMAL: f32 = 0.14;
    /// Collapsing sections, popover fade, toasts.
    pub const SLOW: f32 = 0.22;
    /// Indeterminate progress, spinner: seconds per cycle.
    pub const SPIN: f32 = 1.0;

    /// Easing of state changes (decelerating).
    pub fn ease_out(t: f32) -> f32 {
        egui::emath::easing::cubic_out(t)
    }

    /// Easing of movement that starts and ends at rest.
    pub fn ease_in_out(t: f32) -> f32 {
        egui::emath::easing::cubic_in_out(t)
    }
}

/// Mixes `overlay` (premultiplied, translucent) onto the opaque `base`.
pub fn over(base: Color32, overlay: Color32) -> Color32 {
    let a = f32::from(overlay.a()) / 255.0;
    let mix = |b: u8, o: u8| (f32::from(o) + f32::from(b) * (1.0 - a)).round() as u8;
    Color32::from_rgb(
        mix(base.r(), overlay.r()),
        mix(base.g(), overlay.g()),
        mix(base.b(), overlay.b()),
    )
}

/// Linear blend of two opaque colours (`t` = 0 → `a`, 1 → `b`).
pub fn mix(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let l = |x: u8, y: u8| (f32::from(x) + (f32::from(y) - f32::from(x)) * t).round() as u8;
    Color32::from_rgba_premultiplied(
        l(a.r(), b.r()),
        l(a.g(), b.g()),
        l(a.b(), b.b()),
        l(a.a(), b.a()),
    )
}

/// Relative luminance (WCAG) of an opaque colour.
pub fn luminance(c: Color32) -> f32 {
    let lin = |v: u8| {
        let v = f32::from(v) / 255.0;
        if v <= 0.040_45 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * lin(c.r()) + 0.7152 * lin(c.g()) + 0.0722 * lin(c.b())
}

/// WCAG contrast ratio of two opaque colours.
pub fn contrast(a: Color32, b: Color32) -> f32 {
    let (la, lb) = (luminance(a), luminance(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_contrasts_with_every_surface() {
        for c in [Colors::LIGHT, Colors::DARK] {
            for bg in [c.surface, c.surface_raised, c.surface_sunken, c.field] {
                assert!(contrast(c.text, bg) >= 7.0, "text on {bg:?} ({})", c.dark);
                assert!(
                    contrast(c.text_secondary, bg) >= 4.5,
                    "secondary {}",
                    c.dark
                );
                assert!(contrast(c.text_tertiary, bg) >= 3.0, "tertiary {}", c.dark);
            }
            assert!(
                contrast(c.on_accent, c.accent_fill) >= 4.5,
                "on accent {}",
                c.dark
            );
            assert!(
                contrast(c.on_danger, c.danger_fill) >= 4.5,
                "on danger {}",
                c.dark
            );
            assert!(
                contrast(c.on_highlight, c.highlight) >= 4.5,
                "highlight {}",
                c.dark
            );
            assert!(
                contrast(c.accent_text, c.accent_tint) >= 4.5,
                "tint {}",
                c.dark
            );
            for tone in Tone::ALL {
                let t = c.tone(tone);
                assert!(contrast(t.text, t.tint) >= 4.5, "{tone:?} {}", c.dark);
            }
        }
    }

    #[test]
    fn scales_grow() {
        for w in space::SCALE.windows(2) {
            assert!(w[0].1 < w[1].1);
        }
        for w in radius::SCALE.windows(2) {
            assert!(w[0].1 < w[1].1);
        }
    }

    #[test]
    fn overlay_and_mix() {
        assert_eq!(over(Color32::WHITE, Color32::TRANSPARENT), Color32::WHITE);
        let d = over(Color32::WHITE, Color32::from_black_alpha(30));
        assert!(d.r() < 255 && d.r() > 200);
        assert_eq!(mix(Color32::BLACK, Color32::WHITE, 0.0), Color32::BLACK);
        assert_eq!(mix(Color32::BLACK, Color32::WHITE, 1.0), Color32::WHITE);
        assert!(Elevation::Dialog.shadow(false).blur > Elevation::Card.shadow(false).blur);
    }
}

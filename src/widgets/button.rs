//! Buttons: text buttons in four variants and three sizes (with an
//! optional icon), icon buttons and link buttons.

use egui::{
    Align, Color32, Rect, Response, Role, Sense, Stroke, StrokeKind, Ui, Vec2, Widget, pos2, vec2,
};

use crate::icon::{self, Icon};
use crate::state::{self, State};
use crate::text::{self, Type};
use crate::theme::colors;
use crate::tokens::{Colors, Elevation, mix, motion, over, radius, size, space};

/// The emphasis of a button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Variant {
    /// The one main action of a dialog or card (filled accent).
    Primary,
    /// Other actions (white with a border).
    #[default]
    Secondary,
    /// Low emphasis (no frame until hovered): "Cancel", toolbar text.
    Tertiary,
    /// Deleting or discarding (filled red).
    Destructive,
    /// A low-emphasis destructive action (red text, no frame until
    /// hovered): "Remove all cuts" inside a card.
    DestructiveTertiary,
}

impl Variant {
    pub const ALL: [Variant; 5] = [
        Variant::Primary,
        Variant::Secondary,
        Variant::Tertiary,
        Variant::Destructive,
        Variant::DestructiveTertiary,
    ];
}

/// The height of a button.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ButtonSize {
    /// 24 pt, inside cards and table rows.
    Small,
    /// 28 pt, the default.
    #[default]
    Medium,
    /// 32 pt, dialog footers and wizards.
    Large,
}

impl ButtonSize {
    pub const ALL: [ButtonSize; 3] = [ButtonSize::Small, ButtonSize::Medium, ButtonSize::Large];

    fn height(self) -> f32 {
        match self {
            ButtonSize::Small => size::BUTTON_SM,
            ButtonSize::Medium => size::BUTTON_MD,
            ButtonSize::Large => size::BUTTON_LG,
        }
    }

    fn pad(self) -> f32 {
        match self {
            ButtonSize::Small => space::MD,
            ButtonSize::Medium => space::XL,
            ButtonSize::Large => space::X3L,
        }
    }

    fn text(self, strong: bool) -> Type {
        match (self, strong) {
            (ButtonSize::Large, true) => Type::LabelStrong,
            (ButtonSize::Large, false) => Type::Label,
            (_, true) => Type::BodyStrong,
            (_, false) => Type::Body,
        }
    }

    fn radius(self) -> u8 {
        match self {
            ButtonSize::Small => radius::FIELD,
            ButtonSize::Medium => radius::CONTROL,
            ButtonSize::Large => radius::TILE,
        }
    }
}

/// Fill, border and text colour of a button in a state.
fn look(variant: Variant, s: State, c: &Colors, hover_t: f32) -> (Color32, Stroke, Color32) {
    match variant {
        Variant::Primary => {
            if s.disabled {
                return (
                    mix(c.surface_raised, c.accent_fill, 0.32),
                    Stroke::NONE,
                    mix(
                        c.on_accent,
                        c.surface_raised,
                        if c.dark { 0.3 } else { 0.0 },
                    ),
                );
            }
            let fill = if s.pressed {
                c.accent_fill_pressed
            } else {
                mix(c.accent_fill, c.accent_fill_hover, hover_t)
            };
            (fill, Stroke::NONE, c.on_accent)
        }
        Variant::Destructive => {
            if s.disabled {
                return (
                    mix(c.surface_raised, c.danger_fill, 0.32),
                    Stroke::NONE,
                    c.on_danger,
                );
            }
            let fill = mix(
                c.danger_fill,
                c.danger_fill_hover,
                hover_t.max(f32::from(u8::from(s.pressed))),
            );
            (fill, Stroke::NONE, c.on_danger)
        }
        Variant::Secondary => {
            let base = c.surface_raised;
            let fill = if s.pressed {
                over(base, c.pressed)
            } else {
                over(base, c.hover.gamma_multiply(hover_t))
            };
            let border = if s.hovered { c.border_strong } else { c.border };
            (
                fill,
                Stroke::new(1.0, border),
                state::text_color(s, c.text, c),
            )
        }
        Variant::Tertiary => {
            let fill = if s.pressed {
                c.pressed
            } else {
                c.hover.gamma_multiply(hover_t)
            };
            (
                fill,
                Stroke::NONE,
                state::text_color(s, c.text_secondary, c),
            )
        }
        Variant::DestructiveTertiary => {
            let fill = if s.pressed {
                c.pressed
            } else {
                c.hover.gamma_multiply(hover_t)
            };
            let fg = if s.disabled {
                c.text_disabled
            } else {
                c.tone(crate::tokens::Tone::Danger).text
            };
            (fill, Stroke::NONE, fg)
        }
    }
}

/// A text button, optionally with a leading icon.
#[derive(Debug, Clone)]
#[must_use = "add it with `ui.add(button)`"]
pub struct Button<'a> {
    label: &'a str,
    icon: Option<Icon>,
    trailing: Option<Icon>,
    variant: Variant,
    size: ButtonSize,
    min_width: f32,
    fill_width: bool,
    access_label: Option<&'a str>,
}

impl<'a> Button<'a> {
    pub fn new(label: &'a str) -> Self {
        Button {
            label,
            icon: None,
            trailing: None,
            variant: Variant::Secondary,
            size: ButtonSize::Medium,
            min_width: 0.0,
            fill_width: false,
            access_label: None,
        }
    }

    pub fn primary(label: &'a str) -> Self {
        Self::new(label).variant(Variant::Primary)
    }

    pub fn secondary(label: &'a str) -> Self {
        Self::new(label).variant(Variant::Secondary)
    }

    pub fn tertiary(label: &'a str) -> Self {
        Self::new(label).variant(Variant::Tertiary)
    }

    pub fn destructive(label: &'a str) -> Self {
        Self::new(label).variant(Variant::Destructive)
    }

    /// A low-emphasis destructive action (red text).
    pub fn destructive_tertiary(label: &'a str) -> Self {
        Self::new(label).variant(Variant::DestructiveTertiary)
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// A leading icon.
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// A trailing icon (a chevron for menu buttons).
    pub fn trailing(mut self, icon: Icon) -> Self {
        self.trailing = Some(icon);
        self
    }

    pub fn min_width(mut self, width: f32) -> Self {
        self.min_width = width;
        self
    }

    /// The accessible name when the visible label alone says too little
    /// ("Task area: Architecture" for a menu button showing "Architecture").
    pub fn access_label(mut self, label: &'a str) -> Self {
        self.access_label = Some(label);
        self
    }

    /// Takes the available width (buttons side by side in a card).
    pub fn fill_width(mut self) -> Self {
        self.fill_width = true;
        self
    }
}

impl Widget for Button<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let c = colors(ui);
        let strong = matches!(self.variant, Variant::Primary | Variant::Destructive);
        let t = self.size.text(strong);
        let h = self.size.height();
        let pad = self.size.pad();
        let icon_w = if self.icon.is_some() {
            size::ICON_MD - 2.0 + space::SM
        } else {
            0.0
        };
        let trail_w = if self.trailing.is_some() {
            12.0 + space::XS
        } else {
            0.0
        };
        let text_w = text::galley(ui.painter(), self.label, t, c.text).size().x;
        let natural = pad * 2.0 + icon_w + trail_w + text_w;
        let w = if self.fill_width {
            ui.available_width().max(self.min_width)
        } else {
            natural.max(self.min_width).min(ui.available_width().max(h))
        };
        let (rect, resp) = ui.allocate_exact_size(vec2(w, h), Sense::click());
        state::a11y(&resp, Role::Button, self.access_label.unwrap_or(self.label));
        if !ui.is_rect_visible(rect) {
            return resp;
        }
        let s = state::state(ui, &resp);
        let hover_t = state::animate(ui, resp.id.with("hover"), s.hovered, motion::FAST);
        let (fill, stroke, fg) = look(self.variant, s, c, hover_t);
        let r = self.size.radius();
        let p = ui.painter();
        if self.variant == Variant::Secondary && !s.disabled {
            p.add(Elevation::Card.shadow(c.dark).as_shape(rect, r));
        }
        p.rect(rect, r, fill, stroke, StrokeKind::Inside);
        // Content, centred when it fits.
        let avail = rect.width() - pad * 2.0 - icon_w - trail_w;
        let g = text::truncated(p, self.label, t, fg, avail);
        let content = icon_w + g.size().x + trail_w;
        let mut x = rect.center().x - content / 2.0;
        if let Some(i) = self.icon {
            let s = size::ICON_MD - 2.0;
            icon::paint(
                p,
                Rect::from_center_size(pos2(x + s / 2.0, rect.center().y), Vec2::splat(s)),
                i,
                fg,
            );
            x += icon_w;
        }
        let gw = g.size().x;
        let elided = g.elided;
        text::paint_in(
            p,
            Rect::from_min_max(pos2(x, rect.top()), pos2(x + gw, rect.bottom())),
            g,
            Align::Min,
            fg,
        );
        x += gw + space::XS;
        if let Some(i) = self.trailing {
            icon::paint(
                p,
                Rect::from_center_size(pos2(x + 6.0, rect.center().y), Vec2::splat(12.0)),
                i,
                fg,
            );
        }
        if s.focused {
            state::focus_ring(ui, rect, r, c);
        }
        if elided {
            resp.on_hover_text(self.label)
        } else {
            resp
        }
    }
}

/// A square button with an icon only; the tooltip is its accessible name.
#[derive(Debug, Clone)]
#[must_use = "add it with `ui.add(button)`"]
pub struct IconButton<'a> {
    icon: Icon,
    tip: &'a str,
    size: Vec2,
    icon_size: f32,
    selected: bool,
    framed: bool,
    color: Option<Color32>,
}

impl<'a> IconButton<'a> {
    pub fn new(icon: Icon, tip: &'a str) -> Self {
        IconButton {
            icon,
            tip,
            size: Vec2::splat(size::BUTTON_MD),
            icon_size: size::ICON_MD,
            selected: false,
            framed: false,
            color: None,
        }
    }

    /// Button size in points (default 28 × 28).
    pub fn size(mut self, size: impl Into<Vec2>) -> Self {
        self.size = size.into();
        self
    }

    pub fn icon_size(mut self, size: f32) -> Self {
        self.icon_size = size;
        self
    }

    /// Toggled on: accent tint and accent icon.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// With a field-like frame (next to fields).
    pub fn framed(mut self) -> Self {
        self.framed = true;
        self
    }

    /// An icon colour other than the theme's icon colour.
    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }
}

impl Widget for IconButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let c = colors(ui);
        let (rect, resp) = ui.allocate_exact_size(self.size, Sense::click());
        state::a11y_selected(&resp, Role::Button, self.selected, self.tip);
        let s = state::state(ui, &resp);
        let r = if self.size.y > 30.0 {
            radius::TILE
        } else {
            radius::FIELD
        };
        let p = ui.painter();
        if self.framed {
            let border = if s.hovered { c.border_strong } else { c.border };
            p.rect(
                rect,
                r,
                c.field,
                Stroke::new(1.0, border),
                StrokeKind::Inside,
            );
        }
        if self.selected {
            p.rect_filled(rect, r, c.accent_tint);
        }
        let hover = state::hover_fill(ui, resp.id, s, c);
        if hover.a() > 0 {
            p.rect_filled(rect, r, hover);
        }
        let fg = if s.disabled {
            c.text_disabled
        } else if self.selected {
            c.accent
        } else {
            self.color.unwrap_or(c.icon)
        };
        icon::paint(
            p,
            Rect::from_center_size(rect.center(), Vec2::splat(self.icon_size)),
            self.icon,
            fg,
        );
        if s.focused {
            state::focus_ring(ui, rect, r, c);
        }
        if self.tip.is_empty() {
            resp
        } else {
            resp.on_hover_text(self.tip)
        }
    }
}

/// A frameless accent text button ("+ Layer", "Help").
pub fn link(ui: &mut Ui, label: &str) -> Response {
    let c = colors(ui);
    let g = text::galley(ui.painter(), label, Type::Body, c.accent);
    let (rect, resp) = ui.allocate_exact_size(vec2(g.size().x, size::FIELD_H), Sense::click());
    state::a11y(&resp, Role::Link, label);
    let s = state::state(ui, &resp);
    let fg = if s.disabled {
        c.text_disabled
    } else if s.hovered {
        c.accent_fill_hover.gamma_multiply(1.0)
    } else {
        c.accent
    };
    let g = text::galley(ui.painter(), label, Type::Body, fg);
    let gw = g.size().x;
    text::paint_in(ui.painter(), rect, g, Align::Min, fg);
    if s.hovered {
        let y = rect.center().y + 7.0;
        ui.painter()
            .hline(rect.left()..=rect.left() + gw, y, Stroke::new(1.0, fg));
    }
    if s.focused {
        state::focus_ring(ui, rect.expand(2.0), radius::ROW, c);
    }
    resp
}

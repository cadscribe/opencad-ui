//! Chips and tags: status chips (a dot and a tinted pill), value tags
//! ("U 0,21"), filter chips (selectable, optionally removable) and
//! anchor chips (an accent-tinted value with a menu).

use egui::{Align, Rect, Response, Role, Sense, Stroke, StrokeKind, Ui, Vec2, pos2, vec2};

use crate::icon::{self, Icon};
use crate::state;
use crate::text::{self, Type};
use crate::theme::colors;
use crate::tokens::{Tone, over, radius, size, space};

/// A status chip: a dot and the text in the tone's tint ("entspricht
/// Typ", "geändert").
pub fn status(ui: &mut Ui, label: &str, tone: Tone) -> Response {
    let c = colors(ui);
    let t = c.tone(tone);
    let g = text::galley(ui.painter(), label, Type::CaptionStrong, t.text);
    let w = space::MD + 6.0 + 5.0 + g.size().x + space::MD;
    let (rect, resp) = ui.allocate_exact_size(vec2(w, 20.0), Sense::hover());
    state::a11y(&resp, Role::Status, label);
    let p = ui.painter();
    p.rect_filled(rect, radius::PILL, t.tint);
    p.circle_filled(
        pos2(rect.left() + space::MD + 3.0, rect.center().y),
        3.0,
        t.solid,
    );
    text::paint_in(
        p,
        Rect::from_min_max(pos2(rect.left() + space::MD + 11.0, rect.top()), rect.max),
        g,
        Align::Min,
        t.text,
    );
    resp
}

/// A value tag: tabular text in a tinted pill, `width` fixed or natural
/// ("U 0,11" in the type picker).
pub fn tag(ui: &mut Ui, label: &str, tone: Tone, width: Option<f32>) -> Response {
    let c = colors(ui);
    let t = c.tone(tone);
    let tw = text::tabular_width(ui.painter(), label, Type::Caption);
    let w = width.unwrap_or(tw + 2.0 * space::MD);
    let (rect, resp) = ui.allocate_exact_size(vec2(w, 18.0), Sense::hover());
    state::a11y(&resp, Role::Label, label);
    ui.painter().rect_filled(rect, radius::PILL, t.tint);
    text::paint_tabular(
        ui.painter(),
        rect,
        label,
        Type::Caption,
        t.text,
        Align::Center,
    );
    resp
}

/// What happened to a filter chip.
#[derive(Debug, Clone)]
pub struct FilterChipResponse {
    pub response: Response,
    /// The chip was clicked (toggle it).
    pub clicked: bool,
    /// Its remove button was clicked.
    pub removed: bool,
}

/// A selectable filter chip (category filters); filled accent when
/// `selected`. `removable` adds a small × button.
pub fn filter(ui: &mut Ui, label: &str, selected: bool, removable: bool) -> FilterChipResponse {
    let c = colors(ui);
    let t = if selected {
        Type::CaptionStrong
    } else {
        Type::Caption
    };
    let g = text::galley(ui.painter(), label, t, c.text);
    let x_w = if removable { 14.0 } else { 0.0 };
    let w = g.size().x + 2.0 * space::LG + x_w;
    let (rect, resp) = ui.allocate_exact_size(vec2(w, size::FIELD_H), Sense::click());
    state::a11y_selected(&resp, Role::Button, selected, label);
    let s = state::state(ui, &resp);
    let p = ui.painter();
    let (fill, stroke, fg) = if selected {
        let fill = if s.hovered {
            c.accent_fill_hover
        } else {
            c.accent_fill
        };
        (fill, Stroke::NONE, c.on_accent)
    } else {
        let fill = if s.hovered {
            over(c.surface_raised, c.hover)
        } else {
            c.surface_raised
        };
        (
            fill,
            Stroke::new(1.0, c.border),
            state::text_color(s, c.icon, c),
        )
    };
    p.rect(rect, radius::PILL, fill, stroke, StrokeKind::Inside);
    let g = text::galley(p, label, t, fg);
    text::paint_in(
        p,
        Rect::from_min_max(pos2(rect.left() + space::LG, rect.top()), rect.max),
        g,
        Align::Min,
        fg,
    );
    let mut removed = false;
    if removable {
        let xr = Rect::from_center_size(
            pos2(rect.right() - space::LG - 3.0, rect.center().y),
            Vec2::splat(14.0),
        );
        let xresp = ui.interact(xr, resp.id.with("remove"), Sense::click());
        if xresp.hovered() {
            ui.painter().circle_filled(xr.center(), 7.0, c.hover);
        }
        icon::paint(ui.painter(), xr.shrink(2.0), Icon::Close, fg);
        removed = xresp.clicked();
    }
    if s.focused {
        state::focus_ring(ui, rect, radius::PILL, c);
    }
    let clicked = resp.clicked() && !removed;
    FilterChipResponse {
        response: resp,
        clicked,
        removed,
    }
}

/// An anchor chip: an accent-tinted value with a chevron that opens
/// `menu` ("Rohdecke darüber"). `None` shows "–" (differing values).
pub fn anchor(ui: &mut Ui, value: Option<&str>, menu: impl FnOnce(&mut Ui)) -> Response {
    let c = colors(ui);
    let text = value.unwrap_or("–");
    let max = ui.available_width().max(40.0);
    let g = text::truncated(
        ui.painter(),
        text,
        Type::BodyMedium,
        c.accent_text,
        max - 26.0,
    );
    let w = (g.size().x + 30.0).min(max);
    let (rect, resp) = ui.allocate_exact_size(vec2(w, size::FIELD_H - 2.0), Sense::click());
    state::a11y(&resp, Role::ComboBox, text);
    let s = state::state(ui, &resp);
    let p = ui.painter();
    let fill = if s.hovered {
        over(c.accent_tint, c.hover)
    } else {
        c.accent_tint
    };
    p.rect_filled(rect, radius::FIELD, fill);
    let fg = state::text_color(s, c.accent_text, c);
    let g = text::truncated(p, text, Type::BodyMedium, fg, rect.width() - 26.0);
    text::paint_in(
        p,
        Rect::from_min_max(pos2(rect.left() + space::MD, rect.top()), rect.max),
        g,
        Align::Min,
        fg,
    );
    icon::paint(
        p,
        Rect::from_center_size(
            pos2(rect.right() - 11.0, rect.center().y),
            Vec2::splat(12.0),
        ),
        Icon::ChevronDown,
        fg,
    );
    if s.focused {
        state::focus_ring(ui, rect, radius::FIELD, c);
    }
    super::menu::menu_popup(ui, &resp).show(menu);
    resp
}

/// What happened to a [`value`] chip.
#[derive(Debug, Clone)]
pub struct ValueChipResponse {
    pub response: Response,
    /// The chip was clicked or activated with Enter / Space (edit it).
    pub clicked: bool,
    /// Its remove button was clicked, or Delete / Backspace pressed
    /// while it had the focus.
    pub removed: bool,
}

/// Height of [`value`] and [`add`] chips.
pub const VALUE_H: f32 = 22.0;

/// An editable value in a list ("0,30", "1,15"): tabular figures in a
/// pill; `muted` (a default, not the user's own value) shows it in the
/// secondary colour without a border. On hover (or focus) a remove
/// button fades in at the right and the value moves aside for it;
/// `tip` is shown on hover.
pub fn value(ui: &mut Ui, label: &str, muted: bool, tip: &str) -> ValueChipResponse {
    let c = colors(ui);
    let tw = text::tabular_width(ui.painter(), label, Type::Body);
    let x_w = 12.0;
    let pad = space::SM;
    let w = tw + 2.0 * pad + x_w;
    let (rect, resp) = ui.allocate_exact_size(vec2(w, VALUE_H), Sense::click());
    state::a11y(&resp, Role::Button, label);
    let s = state::state(ui, &resp);
    let xr = Rect::from_center_size(
        pos2(rect.right() - pad / 2.0 - x_w / 2.0, rect.center().y),
        Vec2::splat(x_w),
    );
    let xresp = ui.interact(xr, resp.id.with("remove"), Sense::click());
    let show_x = s.hovered || s.focused || xresp.hovered();
    let t = ui
        .ctx()
        .animate_bool_with_time(resp.id.with("x"), show_x, crate::tokens::motion::FAST);
    let p = ui.painter();
    let (fill, stroke, fg) = if muted {
        let fill = if s.hovered {
            over(c.surface_sunken, c.hover)
        } else {
            c.surface_sunken
        };
        (fill, Stroke::NONE, c.text_secondary)
    } else {
        let fill = if s.hovered {
            over(c.field, c.hover)
        } else {
            c.field
        };
        let border = if s.hovered { c.border_strong } else { c.border };
        (fill, Stroke::new(1.0, border), c.text)
    };
    p.rect(rect, radius::PILL, fill, stroke, StrokeKind::Inside);
    // At rest the value is centred; with the remove button it moves left.
    let rest = rect.center().x - tw / 2.0;
    let left = rect.left() + pad;
    let x = rest + (left - rest) * t;
    text::paint_tabular(
        p,
        Rect::from_min_max(pos2(x, rect.top()), pos2(x + tw, rect.bottom())),
        label,
        Type::Body,
        state::text_color(s, fg, c),
        Align::Min,
    );
    if t > 0.0 {
        let col = if xresp.hovered() { c.text } else { c.icon };
        if xresp.hovered() {
            p.circle_filled(xr.center(), x_w / 2.0, c.hover);
        }
        icon::paint(p, xr.shrink(2.5), Icon::Close, col.gamma_multiply(t));
    }
    if s.focused {
        state::focus_ring(ui, rect, radius::PILL, c);
    }
    let key_remove = resp.has_focus()
        && ui.input(|i| i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace));
    let removed = (show_x && xresp.clicked()) || key_remove;
    let clicked = resp.clicked() && !removed;
    let response = if tip.is_empty() {
        resp
    } else {
        resp.on_hover_text(tip)
    };
    ValueChipResponse {
        response,
        clicked,
        removed,
    }
}

/// The "+" at the end of a list of [`value`] chips: a dashed pill with
/// a plus and an optional label; the caller turns it into a field when
/// clicked.
pub fn add(ui: &mut Ui, label: &str, tip: &str) -> Response {
    let c = colors(ui);
    let g = text::galley(ui.painter(), label, Type::Caption, c.text_secondary);
    let icon_w = 12.0;
    let text_w = if label.is_empty() {
        0.0
    } else {
        g.size().x + space::XS
    };
    let pad = if label.is_empty() {
        space::SM
    } else {
        space::MD
    };
    let w = (icon_w + text_w + 2.0 * pad).max(VALUE_H);
    let (rect, resp) = ui.allocate_exact_size(vec2(w, VALUE_H), Sense::click());
    state::a11y(
        &resp,
        Role::Button,
        if label.is_empty() { tip } else { label },
    );
    let s = state::state(ui, &resp);
    let p = ui.painter();
    let fg = if s.hovered {
        c.accent_text
    } else {
        c.text_secondary
    };
    let fg = state::text_color(s, fg, c);
    if s.hovered {
        p.rect_filled(rect, radius::PILL, c.accent_tint);
    }
    // A dashed outline with round ends (a pill of straight dashes).
    let r = rect.shrink(0.5);
    let rad = r.height() / 2.0;
    let mut pts = Vec::new();
    let steps = 8;
    for i in 0..=steps {
        let a = std::f32::consts::FRAC_PI_2 + std::f32::consts::PI * i as f32 / steps as f32;
        pts.push(pos2(
            r.left() + rad + rad * a.cos(),
            r.center().y - rad * a.sin(),
        ));
    }
    for i in 0..=steps {
        let a = -std::f32::consts::FRAC_PI_2 + std::f32::consts::PI * i as f32 / steps as f32;
        pts.push(pos2(
            r.right() - rad + rad * a.cos(),
            r.center().y - rad * a.sin(),
        ));
    }
    pts.push(pts[0]);
    let border = if s.hovered { c.accent } else { c.border_strong };
    p.extend(egui::Shape::dashed_line(
        &pts,
        Stroke::new(1.0, border),
        3.0,
        2.0,
    ));
    let total = icon_w + text_w;
    let x0 = rect.center().x - total / 2.0;
    icon::paint(
        p,
        Rect::from_center_size(
            pos2(x0 + icon_w / 2.0, rect.center().y),
            Vec2::splat(icon_w),
        ),
        Icon::Plus,
        fg,
    );
    if !label.is_empty() {
        let g = text::galley(p, label, Type::Caption, fg);
        text::paint_in(
            p,
            Rect::from_min_max(pos2(x0 + icon_w + space::XS, rect.top()), rect.max),
            g,
            Align::Min,
            fg,
        );
    }
    if s.focused {
        state::focus_ring(ui, rect, radius::PILL, c);
    }
    if tip.is_empty() {
        resp
    } else {
        resp.on_hover_text(tip)
    }
}

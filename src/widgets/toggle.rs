//! Switches, checkboxes and radio buttons. Each takes an optional label
//! on its right; clicking the label toggles too.

use egui::{
    Align, Color32, Pos2, Rect, Response, Role, Sense, Shape, Stroke, StrokeKind, Ui, Vec2, pos2,
    vec2,
};

use crate::state::{self, State};
use crate::text::{self, Type};
use crate::theme::colors;
use crate::tokens::{Colors, mix, motion, radius, space};

/// Allocates the control and its label; returns the whole rect, the
/// control's rect and the response.
fn layout(ui: &mut Ui, control: Vec2, label: &str) -> (Rect, Rect, Response) {
    let c = colors(ui);
    let label_w = if label.is_empty() {
        0.0
    } else {
        space::MD
            + text::galley(ui.painter(), label, Type::Body, c.text)
                .size()
                .x
    };
    let h = control.y.max(20.0);
    let w = (control.x + label_w).min(ui.available_width().max(control.x));
    let (rect, resp) = ui.allocate_exact_size(vec2(w, h), Sense::click());
    let ctl = Rect::from_min_size(
        pos2(rect.left(), rect.center().y - control.y / 2.0),
        control,
    );
    (rect, ctl, resp)
}

fn paint_label(ui: &Ui, rect: Rect, ctl: Rect, label: &str, s: State, c: &Colors) {
    if label.is_empty() {
        return;
    }
    let fg = state::text_color(s, c.text, c);
    let lr = Rect::from_min_max(pos2(ctl.right() + space::MD, rect.top()), rect.max);
    let g = text::truncated(ui.painter(), label, Type::Body, fg, lr.width());
    text::paint_in(ui.painter(), lr, g, Align::Min, fg);
}

/// A switch (on/off immediately applied).
pub fn switch(ui: &mut Ui, on: &mut bool, label: &str) -> Response {
    let c = colors(ui);
    let (rect, ctl, mut resp) = layout(ui, vec2(32.0, 18.0), label);
    if resp.clicked() {
        *on = !*on;
        resp.mark_changed();
    }
    state::a11y_selected(&resp, Role::Switch, *on, label);
    let s = state::state(ui, &resp);
    let t = state::animate(ui, resp.id, *on, motion::NORMAL);
    let off = if c.dark {
        Color32::from_rgb(0x48, 0x48, 0x4D)
    } else {
        Color32::from_rgb(0xD5, 0xD5, 0xDA)
    };
    let mut fill = mix(off, c.accent_fill, t);
    if s.hovered && !s.disabled {
        fill = mix(fill, c.text, 0.06);
    }
    if s.disabled {
        fill = mix(fill, c.surface_raised, 0.55);
    }
    let p = ui.painter();
    p.rect_filled(ctl, radius::PILL, fill);
    let knob_r = ctl.height() / 2.0 - 2.0;
    let x = egui::lerp(ctl.left() + knob_r + 2.0..=ctl.right() - knob_r - 2.0, t);
    let knob = pos2(x, ctl.center().y);
    p.circle_filled(
        knob + vec2(0.0, 0.5),
        knob_r + 0.5,
        Color32::from_black_alpha(40),
    );
    p.circle_filled(
        knob,
        knob_r,
        if s.disabled {
            mix(Color32::WHITE, fill, 0.2)
        } else {
            Color32::WHITE
        },
    );
    paint_label(ui, rect, ctl, label, s, c);
    if s.focused {
        state::focus_ring(ui, ctl, radius::PILL, c);
    }
    resp
}

/// The check state of a checkbox.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Check {
    Off,
    On,
    /// Differing values in a multi-selection ("–").
    Mixed,
}

fn paint_box(ui: &Ui, ctl: Rect, check: Check, s: State, t: f32, c: &Colors) {
    let p = ui.painter();
    let filled = check != Check::Off;
    let r = 4;
    if filled {
        let fill = if s.disabled {
            mix(c.surface_raised, c.accent_fill, 0.35)
        } else if s.pressed {
            c.accent_fill_pressed
        } else {
            c.accent_fill
        };
        p.rect_filled(ctl, r, fill);
    } else {
        let border = if s.hovered {
            c.border_mixed
        } else {
            c.border_strong
        };
        p.rect(
            ctl,
            r,
            if s.disabled {
                c.surface_sunken
            } else {
                c.field
            },
            Stroke::new(1.0, border),
            StrokeKind::Inside,
        );
    }
    let fg = c.on_accent;
    match check {
        Check::On => {
            // The check mark draws in.
            let a = ctl.center() + vec2(-3.6, 0.2);
            let b = ctl.center() + vec2(-1.0, 2.8);
            let d = ctl.center() + vec2(3.8, -2.8);
            let first = (b - a).length();
            let second = (d - b).length();
            let len = (first + second) * t;
            let mut pts: Vec<Pos2> = vec![a];
            if len <= first {
                pts.push(a + (b - a) * (len / first));
            } else {
                pts.push(b);
                pts.push(b + (d - b) * ((len - first) / second));
            }
            p.add(Shape::line(pts, Stroke::new(1.8, fg)));
        }
        Check::Mixed => {
            p.hline(
                ctl.center().x - 3.5..=ctl.center().x + 3.5,
                ctl.center().y,
                Stroke::new(1.8, fg),
            );
        }
        Check::Off => {}
    }
}

/// A checkbox; `true` in the response's `changed()` when toggled.
pub fn checkbox(ui: &mut Ui, on: &mut bool, label: &str) -> Response {
    let mut check = if *on { Check::On } else { Check::Off };
    let resp = checkbox_tri(ui, &mut check, label);
    if resp.changed() {
        *on = check == Check::On;
    }
    resp
}

/// A checkbox with the mixed state; clicking a mixed box turns it on.
pub fn checkbox_tri(ui: &mut Ui, check: &mut Check, label: &str) -> Response {
    let c = colors(ui);
    let (rect, ctl, mut resp) = layout(ui, vec2(14.0, 14.0), label);
    if resp.clicked() {
        *check = match *check {
            Check::On => Check::Off,
            Check::Off | Check::Mixed => Check::On,
        };
        resp.mark_changed();
    }
    let enabled = resp.enabled();
    let selected = *check == Check::On;
    resp.widget_info(|| {
        let mut info = egui::WidgetInfo::selected(Role::CheckBox, enabled, selected, label);
        if *check == Check::Mixed {
            info.selected = None;
        }
        info
    });
    let s = state::state(ui, &resp);
    let t = state::animate(ui, resp.id, *check == Check::On, motion::NORMAL);
    paint_box(
        ui,
        ctl,
        *check,
        s,
        if *check == Check::On {
            t.max(0.05)
        } else {
            1.0
        },
        c,
    );
    paint_label(ui, rect, ctl, label, s, c);
    if s.focused {
        state::focus_ring(ui, ctl, 4, c);
    }
    resp
}

/// A radio button; returns its response (clicked = chosen).
pub fn radio(ui: &mut Ui, selected: bool, label: &str) -> Response {
    let c = colors(ui);
    let (rect, ctl, resp) = layout(ui, vec2(14.0, 14.0), label);
    state::a11y_selected(&resp, Role::RadioButton, selected, label);
    let s = state::state(ui, &resp);
    let t = state::animate(ui, resp.id, selected, motion::NORMAL);
    let p = ui.painter();
    let center = ctl.center();
    if selected {
        let fill = if s.disabled {
            mix(c.surface_raised, c.accent_fill, 0.35)
        } else {
            c.accent_fill
        };
        p.circle_filled(center, 7.0, fill);
        p.circle_filled(center, 2.8 * t.max(0.3), c.on_accent);
    } else {
        let border = if s.hovered {
            c.border_mixed
        } else {
            c.border_strong
        };
        p.circle(
            center,
            6.5,
            if s.disabled {
                c.surface_sunken
            } else {
                c.field
            },
            Stroke::new(1.0, border),
        );
    }
    paint_label(ui, rect, ctl, label, s, c);
    if s.focused {
        state::focus_ring(ui, ctl, radius::PILL, c);
    }
    resp
}

/// A radio button that sets `value` to `option` when chosen; `changed()`
/// on the response when it did.
pub fn radio_value<T: PartialEq + Copy>(
    ui: &mut Ui,
    value: &mut T,
    option: T,
    label: &str,
) -> Response {
    let mut resp = radio(ui, *value == option, label);
    if resp.clicked() && *value != option {
        *value = option;
        resp.mark_changed();
    }
    resp
}

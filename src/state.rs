//! Interaction states shared by every widget: hovered, pressed, focused,
//! disabled; the focus ring; short animations; AccessKit labels; and a
//! forced state for the gallery (to show "hover" or "focus" without a
//! pointer).

use egui::{Color32, CornerRadius, Id, Rect, Response, Role, Stroke, StrokeKind, Ui, WidgetInfo};

use crate::tokens::{Colors, motion, size};

/// The visible state of a widget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct State {
    pub hovered: bool,
    pub pressed: bool,
    pub focused: bool,
    pub disabled: bool,
}

/// A state the gallery forces onto the widgets it draws.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Forced {
    Rest,
    Hover,
    Pressed,
    Focus,
    Disabled,
}

impl Forced {
    pub const ALL: [Forced; 5] = [
        Forced::Rest,
        Forced::Hover,
        Forced::Pressed,
        Forced::Focus,
        Forced::Disabled,
    ];
}

fn forced_id() -> Id {
    Id::unique("opencad_ui_forced_state")
}

/// Draws `add` with every widget in it showing `state` (gallery).
pub fn with_forced<R>(ui: &mut Ui, state: Forced, add: impl FnOnce(&mut Ui) -> R) -> R {
    let before = ui.data(|d| d.get_temp::<Forced>(forced_id()));
    ui.data_mut(|d| d.insert_temp(forced_id(), state));
    let r = ui
        .scope(|ui| {
            if state == Forced::Disabled {
                ui.disable();
            }
            add(ui)
        })
        .inner;
    ui.data_mut(|d| match before {
        Some(b) => {
            d.insert_temp(forced_id(), b);
        }
        None => d.remove::<Forced>(forced_id()),
    });
    r
}

/// The state of `resp`, or the forced one.
pub fn state(ui: &Ui, resp: &Response) -> State {
    let forced = ui.data(|d| d.get_temp::<Forced>(forced_id()));
    let disabled = !ui.is_enabled() || !resp.enabled();
    match forced {
        Some(Forced::Hover) => State {
            hovered: true,
            disabled,
            ..Default::default()
        },
        Some(Forced::Pressed) => State {
            hovered: true,
            pressed: true,
            disabled,
            ..Default::default()
        },
        Some(Forced::Focus) => State {
            focused: true,
            disabled,
            ..Default::default()
        },
        Some(Forced::Rest | Forced::Disabled) => State {
            disabled,
            ..Default::default()
        },
        None => State {
            hovered: resp.hovered() && !disabled,
            pressed: resp.is_pointer_button_down_on() && !disabled,
            focused: resp.has_focus() && !disabled,
            disabled,
        },
    }
}

/// Whether the gallery forces a state (some widgets show their open or
/// active form then).
pub fn forced(ui: &Ui) -> Option<Forced> {
    ui.data(|d| d.get_temp::<Forced>(forced_id()))
}

/// The soft focus ring around `rect`: the accent outline and the wide,
/// light ring of the mockups.
pub fn focus_ring(ui: &Ui, rect: Rect, corner: impl Into<CornerRadius>, c: &Colors) {
    let corner = corner.into();
    let p = ui.painter();
    p.rect_stroke(
        rect.expand(0.5),
        CornerRadius {
            nw: corner.nw.saturating_add(1),
            ne: corner.ne.saturating_add(1),
            sw: corner.sw.saturating_add(1),
            se: corner.se.saturating_add(1),
        },
        Stroke::new(size::FOCUS_RING, c.focus),
        StrokeKind::Outside,
    );
    p.rect_stroke(rect, corner, Stroke::new(1.0, c.accent), StrokeKind::Inside);
}

/// A 0…1 animation towards `on` (hover fades, switches, chevrons).
pub fn animate(ui: &Ui, id: Id, on: bool, duration: f32) -> f32 {
    ui.ctx()
        .animate_bool_with_time_and_easing(id, on, duration, motion::ease_out)
}

/// The hover overlay of a flat control, faded in.
pub fn hover_fill(ui: &Ui, id: Id, s: State, c: &Colors) -> Color32 {
    let t = animate(ui, id.with("hover"), s.hovered, motion::FAST);
    if s.pressed {
        c.pressed
    } else {
        c.hover.gamma_multiply(t)
    }
}

/// Reports the role and label of a widget to AccessKit.
pub fn a11y(resp: &Response, role: Role, label: &str) {
    let enabled = resp.enabled();
    resp.widget_info(|| WidgetInfo::labeled(role, enabled, label));
}

/// Reports a selectable widget (toggle, tab, segment) to AccessKit.
pub fn a11y_selected(resp: &Response, role: Role, selected: bool, label: &str) {
    let enabled = resp.enabled();
    resp.widget_info(|| WidgetInfo::selected(role, enabled, selected, label));
}

/// The text colour for the state: `normal`, or the disabled text.
pub fn text_color(s: State, normal: Color32, c: &Colors) -> Color32 {
    if s.disabled { c.text_disabled } else { normal }
}

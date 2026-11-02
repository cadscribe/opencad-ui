//! A splitter: two panes side by side (or stacked) with a draggable
//! divider; the fraction is remembered per id. The divider is focusable
//! (←/→ or ↑/↓ move it) and resets on double click.

use egui::{CursorIcon, Id, Key, Rect, Response, Role, Sense, Stroke, Ui, UiBuilder, pos2, vec2};

use crate::state;
use crate::theme::colors;
use crate::tokens::motion;

crate::messages! {
    enum Msg {
        Divider: "Resize panes", "Bereiche anpassen";
    }
}

/// The direction of the split.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    /// Panes side by side, a vertical divider.
    Horizontal,
    /// Panes stacked, a horizontal divider.
    Vertical,
}

/// A two-pane splitter.
#[derive(Debug, Clone, Copy)]
#[must_use = "call `.show(ui, first, second)`"]
pub struct Splitter {
    id: Id,
    axis: Axis,
    initial: f32,
    min: f32,
    height: Option<f32>,
}

impl Splitter {
    pub fn new(id_salt: impl std::hash::Hash + std::fmt::Debug, axis: Axis) -> Self {
        Splitter {
            id: Id::unique(("opencad_ui_splitter", id_salt)),
            axis,
            initial: 0.5,
            min: 80.0,
            height: None,
        }
    }

    /// The first pane's share at the start (0..1).
    pub fn initial(mut self, f: f32) -> Self {
        self.initial = f.clamp(0.05, 0.95);
        self
    }

    /// The smallest pane size in points.
    pub fn min_size(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    /// The height of the whole splitter (else the available height).
    pub fn height(mut self, h: f32) -> Self {
        self.height = Some(h);
        self
    }

    /// Shows the panes; returns the divider's response.
    pub fn show(
        self,
        ui: &mut Ui,
        first: impl FnOnce(&mut Ui),
        second: impl FnOnce(&mut Ui),
    ) -> Response {
        let c = colors(ui);
        let h = self.height.unwrap_or(ui.available_height());
        let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), h), Sense::hover());
        let mut f: f32 = ui.data(|d| d.get_temp(self.id)).unwrap_or(self.initial);
        let len = match self.axis {
            Axis::Horizontal => rect.width(),
            Axis::Vertical => rect.height(),
        };
        let grip = 6.0;
        let min_f = (self.min / len).min(0.45);
        let at = |f: f32| match self.axis {
            Axis::Horizontal => rect.left() + len * f,
            Axis::Vertical => rect.top() + len * f,
        };
        let handle = |f: f32| match self.axis {
            Axis::Horizontal => Rect::from_min_max(
                pos2(at(f) - grip / 2.0, rect.top()),
                pos2(at(f) + grip / 2.0, rect.bottom()),
            ),
            Axis::Vertical => Rect::from_min_max(
                pos2(rect.left(), at(f) - grip / 2.0),
                pos2(rect.right(), at(f) + grip / 2.0),
            ),
        };
        let resp = ui.interact(handle(f), self.id.with("handle"), Sense::click_and_drag());
        state::a11y(&resp, Role::Splitter, crate::i18n::tr(Msg::Divider));
        if resp.dragged()
            && let Some(p) = ui.ctx().pointer_interact_pos()
        {
            f = match self.axis {
                Axis::Horizontal => (p.x - rect.left()) / len,
                Axis::Vertical => (p.y - rect.top()) / len,
            };
        }
        if resp.double_clicked() {
            f = self.initial;
        }
        if resp.has_focus() {
            let (dec, inc) = ui.input(|i| match self.axis {
                Axis::Horizontal => (
                    i.key_pressed(Key::ArrowLeft),
                    i.key_pressed(Key::ArrowRight),
                ),
                Axis::Vertical => (i.key_pressed(Key::ArrowUp), i.key_pressed(Key::ArrowDown)),
            });
            f += (f32::from(u8::from(inc)) - f32::from(u8::from(dec))) * 0.02;
        }
        f = f.clamp(min_f, 1.0 - min_f);
        ui.data_mut(|d| d.insert_temp(self.id, f));
        let s = state::state(ui, &resp);
        let active = s.hovered || resp.dragged() || s.focused;
        let t = state::animate(ui, self.id.with("hot"), active, motion::FAST);
        let line = egui::lerp(1.0..=2.0, t);
        let col = crate::tokens::mix(c.border, c.accent, t);
        let pos = at(f);
        let p = ui.painter();
        match self.axis {
            Axis::Horizontal => p.vline(pos, rect.y_range(), Stroke::new(line, col)),
            Axis::Vertical => p.hline(rect.x_range(), pos, Stroke::new(line, col)),
        };
        if active {
            ui.ctx().set_cursor_icon(match self.axis {
                Axis::Horizontal => CursorIcon::ResizeHorizontal,
                Axis::Vertical => CursorIcon::ResizeVertical,
            });
        }
        let (a, b) = match self.axis {
            Axis::Horizontal => (
                Rect::from_min_max(rect.min, pos2(pos - grip / 2.0, rect.bottom())),
                Rect::from_min_max(pos2(pos + grip / 2.0, rect.top()), rect.max),
            ),
            Axis::Vertical => (
                Rect::from_min_max(rect.min, pos2(rect.right(), pos - grip / 2.0)),
                Rect::from_min_max(pos2(rect.left(), pos + grip / 2.0), rect.max),
            ),
        };
        let mut ua = ui.new_child(UiBuilder::new().max_rect(a));
        ua.set_clip_rect(a.intersect(ui.clip_rect()));
        first(&mut ua);
        let mut ub = ui.new_child(UiBuilder::new().max_rect(b));
        ub.set_clip_rect(b.intersect(ui.clip_rect()));
        second(&mut ub);
        resp
    }
}

/// A divider between two docked panes laid out by the caller: `gap` is
/// the gutter between them inside `rect`, along `axis`. Dragging moves
/// it; the new fraction of `rect` is returned while it changes (the
/// caller keeps it in its layout model; a double click resets it to the
/// middle). Drawn only while hovered, dragged or focused: at rest the
/// panes' cards separate themselves.
pub fn divider(
    ui: &mut Ui,
    id_salt: impl std::hash::Hash + std::fmt::Debug,
    rect: Rect,
    gap: Rect,
    axis: Axis,
    min: f32,
) -> (Response, Option<f32>) {
    let c = colors(ui);
    let id = Id::unique(("opencad_ui_divider", id_salt));
    let grip = match axis {
        Axis::Horizontal => gap.expand2(vec2(2.0, 0.0)),
        Axis::Vertical => gap.expand2(vec2(0.0, 2.0)),
    };
    let resp = ui.interact(grip, id, Sense::click_and_drag());
    state::a11y(&resp, Role::Splitter, crate::i18n::tr(Msg::Divider));
    let len = match axis {
        Axis::Horizontal => rect.width(),
        Axis::Vertical => rect.height(),
    }
    .max(1.0);
    let min_f = (min / len).min(0.45);
    let mut out = None;
    if resp.dragged()
        && let Some(p) = ui.ctx().pointer_interact_pos()
    {
        let f = match axis {
            Axis::Horizontal => (p.x - rect.left()) / len,
            Axis::Vertical => (p.y - rect.top()) / len,
        };
        out = Some(f.clamp(min_f, 1.0 - min_f));
    }
    if resp.double_clicked() {
        out = Some(0.5);
    }
    let s = state::state(ui, &resp);
    let active = s.hovered || resp.dragged() || s.focused;
    let t = state::animate(ui, id.with("hot"), active, motion::FAST);
    if t > 0.0 {
        let col = c.accent.gamma_multiply(t);
        let p = ui.painter();
        match axis {
            Axis::Horizontal => p.vline(gap.center().x, gap.y_range(), Stroke::new(2.0, col)),
            Axis::Vertical => p.hline(gap.x_range(), gap.center().y, Stroke::new(2.0, col)),
        };
    }
    if active {
        ui.ctx().set_cursor_icon(match axis {
            Axis::Horizontal => CursorIcon::ResizeHorizontal,
            Axis::Vertical => CursorIcon::ResizeVertical,
        });
    }
    (resp, out)
}

//! Tabs: attached tabs over a card (the palette), underline tabs (the
//! ribbon), and document tabs of a pane (icon, title, note, close
//! button, drag to reorder, "+" to add).

use egui::{Align, CornerRadius, Id, Rect, Role, Sense, Stroke, Ui, Vec2, pos2, vec2};

use crate::icon::{self, Icon};
use crate::state;
use crate::text::{self, Type};
use crate::theme::colors;
use crate::tokens::{motion, radius, size, space};

crate::messages! {
    enum Msg {
        CloseTab: "Close tab", "Tab schließen";
        NewTab: "New view", "Neue Ansicht";
    }
}

/// Tabs attached to the card below (palette tabs): the selected tab in
/// the card colour with icon and name, the others icon-only (or with
/// names when there is room). Returns the clicked index.
pub fn attached(ui: &mut Ui, items: &[(Icon, &str)], selected: usize) -> Option<usize> {
    let c = colors(ui);
    let w = ui.available_width();
    let n = items.len().max(1);
    let gap = 2.0;
    let names_fit = {
        let total: f32 = items
            .iter()
            .map(|(_, t)| {
                text::galley(ui.painter(), t, Type::BodyStrong, c.text)
                    .size()
                    .x
                    + 44.0
            })
            .sum();
        total + gap * (n - 1) as f32 <= w
    };
    let icon_w = 28.0;
    // The tab ids derive from this strip's own (automatic) id, not from
    // the scope: sibling strips in one scope (the gallery's light and dark
    // columns) would otherwise share ids and the last one take every click.
    let (rect, strip) = ui.allocate_exact_size(vec2(w, 28.0), Sense::hover());
    let p = ui.painter().clone();
    let mut clicked = None;
    let mut x = rect.left();
    let sel_w = if names_fit {
        (w - gap * (n - 1) as f32) / n as f32
    } else {
        (w - (n - 1) as f32 * (icon_w + gap)).max(icon_w)
    };
    for (i, (ic, name)) in items.iter().enumerate() {
        let tw = if names_fit || i == selected {
            sel_w
        } else {
            icon_w
        };
        let r = Rect::from_min_size(pos2(x, rect.top()), vec2(tw, rect.height()));
        x += tw + gap;
        let resp = ui.interact(r, strip.id.with(("attached_tab", i)), Sense::click());
        state::a11y_selected(&resp, Role::Tab, i == selected, name);
        let s = state::state(ui, &resp);
        let corners = CornerRadius {
            nw: radius::FIELD,
            ne: radius::FIELD,
            sw: 0,
            se: 0,
        };
        let on = i == selected;
        if on {
            p.rect_filled(r, corners, c.surface_raised);
        } else if s.hovered {
            p.rect_filled(r, corners, c.hover);
        }
        let fg = if on { c.text } else { c.text_secondary };
        let t = if on { Type::BodyStrong } else { Type::Body };
        let g = text::galley(&p, name, t, fg);
        let content = 16.0 + space::SM + g.size().x;
        if (on || names_fit) && content + 12.0 <= tw {
            let left = r.center().x - content / 2.0;
            icon::paint(
                &p,
                Rect::from_center_size(pos2(left + 8.0, r.center().y), Vec2::splat(16.0)),
                *ic,
                if on { c.accent } else { c.icon },
            );
            let gw = g.size().x;
            text::paint_in(
                &p,
                Rect::from_min_max(
                    pos2(left + 22.0, r.top()),
                    pos2(left + 22.0 + gw, r.bottom()),
                ),
                g,
                Align::Min,
                fg,
            );
        } else {
            icon::paint(
                &p,
                Rect::from_center_size(r.center(), Vec2::splat(16.0)),
                *ic,
                if on { c.accent } else { c.icon },
            );
        }
        if s.focused {
            state::focus_ring(ui, r.shrink(1.0), radius::ROW, c);
        }
        if resp.on_hover_text(*name).clicked() {
            clicked = Some(i);
        }
    }
    clicked
}

/// Underline tabs (ribbon): the selected one semibold with an accent
/// bar sliding under it. `true` when the selection changed.
pub fn underline(ui: &mut Ui, items: &[&str], selected: &mut usize) -> bool {
    let c = colors(ui);
    let h = 36.0;
    let mut changed = false;
    let mut rects = Vec::with_capacity(items.len());
    let mut bar_id = None;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = space::XS;
        for (i, name) in items.iter().enumerate() {
            let w = text::galley(ui.painter(), name, Type::BodyStrong, c.text)
                .size()
                .x
                + 2.0 * space::XL;
            let (r, resp) = ui.allocate_exact_size(vec2(w, h), Sense::click());
            state::a11y_selected(&resp, Role::Tab, i == *selected, name);
            let s = state::state(ui, &resp);
            let on = i == *selected;
            let fg = if on || s.hovered {
                c.text
            } else {
                c.text_secondary
            };
            let g = text::galley(
                ui.painter(),
                name,
                if on { Type::BodyStrong } else { Type::Body },
                fg,
            );
            text::paint_in(ui.painter(), r, g, Align::Center, fg);
            if s.focused {
                state::focus_ring(ui, r.shrink2(vec2(2.0, 6.0)), radius::ROW, c);
            }
            if resp.clicked() && !on {
                *selected = i;
                changed = true;
            }
            rects.push(r);
            bar_id.get_or_insert(resp.id.with("underline_bar"));
        }
    });
    if let Some(r) = rects.get(*selected) {
        let id = bar_id.unwrap_or(egui::Id::NULL);
        let l = ui
            .ctx()
            .animate_value_with_time(id.with("l"), r.left(), motion::NORMAL);
        let rr = ui
            .ctx()
            .animate_value_with_time(id.with("r"), r.right(), motion::NORMAL);
        let bar = Rect::from_min_max(pos2(l + 2.0, r.bottom() - 2.0), pos2(rr - 2.0, r.bottom()));
        ui.painter().rect_filled(bar, 1.0, c.accent);
    }
    changed
}

/// A document tab of a pane.
#[derive(Debug, Clone, Copy)]
pub struct DocTab<'a> {
    pub icon: Option<Icon>,
    pub title: &'a str,
    /// A secondary note ("1:100").
    pub note: &'a str,
    pub closable: bool,
}

/// What happened in a tab strip.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TabEvent {
    Select(usize),
    Close(usize),
    /// Drag-reordered: from, to.
    Move(usize, usize),
    /// A tab dragged and released away from the strip (to dock it
    /// elsewhere or float it; the pointer position says where).
    Detach(usize),
    Add,
}

/// A drawn tab strip: its events, the strip rect, the tabs' responses
/// (context menus, double clicks, drags) and the "+" button's response.
#[derive(Debug)]
pub struct TabStrip {
    pub events: Vec<TabEvent>,
    pub strip: Rect,
    pub tabs: Vec<egui::Response>,
    pub add: Option<egui::Response>,
    /// The slot next to the selected tab (see [`doc_tab_strip_slot`]);
    /// zero wide without one.
    pub slot: Rect,
    /// The tabs' width without the slot.
    pub tabs_w: f32,
}

impl TabStrip {
    /// The tab being dragged away from the strip, if any.
    pub fn dragged_out(&self, ui: &Ui) -> Option<usize> {
        let pos = ui.ctx().pointer_interact_pos()?;
        self.tabs
            .iter()
            .position(egui::Response::dragged)
            .filter(|_| !in_strip(self.strip, pos))
    }
}

/// `true` if `pos` is close enough to the strip to reorder rather than
/// detach.
pub fn in_strip(strip: Rect, pos: egui::Pos2) -> bool {
    strip.expand2(vec2(0.0, space::MD)).contains(pos)
}

/// The tab strip of a pane: rounded top tabs, the selected one raised
/// into the pane, a close button on hover (always on the selected one),
/// drag to reorder, optionally "+" at the end. Space for buttons on the
/// right is left free (`trailing_w`).
pub fn doc_tabs(
    ui: &mut Ui,
    id_salt: impl std::hash::Hash + std::fmt::Debug,
    tabs: &[DocTab<'_>],
    selected: usize,
    add_button: bool,
    trailing_w: f32,
) -> Vec<TabEvent> {
    doc_tab_strip(ui, id_salt, tabs, selected, add_button, trailing_w).events
}

/// [`doc_tabs`] with the strip and the responses of the tabs and of the
/// "+" button (docking: drag out, context menu, double click). A tab
/// released away from the strip is [`TabEvent::Detach`]ed.
pub fn doc_tab_strip(
    ui: &mut Ui,
    id_salt: impl std::hash::Hash + std::fmt::Debug,
    tabs: &[DocTab<'_>],
    selected: usize,
    add_button: bool,
    trailing_w: f32,
) -> TabStrip {
    doc_tab_strip_slot(ui, id_salt, tabs, selected, add_button, trailing_w, 0.0)
}

/// [`doc_tab_strip`] with a slot `slot_w` wide right of the selected tab
/// (a compact toolbar docked in the tab header, [`crate::widgets::floatbar`]).
pub fn doc_tab_strip_slot(
    ui: &mut Ui,
    id_salt: impl std::hash::Hash + std::fmt::Debug,
    tabs: &[DocTab<'_>],
    selected: usize,
    add_button: bool,
    trailing_w: f32,
    slot_w: f32,
) -> TabStrip {
    let c = colors(ui);
    let id = Id::unique(("opencad_ui_doc_tabs", id_salt));
    let mut events = Vec::new();
    let strip_h = size::TAB_STRIP_H;
    let (strip, _) = ui.allocate_exact_size(vec2(ui.available_width(), strip_h), Sense::hover());
    let p = ui.painter().clone();
    let tab_h = 30.0;
    let mut x = strip.left() + space::MD;
    let slot_gap = if slot_w > 0.0 {
        slot_w + space::SM
    } else {
        0.0
    };
    let max_right = strip.right() - trailing_w - if add_button { 30.0 } else { 0.0 } - slot_gap;
    let mut rects = Vec::with_capacity(tabs.len());
    let mut responses = Vec::with_capacity(tabs.len());
    let mut slot = Rect::NOTHING;
    let mut tabs_w = 0.0;
    for (k, t) in tabs.iter().enumerate() {
        let tw = text::galley(&p, t.title, Type::BodyStrong, c.text).size().x;
        let nw = if t.note.is_empty() {
            0.0
        } else {
            text::galley(&p, t.note, Type::Body, c.text).size().x + space::MD
        };
        let w = (2.0 * space::XL
            + if t.icon.is_some() {
                13.0 + space::MD
            } else {
                0.0
            }
            + tw
            + nw
            + if t.closable { 16.0 + space::MD } else { 0.0 })
        .min(220.0);
        let w = w.min((max_right - x).max(60.0));
        rects.push(Rect::from_min_size(
            pos2(x, strip.bottom() - tab_h),
            vec2(w, tab_h),
        ));
        x += w + 2.0;
        tabs_w += w + 2.0;
        if k == selected {
            let h = crate::widgets::floatbar::HEADER_H;
            slot = Rect::from_min_size(
                pos2(x + space::XS, strip.bottom() - tab_h / 2.0 - h / 2.0 - 1.0),
                vec2(slot_w, h),
            );
            x += slot_gap;
        }
    }
    let dragging: Option<usize> = ui.data(|d| d.get_temp(id.with("drag")));
    for (i, (t, r)) in tabs.iter().zip(&rects).enumerate() {
        let resp = ui.interact(*r, id.with(i), Sense::click_and_drag());
        state::a11y_selected(&resp, Role::Tab, i == selected, t.title);
        let on = i == selected;
        let s = state::state(ui, &resp);
        let corners = CornerRadius {
            nw: radius::CONTROL,
            ne: radius::CONTROL,
            sw: 0,
            se: 0,
        };
        if on {
            p.rect_filled(*r, corners, c.surface_raised);
            // The hairline around the raised tab, open at the bottom.
            let rr = r.expand2(vec2(0.5, 0.0));
            p.add(egui::Shape::line(
                vec![
                    rr.left_bottom(),
                    rr.left_top() + vec2(0.0, 4.0),
                    rr.left_top() + vec2(4.0, 0.0),
                    rr.right_top() - vec2(4.0, 0.0),
                    rr.right_top() + vec2(0.0, 4.0),
                    rr.right_bottom(),
                ],
                Stroke::new(1.0, c.divider),
            ));
        } else if s.hovered {
            p.rect_filled(
                r.shrink2(vec2(0.0, 3.0)).translate(vec2(0.0, -1.0)),
                radius::FIELD,
                c.hover,
            );
        }
        if dragging == Some(i) {
            p.rect_filled(*r, corners, c.accent_tint.gamma_multiply(0.6));
        }
        let fg = if on { c.text } else { c.text_secondary };
        let mut cx = r.left() + space::XL;
        if let Some(ic) = t.icon {
            icon::paint(
                &p,
                Rect::from_center_size(pos2(cx + 6.5, r.center().y), Vec2::splat(13.0)),
                ic,
                if on { c.accent } else { c.text_secondary },
            );
            cx += 13.0 + space::MD;
        }
        let close_w = if t.closable { 16.0 + space::SM } else { 0.0 };
        let avail = r.right() - space::XL - close_w - cx;
        let g = text::truncated(
            &p,
            t.title,
            if on { Type::BodyStrong } else { Type::Body },
            fg,
            avail,
        );
        let gw = g.size().x;
        text::paint_in(
            &p,
            Rect::from_min_max(pos2(cx, r.top()), pos2(cx + gw, r.bottom())),
            g,
            Align::Min,
            fg,
        );
        if !t.note.is_empty() && gw + 30.0 < avail {
            let ng = text::truncated(
                &p,
                t.note,
                Type::Body,
                c.text_tertiary,
                avail - gw - space::MD,
            );
            text::paint_in(
                &p,
                Rect::from_min_max(
                    pos2(cx + gw + space::MD, r.top()),
                    pos2(r.right(), r.bottom()),
                ),
                ng,
                Align::Min,
                c.text_tertiary,
            );
        }
        if t.closable && (on || resp.hovered()) {
            let xr = Rect::from_center_size(
                pos2(r.right() - space::XL + 2.0 - 8.0, r.center().y),
                Vec2::splat(16.0),
            );
            let xresp = ui.interact(xr, id.with(("close", i)), Sense::click());
            state::a11y(&xresp, Role::Button, crate::i18n::tr(Msg::CloseTab));
            if xresp.hovered() {
                p.rect_filled(xr, 4, c.hover);
            }
            icon::paint(&p, xr.shrink(3.0), crate::Icon::Close, c.text_tertiary);
            if xresp.clicked() {
                events.push(TabEvent::Close(i));
            }
        }
        if resp.clicked() && !on {
            events.push(TabEvent::Select(i));
        }
        if resp.middle_clicked() && t.closable {
            events.push(TabEvent::Close(i));
        }
        if resp.drag_started() {
            ui.data_mut(|d| d.insert_temp(id.with("drag"), i));
        }
        if resp.drag_stopped() {
            ui.data_mut(|d| d.remove::<usize>(id.with("drag")));
            let pos = ui.ctx().pointer_interact_pos();
            if pos.is_some_and(|q| !in_strip(strip, q)) {
                events.push(TabEvent::Detach(i));
            } else if let Some(pos) = pos {
                let to = rects
                    .iter()
                    .position(|q| q.x_range().contains(pos.x))
                    .unwrap_or(if pos.x > r.center().x {
                        tabs.len() - 1
                    } else {
                        0
                    });
                if to != i {
                    events.push(TabEvent::Move(i, to));
                }
            }
        }
        if s.focused {
            state::focus_ring(ui, r.shrink(2.0), radius::FIELD, c);
        }
        responses.push(resp);
    }
    // Drop marker while dragging.
    if let (Some(from), Some(pos)) = (dragging, ui.ctx().pointer_interact_pos())
        && in_strip(strip, pos)
        && let Some(to) = rects.iter().position(|q| q.x_range().contains(pos.x))
        && to != from
    {
        let mx = if to > from {
            rects[to].right() + 1.0
        } else {
            rects[to].left() - 1.0
        };
        p.vline(mx, rects[to].y_range(), Stroke::new(2.0, c.accent));
    }
    let mut add = None;
    if add_button {
        let ar = Rect::from_min_size(pos2(x + 2.0, strip.bottom() - 28.0), Vec2::splat(26.0));
        let aresp = ui.interact(ar, id.with("add"), Sense::click());
        state::a11y(&aresp, Role::Button, crate::i18n::tr(Msg::NewTab));
        if aresp.hovered() {
            p.rect_filled(ar, radius::FIELD, c.hover);
        }
        icon::paint(&p, ar.shrink(6.0), crate::Icon::Plus, c.text_secondary);
        let aresp = aresp.on_hover_text(crate::i18n::tr(Msg::NewTab));
        if aresp.clicked() {
            events.push(TabEvent::Add);
        }
        add = Some(aresp);
    }
    TabStrip {
        events,
        strip,
        tabs: responses,
        add,
        slot,
        tabs_w,
    }
}

/// Applies a [`TabEvent::Move`] to a list.
pub fn apply_move<T>(items: &mut Vec<T>, from: usize, to: usize) {
    if from < items.len() && to < items.len() {
        let item = items.remove(from);
        items.insert(to, item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moving_tabs() {
        let mut v = vec!['a', 'b', 'c', 'd'];
        apply_move(&mut v, 0, 2);
        assert_eq!(v, vec!['b', 'c', 'a', 'd']);
        apply_move(&mut v, 3, 0);
        assert_eq!(v, vec!['d', 'b', 'c', 'a']);
        apply_move(&mut v, 9, 0);
        assert_eq!(v.len(), 4);
    }

    /// Two attached tab strips in sibling columns (one id scope, as in the
    /// gallery): a click in the first column reaches the first strip.
    #[test]
    fn sibling_tab_strips_take_their_own_clicks() {
        use egui::{Event, PointerButton, RawInput};
        let ctx = egui::Context::default();
        crate::install(&ctx, |_| {});
        let items = [
            (Icon::Properties, "A"),
            (Icon::Library, "B"),
            (Icon::RoomBook, "C"),
        ];
        let screen = Rect::from_min_size(pos2(0.0, 0.0), vec2(800.0, 200.0));
        let frame = |events: Vec<Event>| {
            let input = RawInput {
                screen_rect: Some(screen),
                events,
                ..Default::default()
            };
            let mut clicked = [None; 2];
            let mut rects = [Rect::NOTHING; 2];
            let mut out = ctx.run_ui(input, |ui| {
                ui.columns(2, |cols| {
                    for (i, col) in cols.iter_mut().enumerate() {
                        rects[i] = col.max_rect();
                        clicked[i] = attached(col, &items, 0);
                    }
                });
            });
            out.textures_delta.clear();
            (clicked, rects)
        };
        let (_, rects) = frame(vec![]);
        // The last tab of the first (left) column.
        let r = rects[0];
        let at = pos2(r.left() + r.width() * 5.0 / 6.0, r.top() + 14.0);
        let button = |pressed| Event::PointerButton {
            pos: at,
            button: PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        frame(vec![Event::PointerMoved(at)]);
        frame(vec![button(true)]);
        let (clicked, _) = frame(vec![button(false)]);
        assert_eq!(clicked, [Some(2), None]);
    }
}

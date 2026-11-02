//! Dockable floating toolbars ("Schwebeleisten"): a toolbar pill over a
//! viewport with a drag handle that docks at one of several places of
//! its pane: left or right (middle, vertical), top or bottom (centre,
//! horizontal), a corner, or compact in the pane's tab strip next to the
//! active tab ([`Dock::Header`]).
//!
//! Dragging the handle shows dashed target zones for the places the
//! toolbar allows; released in a zone it docks there, released elsewhere
//! it snaps back. A right click on the pill opens the "Position" menu.
//! The caller keeps the dock (per pane and toolbar) and applies
//! [`FloatBarOutput::moved`].
//!
//! A horizontal toolbar wider than its pane scrolls: its outer tools
//! collapse behind ‹ › chevrons. The toolbar takes the pointer only
//! inside its own bounds.

use egui::{
    Align, Align2, CursorIcon, Frame, Id, Layout, Margin, Pos2, Rect, Sense, Stroke, Ui, UiBuilder,
    Vec2, vec2,
};

use crate::i18n::tr;
use crate::icon::Icon;
use crate::theme::colors;
use crate::tokens::{Elevation, radius, space};
use crate::widgets::menu::{self, MenuItem};
use crate::widgets::toolbar::{self, ToolButton};

crate::messages! {
    enum Msg {
        Position: "Position", "Position";
        Left: "Left", "Links";
        Right: "Right", "Rechts";
        Top: "Top", "Oben";
        Bottom: "Bottom", "Unten";
        Header: "In the tab header", "Im Tab-Kopf";
        TopLeft: "Top left", "Oben links";
        TopRight: "Top right", "Oben rechts";
        BottomLeft: "Bottom left", "Unten links";
        BottomRight: "Bottom right", "Unten rechts";
        Handle: "Drag to dock elsewhere; right click: position", "Ziehen zum Andocken; Rechtsklick: Position";
        ScrollBack: "More tools", "Weitere Werkzeuge";
    }
}

/// Where a floating toolbar docks in its pane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dock {
    /// Left edge, middle; vertical.
    Left,
    /// Right edge, middle; vertical.
    Right,
    /// Top edge, centre; horizontal.
    Top,
    /// Bottom edge, centre; horizontal.
    Bottom,
    /// Compact in the tab strip, next to the pane's active tab.
    Header,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Dock {
    /// The edges and the tab header (the "Position" menu's order).
    pub const SIDES: [Dock; 5] = [
        Dock::Left,
        Dock::Right,
        Dock::Top,
        Dock::Bottom,
        Dock::Header,
    ];
    pub const CORNERS: [Dock; 4] = [
        Dock::TopLeft,
        Dock::TopRight,
        Dock::BottomLeft,
        Dock::BottomRight,
    ];
    pub const ALL: [Dock; 9] = [
        Dock::Left,
        Dock::Right,
        Dock::Top,
        Dock::Bottom,
        Dock::Header,
        Dock::TopLeft,
        Dock::TopRight,
        Dock::BottomLeft,
        Dock::BottomRight,
    ];

    /// Storage key.
    pub fn key(self) -> &'static str {
        match self {
            Dock::Left => "left",
            Dock::Right => "right",
            Dock::Top => "top",
            Dock::Bottom => "bottom",
            Dock::Header => "header",
            Dock::TopLeft => "top-left",
            Dock::TopRight => "top-right",
            Dock::BottomLeft => "bottom-left",
            Dock::BottomRight => "bottom-right",
        }
    }

    pub fn from_key(key: &str) -> Option<Dock> {
        Dock::ALL.into_iter().find(|d| d.key() == key)
    }

    pub fn label(self) -> &'static str {
        tr(match self {
            Dock::Left => Msg::Left,
            Dock::Right => Msg::Right,
            Dock::Top => Msg::Top,
            Dock::Bottom => Msg::Bottom,
            Dock::Header => Msg::Header,
            Dock::TopLeft => Msg::TopLeft,
            Dock::TopRight => Msg::TopRight,
            Dock::BottomLeft => Msg::BottomLeft,
            Dock::BottomRight => Msg::BottomRight,
        })
    }

    /// The tools stack vertically (left, right and the corners).
    pub fn vertical(self) -> bool {
        !matches!(self, Dock::Top | Dock::Bottom | Dock::Header)
    }

    /// Where the toolbar sits in the pane's body.
    pub fn align(self) -> Align2 {
        match self {
            Dock::Left => Align2::LEFT_CENTER,
            Dock::Right => Align2::RIGHT_CENTER,
            Dock::Top | Dock::Header => Align2::CENTER_TOP,
            Dock::Bottom => Align2::CENTER_BOTTOM,
            Dock::TopLeft => Align2::LEFT_TOP,
            Dock::TopRight => Align2::RIGHT_TOP,
            Dock::BottomLeft => Align2::LEFT_BOTTOM,
            Dock::BottomRight => Align2::RIGHT_BOTTOM,
        }
    }
}

/// Width of the tab header target zone when the slot is empty.
pub const HEADER_ZONE_W: f32 = 110.0;
/// Height of the compact chip in the tab header.
pub const HEADER_H: f32 = 26.0;

/// The dashed target zones of the places in `allowed`: where a toolbar
/// would dock in `body` (the pane below its tab strip), `header` the
/// slot next to the active tab.
pub fn zones(
    body: Rect,
    header: Option<Rect>,
    allowed: &[Dock],
    margin: Vec2,
) -> Vec<(Dock, Rect)> {
    let inner = body.shrink2(margin);
    let side = vec2(96.0, (body.height() * 0.45).clamp(80.0, 240.0));
    let edge = vec2((body.width() * 0.45).clamp(120.0, 360.0), 40.0);
    let corner = vec2(140.0, 110.0);
    allowed
        .iter()
        .filter_map(|&d| {
            let r = match d {
                Dock::Header => {
                    let h = header?;
                    Rect::from_min_size(h.min, vec2(h.width().max(HEADER_ZONE_W), h.height()))
                }
                Dock::Left | Dock::Right => d.align().align_size_within_rect(side, inner),
                Dock::Top | Dock::Bottom => d.align().align_size_within_rect(edge, inner),
                _ => d.align().align_size_within_rect(corner, inner),
            };
            Some((d, r))
        })
        .collect()
}

/// The zone under `pos` (a little slack around each); `None`: released
/// elsewhere, the toolbar snaps back.
pub fn zone_at(zones: &[(Dock, Rect)], pos: Pos2) -> Option<Dock> {
    zones
        .iter()
        .find(|(_, r)| r.expand(space::MD).contains(pos))
        .map(|&(d, _)| d)
}

/// The size a toolbar had last frame (to reserve its header slot).
pub fn last_size(
    ctx: &egui::Context,
    id_salt: impl std::hash::Hash + std::fmt::Debug,
) -> Option<Vec2> {
    ctx.data(|d| d.get_temp(bar_id(id_salt)))
}

fn bar_id(id_salt: impl std::hash::Hash + std::fmt::Debug) -> Id {
    Id::unique(("opencad_ui_floatbar", id_salt))
}

/// A dockable floating toolbar.
#[derive(Debug)]
#[must_use = "call `.show(ui, …)`"]
pub struct FloatBar<'a> {
    id: Id,
    dock: Dock,
    body: Rect,
    header: Option<Rect>,
    allowed: &'a [Dock],
    margin: Vec2,
    scroll: bool,
}

/// What a [`FloatBar`] did.
#[derive(Debug)]
pub struct FloatBarOutput<R> {
    pub inner: R,
    pub rect: Rect,
    /// Docked somewhere else (dragged into a zone or chosen in the menu).
    pub moved: Option<Dock>,
}

impl<'a> FloatBar<'a> {
    /// A toolbar docked at `dock` over `body` (the pane below its tab
    /// strip).
    pub fn new(id_salt: impl std::hash::Hash + std::fmt::Debug, dock: Dock, body: Rect) -> Self {
        FloatBar {
            id: bar_id(id_salt),
            dock,
            body,
            header: None,
            allowed: &Dock::SIDES,
            margin: Vec2::splat(space::XL),
            scroll: true,
        }
    }

    /// The slot in the tab strip next to the active tab (the toolbar's
    /// place with [`Dock::Header`]).
    pub fn header(mut self, slot: Option<Rect>) -> Self {
        self.header = slot;
        self
    }

    /// The places it may dock (default: [`Dock::SIDES`]).
    pub fn allowed(mut self, allowed: &'a [Dock]) -> Self {
        self.allowed = allowed;
        self
    }

    /// Distance from the pane's edges.
    pub fn margin(mut self, margin: Vec2) -> Self {
        self.margin = margin;
        self
    }

    /// Horizontal overflow scrolls behind ‹ › (default); off for a
    /// toolbar that collapses its tools itself.
    pub fn scroll(mut self, on: bool) -> Self {
        self.scroll = on;
        self
    }

    /// Shows the toolbar; `add` gets the dock to lay out its tools for
    /// (vertical, horizontal, compact header) and `menu_extra` may add
    /// items to the "Position" menu.
    pub fn show<R>(
        self,
        ui: &mut Ui,
        menu_extra: impl FnOnce(&mut Ui),
        add: impl FnOnce(&mut Ui, Dock) -> R,
    ) -> FloatBarOutput<R> {
        let FloatBar {
            id,
            dock,
            body,
            header,
            allowed,
            margin,
            scroll,
        } = self;
        let c = colors(ui);
        let in_header = dock == Dock::Header;
        let size: Option<Vec2> = ui.data(|d| d.get_temp(id));
        let inner_area = if in_header {
            header.unwrap_or(Rect::from_min_size(body.min, vec2(HEADER_ZONE_W, HEADER_H)))
        } else {
            body.shrink2(margin)
        };
        let rect = if in_header {
            Rect::from_min_size(
                inner_area.left_center() - vec2(0.0, HEADER_H / 2.0),
                size.unwrap_or(Vec2::ZERO),
            )
        } else {
            let r = dock
                .align()
                .align_size_within_rect(size.unwrap_or(Vec2::ZERO), inner_area);
            // Larger than the pane: start at the edge, cut at the other.
            r.translate(vec2(
                (inner_area.left() - r.left()).max(0.0),
                (inner_area.top() - r.top()).max(0.0),
            ))
        };
        // The background swallows the pointer inside the pill (also
        // between the tools); registered before the tools, which stay on
        // top. Right click anywhere on it opens the menu.
        let drag_id = id.with("drag");
        let dragging = ui.data(|d| d.get_temp::<bool>(drag_id)).unwrap_or(false);

        let mut child = ui.new_child(
            UiBuilder::new()
                .id_salt(id)
                .max_rect(Rect::from_min_size(
                    rect.min,
                    inner_area.size().max(vec2(40.0, HEADER_H)),
                ))
                .layout(Layout::top_down(Align::Min)),
        );
        let clip = if in_header {
            inner_area.expand(2.0)
        } else {
            body
        };
        child.set_clip_rect(clip);
        let bg = child.interact(
            if size.is_some() { rect } else { Rect::NOTHING },
            id.with("background"),
            Sense::click_and_drag(),
        );
        if dragging {
            child.multiply_opacity(0.45);
        }
        let frame = if in_header {
            Frame::new()
                .fill(c.surface_raised)
                .stroke(Stroke::new(1.0, c.divider))
                .corner_radius(radius::TILE)
                .inner_margin(Margin::same(1))
        } else {
            Frame::new()
                .fill(c.glass)
                .stroke(Stroke::new(1.0, c.ring))
                .corner_radius(radius::OVERLAY)
                .inner_margin(Margin::same(space::XS as i8))
                .shadow(Elevation::Popover.shadow(c.dark))
        };
        let vertical = dock.vertical();
        let mut handle = None;
        let shown = frame.show(&mut child, |ui| {
            let body_ui = |ui: &mut Ui| {
                handle = Some(grip(ui, id, vertical, in_header));
                if in_header {
                    ui.spacing_mut().item_spacing.x = 0.0;
                }
                add(ui, dock)
            };
            if vertical {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 2.0;
                    body_ui(ui)
                })
                .inner
            } else if scroll && !in_header {
                // The tools' own width last frame against the room.
                let room = inner_area.width() - 2.0 * space::XS;
                scrolled_row(ui, id, room, body_ui)
            } else {
                toolbar::toolbar(ui, body_ui)
            }
        });
        let r = shown.response.rect;
        if size != Some(r.size()) {
            ui.data_mut(|d| d.insert_temp(id, r.size()));
            ui.ctx().request_repaint();
        }

        // Dragging the handle: the target zones and a ghost at the
        // pointer; released in a zone, the toolbar docks there.
        let mut moved = None;
        let zones = zones(body, header, allowed, margin);
        if let Some(h) = &handle {
            if h.drag_started() {
                let grab = h.interact_pointer_pos().unwrap_or(r.min) - r.min;
                ui.data_mut(|d| {
                    d.insert_temp(drag_id, true);
                    d.insert_temp(id.with("grab"), grab);
                });
            }
            let pos = ui.ctx().pointer_interact_pos();
            if h.dragged()
                && let Some(pos) = pos
            {
                let grab: Vec2 = ui.data(|d| d.get_temp(id.with("grab"))).unwrap_or_default();
                let hovered = zone_at(&zones, pos);
                paint_zones(
                    ui,
                    &zones,
                    hovered,
                    Rect::from_min_size(pos - grab, r.size()),
                    clip_of(body, header),
                );
                ui.ctx().set_cursor_icon(CursorIcon::Grabbing);
            } else if h.hovered() {
                ui.ctx().set_cursor_icon(CursorIcon::Grab);
            }
            if h.drag_stopped() {
                ui.data_mut(|d| d.remove::<bool>(drag_id));
                moved = pos.and_then(|p| zone_at(&zones, p)).filter(|&d| d != dock);
            }
        }

        // The "Position" menu: on a right click anywhere on the pill.
        let secondary = ui.input(|i| i.pointer.secondary_clicked())
            && ui
                .ctx()
                .pointer_interact_pos()
                .is_some_and(|p| r.contains(p) && clip.contains(p));
        menu::context_menu_forced(ui, &bg, secondary, |ui| {
            menu::section_header(ui, tr(Msg::Position), None);
            for &d in allowed {
                if d == Dock::Header && header.is_none() {
                    continue;
                }
                if MenuItem::new(d.label())
                    .checked(d == dock)
                    .show(ui)
                    .clicked()
                {
                    if d != dock {
                        moved = Some(d);
                    }
                    ui.close();
                }
            }
            menu_extra(ui);
        });
        FloatBarOutput {
            inner: shown.inner,
            rect: r,
            moved,
        }
    }
}

fn clip_of(body: Rect, header: Option<Rect>) -> Rect {
    match header {
        Some(h) => body.union(h),
        None => body,
    }
    .expand(2.0)
}

/// The drag handle: three dots across the top (vertical toolbar) or
/// down the left side.
fn grip(ui: &mut Ui, id: Id, vertical: bool, compact: bool) -> egui::Response {
    let c = colors(ui);
    let s = if vertical {
        vec2(ui.min_rect().width().max(28.0), 10.0)
    } else if compact {
        vec2(10.0, HEADER_H - 2.0)
    } else {
        vec2(12.0, 28.0)
    };
    let (rect, _) = ui.allocate_exact_size(s, Sense::hover());
    note_item(ui, rect);
    let resp = ui
        .interact(rect, id.with("handle"), Sense::drag())
        .on_hover_text(tr(Msg::Handle));
    let tone = if resp.hovered() || resp.dragged() {
        c.text_secondary
    } else {
        c.text_tertiary
    };
    let step = if vertical {
        vec2(4.0, 0.0)
    } else {
        vec2(0.0, 4.0)
    };
    for k in -1..=1 {
        ui.painter()
            .circle_filled(rect.center() + step * k as f32, 1.2, tone);
    }
    resp
}

/// Width of a ‹ › chevron button ([`ToolButton::icon`]: its height
/// plus 2).
const CHEVRON_W: f32 = 30.0;
/// Spacing between the items of a toolbar row.
const ROW_GAP: f32 = 2.0;
/// Accumulated wheel distance that scrolls the row by one tool.
const WHEEL_STEP: f32 = 40.0;

/// The tools of the scrolled row being laid out: its layer, its band on
/// screen and the horizontal extents of its tools (screen x).
#[derive(Debug, Clone)]
struct Collect {
    layer: egui::LayerId,
    rows: egui::Rangef,
    items: Vec<(f32, f32)>,
}

fn collect_id() -> Id {
    Id::unique("opencad_ui_floatbar_items")
}

/// Notes a tool of a toolbar row at `rect` (called by [`ToolButton`]
/// and the grip): in a scrolled row the visible window starts and ends
/// at these tools instead of cutting one. Outside a scrolled row (and
/// for widgets in popups) it does nothing.
pub fn note_item(ui: &Ui, rect: Rect) {
    let layer = ui.layer_id();
    ui.ctx().data_mut(|d| {
        if let Some(c) = d.get_temp_mut_or_insert_with(collect_id(), || None::<Collect>)
            && c.layer == layer
            && c.rows.intersects(rect.y_range())
        {
            c.items.push((rect.left(), rect.right()));
        }
    });
}

/// The visible window of a scrolled row: `items` are the tools'
/// extents in content coordinates (sorted), `offset` the wanted scroll
/// offset, `room` the most width there is. The window starts at a tool
/// (the first at or after `offset`, earlier if the rest fits) and ends
/// with the last tool that fits whole, so no tool is cut and the gaps at
/// both chevrons are the same. Returns `(offset, width)`.
pub fn window(items: &[(f32, f32)], offset: f32, room: f32) -> (f32, f32) {
    let Some(&(last_start, last)) = items.last() else {
        return (offset.max(0.0), room);
    };
    // The first tool not left of the offset.
    let mut start = items
        .iter()
        .map(|i| i.0)
        .find(|&s| s >= offset - 0.5)
        .unwrap_or(last_start);
    // Near the end: back as far as the rest still fits.
    if let Some(s) = items.iter().map(|i| i.0).find(|&s| last - s <= room + 0.5) {
        start = start.min(s);
    }
    let width = items
        .iter()
        .filter(|i| i.0 >= start - 0.5 && i.1 <= start + room + 0.5)
        .map(|i| i.1 - start)
        .fold(0.0_f32, f32::max);
    // A single tool wider than the room: cut it.
    (start, if width > 0.0 { width } else { room })
}

/// The offset one page on (`forward`) or back from the window
/// `(offset, width)`: forward the first tool not shown whole, back so
/// that the tool before the window becomes the last one shown.
pub fn page(items: &[(f32, f32)], (offset, width): (f32, f32), room: f32, forward: bool) -> f32 {
    if forward {
        return items
            .iter()
            .find(|i| i.1 > offset + width + 0.5)
            .map_or(offset, |i| i.0);
    }
    let Some(end) = items.iter().rev().find(|i| i.0 < offset - 0.5).map(|i| i.1) else {
        return 0.0;
    };
    items
        .iter()
        .find(|i| end - i.0 <= room + 0.5)
        .map_or(0.0, |i| i.0)
}

/// The offset `steps` tools on (positive) or back from `offset`.
fn step(items: &[(f32, f32)], mut offset: f32, steps: i32) -> f32 {
    for _ in 0..steps.unsigned_abs() {
        offset = if steps > 0 {
            items
                .iter()
                .find(|i| i.0 > offset + 0.5)
                .map_or(offset, |i| i.0)
        } else {
            items
                .iter()
                .rev()
                .find(|i| i.0 < offset - 0.5)
                .map_or(0.0, |i| i.0)
        };
    }
    offset
}

/// A horizontal row that scrolls behind ‹ › when its tools are wider
/// than `room`. The window shows whole tools only (see [`window`]), so
/// the row is as wide as what it shows and the toolbar stays centred.
fn scrolled_row<R>(ui: &mut Ui, id: Id, room: f32, add: impl FnOnce(&mut Ui) -> R) -> R {
    let natural_id = id.with("natural");
    let offset_id = id.with("offset");
    let items_id = id.with("items");
    let wheel_id = id.with("wheel");
    let height_id = id.with("height");
    let natural: f32 = ui.data(|d| d.get_temp(natural_id)).unwrap_or(0.0);
    let chevrons = 2.0 * (CHEVRON_W + ROW_GAP);
    if natural <= room + 0.5 || room < 2.0 * chevrons {
        let r = ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = ROW_GAP;
            add(ui)
        });
        let w = r.response.rect.width();
        if (w - natural).abs() > 0.5 {
            ui.data_mut(|d| d.insert_temp(natural_id, w));
        }
        return r.inner;
    }
    let items: Vec<(f32, f32)> = ui.data(|d| d.get_temp(items_id)).unwrap_or_default();
    let max_view = room - chevrons;
    let wanted: f32 = ui.data(|d| d.get_temp(offset_id)).unwrap_or(0.0);
    let (mut offset, view) = window(&items, wanted, max_view);
    let shown = (offset, view);
    // The arrows as tall as the tools (last frame), so they share the
    // row's centre line with them.
    let height: Option<f32> = ui.data(|d| d.get_temp(height_id));
    let arrow = |icon| {
        let b = ToolButton::icon(icon, tr(Msg::ScrollBack));
        match height {
            Some(h) => b.height(h),
            None => b,
        }
    };
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = ROW_GAP;
        let back = ui.add_enabled(offset > 0.5, arrow(Icon::ChevronLeft));
        if back.clicked() {
            offset = page(&items, shown, max_view, false);
        }
        let collect = Collect {
            layer: ui.layer_id(),
            rows: ui.max_rect().y_range(),
            items: Vec::new(),
        };
        ui.ctx()
            .data_mut(|d| d.insert_temp(collect_id(), Some(collect)));
        let out = egui::ScrollArea::horizontal()
            .id_salt(id.with("scroll"))
            .max_width(view)
            .min_scrolled_width(view)
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysHidden)
            .scroll_source(egui::scroll_area::ScrollSource::NONE)
            .horizontal_scroll_offset(shown.0)
            .show(ui, |ui| {
                // A row of the tools' height: `horizontal` would centre
                // them on the taller default row of the scroll area.
                let h = height.unwrap_or(ui.spacing().interact_size.y);
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), h),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.spacing_mut().item_spacing.x = ROW_GAP;
                        add(ui)
                    },
                )
                .inner
            });
        let collected = ui
            .ctx()
            .data_mut(|d| d.remove_temp::<Option<Collect>>(collect_id()))
            .flatten();
        // The tools in content coordinates, for the next frame.
        let left = out.inner_rect.left() - out.state.clamped_offset().x;
        let mut found: Vec<(f32, f32)> = collected
            .map(|c| c.items)
            .unwrap_or_default()
            .into_iter()
            .map(|(a, b)| (a - left, b - left))
            .collect();
        found.sort_by(|a, b| a.0.total_cmp(&b.0));
        let content = out.content_size.x;
        let fwd = ui.add_enabled(shown.0 + view < content - 0.5, arrow(Icon::ChevronRight));
        if fwd.clicked() {
            offset = page(&items, shown, max_view, true);
        }
        // The wheel (or a trackpad swipe) over the row: one tool per
        // step.
        if ui.rect_contains_pointer(back.rect.union(fwd.rect)) {
            let delta = ui.input(|i| i.smooth_scroll_delta);
            let delta = if delta.x.abs() > delta.y.abs() {
                delta.x
            } else {
                delta.y
            };
            if delta != 0.0 {
                let mut acc: f32 = ui.data(|d| d.get_temp(wheel_id)).unwrap_or(0.0) + delta;
                let steps = (acc / WHEEL_STEP).trunc();
                acc -= steps * WHEEL_STEP;
                offset = step(&items, offset, -(steps as i32));
                ui.data_mut(|d| d.insert_temp(wheel_id, acc));
            }
        }
        if (offset - wanted).abs() > 0.5 || found != items {
            ui.ctx().request_repaint();
        }
        ui.data_mut(|d| {
            d.insert_temp(offset_id, offset);
            d.insert_temp(items_id, found);
            d.insert_temp(height_id, out.content_size.y);
            if (content - natural).abs() > 0.5 {
                d.insert_temp(natural_id, content);
            }
        });
        out.inner
    })
    .inner
}

/// The dashed target zones (the hovered one filled) and a ghost of the
/// toolbar at the pointer, over everything in the pane.
fn paint_zones(ui: &Ui, zones: &[(Dock, Rect)], hovered: Option<Dock>, ghost: Rect, clip: Rect) {
    let c = colors(ui);
    let p = ui
        .ctx()
        .layer_painter(egui::LayerId::new(
            egui::Order::Foreground,
            ui.scope_id().with("floatbar_zones"),
        ))
        .with_clip_rect(clip);
    for &(d, r) in zones {
        let on = hovered == Some(d);
        let fill = if on {
            c.accent.gamma_multiply(0.16)
        } else {
            c.accent.gamma_multiply(0.05)
        };
        p.rect_filled(r, radius::CARD, fill);
        dashed(
            &p,
            r.shrink(1.0),
            Stroke::new(
                if on { 2.0 } else { 1.5 },
                c.accent.gamma_multiply(if on { 1.0 } else { 0.6 }),
            ),
        );
    }
    p.rect_filled(ghost, radius::OVERLAY, c.glass.gamma_multiply(0.9));
    p.rect_stroke(
        ghost,
        radius::OVERLAY,
        Stroke::new(1.0, c.accent),
        egui::StrokeKind::Inside,
    );
}

fn dashed(p: &egui::Painter, r: Rect, stroke: Stroke) {
    let pts = [
        r.left_top(),
        r.right_top(),
        r.right_bottom(),
        r.left_bottom(),
        r.left_top(),
    ];
    p.extend(egui::Shape::dashed_line(&pts, stroke, 6.0, 4.0));
}

/// A point in the middle of a zone (tests, scripts).
pub fn zone_center(zones: &[(Dock, Rect)], dock: Dock) -> Option<Pos2> {
    zones
        .iter()
        .find(|(d, _)| *d == dock)
        .map(|(_, r)| r.center())
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::pos2;

    fn body() -> Rect {
        Rect::from_min_size(pos2(0.0, 36.0), vec2(800.0, 500.0))
    }

    #[test]
    fn docks_round_trip_through_their_keys() {
        for d in Dock::ALL {
            assert_eq!(Dock::from_key(d.key()), Some(d));
        }
        assert_eq!(Dock::from_key("middle"), None);
        assert!(Dock::Left.vertical() && !Dock::Bottom.vertical() && !Dock::Header.vertical());
    }

    #[test]
    fn zones_are_hit_where_the_toolbar_would_dock() {
        let header = Rect::from_min_size(pos2(140.0, 41.0), vec2(0.0, HEADER_H));
        let z = zones(body(), Some(header), &Dock::SIDES, Vec2::splat(12.0));
        assert_eq!(z.len(), 5);
        let b = body();
        assert_eq!(
            zone_at(&z, pos2(b.left() + 30.0, b.center().y)),
            Some(Dock::Left)
        );
        assert_eq!(
            zone_at(&z, pos2(b.right() - 30.0, b.center().y)),
            Some(Dock::Right)
        );
        assert_eq!(
            zone_at(&z, pos2(b.center().x, b.top() + 25.0)),
            Some(Dock::Top)
        );
        assert_eq!(
            zone_at(&z, pos2(b.center().x, b.bottom() - 25.0)),
            Some(Dock::Bottom)
        );
        // The empty header slot still offers a zone of its own width.
        assert_eq!(zone_at(&z, pos2(200.0, 50.0)), Some(Dock::Header));
        // The middle of the view: no zone, the toolbar snaps back.
        assert_eq!(zone_at(&z, b.center()), None);
        // Without a header slot no header zone; corners only if allowed.
        let z = zones(b, None, &Dock::CORNERS, Vec2::splat(12.0));
        assert_eq!(z.len(), 4);
        assert_eq!(
            zone_at(&z, b.right_bottom() - vec2(30.0, 30.0)),
            Some(Dock::BottomRight)
        );
        assert_eq!(zone_at(&z, pos2(b.center().x, b.top() + 25.0)), None);
        assert_eq!(
            zone_center(&z, Dock::TopLeft).map(|p| b.contains(p)),
            Some(true)
        );
    }

    /// Ten tools of 30 with gaps of 2 (and a separator of 9 after the
    /// fourth), in content coordinates.
    fn tools() -> Vec<(f32, f32)> {
        let mut x = 0.0;
        let mut out = Vec::new();
        for i in 0..10 {
            out.push((x, x + 30.0));
            x += 32.0;
            if i == 3 {
                x += 11.0;
            }
        }
        out
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn the_window_shows_whole_tools_only() {
        let t = tools();
        // 100 of room: three tools (94), not a third of the fourth.
        assert_eq!(window(&t, 0.0, 100.0), (0.0, 94.0));
        // An offset inside a tool starts at the next one.
        assert_eq!(window(&t, 10.0, 100.0), (32.0, 94.0));
        // Near the end it backs up so that the room is used.
        let (o, w) = window(&t, 300.0, 100.0);
        assert_eq!(o + w, t[9].1);
        assert!(w <= 100.0 && w > 60.0);
        // No tools known yet: the plain room.
        assert_eq!(window(&[], 5.0, 100.0), (5.0, 100.0));
        // A tool wider than the room is cut.
        assert_eq!(window(&[(0.0, 300.0)], 0.0, 100.0), (0.0, 100.0));
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn paging_moves_by_whole_tools_and_back_again() {
        let t = tools();
        let room = 100.0;
        let first = window(&t, 0.0, room);
        let next = page(&t, first, room, true);
        // The first tool not shown whole comes first.
        assert_eq!(next, t[3].0);
        let second = window(&t, next, room);
        // Back: the tool before the window is the last one shown.
        let back = page(&t, second, room, false);
        let w = window(&t, back, room);
        assert_eq!(w.0 + w.1, t[2].1);
        assert_eq!(page(&t, first, room, false), 0.0);
        // One tool per wheel step.
        assert_eq!(step(&t, 0.0, 2), t[2].0);
        assert_eq!(step(&t, t[2].0, -1), t[1].0);
        assert_eq!(step(&t, 0.0, -1), 0.0);
    }

    /// A pane narrower than the toolbar: the row ends at a tool, the
    /// gaps next to both chevrons are equal and the toolbar is centred.
    #[test]
    fn a_scrolled_toolbar_is_centred_and_cut_between_tools() {
        use egui::RawInput;
        let ctx = egui::Context::default();
        crate::install(&ctx, |_| {});
        let screen = Rect::from_min_size(pos2(0.0, 0.0), vec2(1000.0, 400.0));
        let body = Rect::from_min_size(pos2(100.0, 36.0), vec2(330.0, 300.0));
        let mut out = None;
        let mut buttons = Vec::new();
        for _ in 0..6 {
            buttons.clear();
            let mut o = ctx.run_ui(
                RawInput {
                    screen_rect: Some(screen),
                    ..Default::default()
                },
                |ui| {
                    let r = FloatBar::new("test_bar", Dock::Top, body).show(
                        ui,
                        |_| {},
                        |ui, _| {
                            for i in 0..12 {
                                let b = ui.add(ToolButton::icon(Icon::ZoomAll, "x"));
                                buttons.push(b.rect);
                                if i % 4 == 3 {
                                    toolbar::separator(ui);
                                }
                            }
                        },
                    );
                    out = Some(r.rect);
                },
            );
            o.textures_delta.clear();
        }
        let bar = out.unwrap();
        let inner = body.shrink2(Vec2::splat(space::XL));
        assert!(bar.width() <= inner.width() + 0.5, "{bar:?}");
        // Centred in the pane.
        assert!(
            (bar.center().x - inner.center().x).abs() < 1.0,
            "{bar:?} in {inner:?}"
        );
        // The ‹ chevron, the grip and the tools: the last tool shown
        // whole ends where the › chevron begins (less the spacing), at
        // the same distance as the first tool after ‹.
        let chevron_l = bar.left() + space::XS + CHEVRON_W;
        let chevron_r = bar.right() - space::XS - CHEVRON_W;
        let shown: Vec<&Rect> = buttons
            .iter()
            .filter(|r| r.right() > chevron_l + 0.5 && r.left() < chevron_r - 0.5)
            .collect();
        assert!(shown.len() >= 3, "{shown:?}");
        for r in &shown {
            assert!(
                r.left() >= chevron_l - 0.5 && r.right() <= chevron_r + 0.5,
                "{r:?} {bar:?} {chevron_l} {chevron_r} {shown:?}"
            );
        }
        let last = shown.last().unwrap();
        assert!(
            (chevron_r - last.right() - ROW_GAP).abs() < 1.0,
            "{last:?} {bar:?} {chevron_r}"
        );
    }
}

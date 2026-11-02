//! The visual pieces of docking: the pane card with its tab strip and
//! buttons, the drop-target cross (and its hit test) and the drop
//! preview. The app's viewports (tabs, splits, floating views) are built
//! on them.

use egui::{Frame, Margin, Pos2, Rect, Response, Sense, Stroke, StrokeKind, Ui, Vec2, pos2, vec2};

use crate::icon::{self, Icon};
use crate::theme::colors;
use crate::tokens::{Elevation, radius, size, space};

/// Where a dragged pane would dock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DropZone {
    Left,
    Right,
    Top,
    Bottom,
    /// As a tab of the target pane.
    Center,
}

impl DropZone {
    pub const ALL: [DropZone; 5] = [
        DropZone::Top,
        DropZone::Left,
        DropZone::Center,
        DropZone::Right,
        DropZone::Bottom,
    ];

    fn icon(self) -> Icon {
        match self {
            DropZone::Left => Icon::DockLeft,
            DropZone::Right => Icon::DockRight,
            DropZone::Top => Icon::DockTop,
            DropZone::Bottom => Icon::DockBottom,
            DropZone::Center => Icon::DockCenter,
        }
    }

    /// The cell of the zone in the 3 × 3 cross.
    fn cell(self) -> (i32, i32) {
        match self {
            DropZone::Top => (1, 0),
            DropZone::Left => (0, 1),
            DropZone::Center => (1, 1),
            DropZone::Right => (2, 1),
            DropZone::Bottom => (1, 2),
        }
    }

    /// The part of `pane` the zone would take.
    pub fn preview(self, pane: Rect) -> Rect {
        let half = |r: Rect| r;
        match self {
            DropZone::Left => Rect::from_min_max(pane.min, pos2(pane.center().x, pane.bottom())),
            DropZone::Right => Rect::from_min_max(pos2(pane.center().x, pane.top()), pane.max),
            DropZone::Top => Rect::from_min_max(pane.min, pos2(pane.right(), pane.center().y)),
            DropZone::Bottom => Rect::from_min_max(pos2(pane.left(), pane.center().y), pane.max),
            DropZone::Center => half(pane),
        }
    }
}

/// A pane: a card with a ring and a soft shadow, the tab strip on top
/// (`tabs`, drawn on the sunken strip) and the content below.
pub fn pane<R>(
    ui: &mut Ui,
    size_: Vec2,
    tabs: impl FnOnce(&mut Ui),
    body: impl FnOnce(&mut Ui) -> R,
) -> R {
    let c = colors(ui);
    Frame::new()
        .fill(c.surface_raised)
        .stroke(Stroke::new(1.0, c.ring))
        .corner_radius(radius::CARD)
        .shadow(Elevation::Pane.shadow(c.dark))
        .show(ui, |ui| {
            // The 1 pt ring sits outside the content on both sides.
            ui.set_width(size_.x - 2.0);
            ui.set_height(size_.y);
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            let (strip, _) = ui.allocate_exact_size(
                vec2(ui.available_width(), size::TAB_STRIP_H),
                Sense::hover(),
            );
            let corners = egui::CornerRadius {
                nw: radius::CARD,
                ne: radius::CARD,
                sw: 0,
                se: 0,
            };
            ui.painter().rect_filled(strip, corners, c.surface_sunken);
            ui.painter().hline(
                strip.x_range(),
                strip.bottom() - 0.5,
                Stroke::new(1.0, c.divider),
            );
            let mut tu = ui.new_child(egui::UiBuilder::new().max_rect(strip));
            tabs(&mut tu);
            Frame::new()
                .inner_margin(Margin::same(space::XL as i8))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.set_height(ui.available_height());
                    body(ui)
                })
                .inner
        })
        .inner
}

/// The size of a cell of the drop-target cross and the gap between cells.
const CROSS_CELL: f32 = 40.0;
const CROSS_GAP: f32 = 4.0;

/// The rect of `zone`'s cell in the drop-target cross centred on `center`.
pub fn cross_cell(center: Pos2, zone: DropZone) -> Rect {
    let (cx, cy) = zone.cell();
    Rect::from_center_size(
        center
            + vec2(
                (cx - 1) as f32 * (CROSS_CELL + CROSS_GAP),
                (cy - 1) as f32 * (CROSS_CELL + CROSS_GAP),
            ),
        Vec2::splat(CROSS_CELL),
    )
}

/// The zone of the drop-target cross centred on `center` under `pos`.
pub fn cross_zone(center: Pos2, pos: Pos2) -> Option<DropZone> {
    DropZone::ALL
        .into_iter()
        .find(|&z| cross_cell(center, z).contains(pos))
}

/// The drop-target cross centred on `center`; returns the zone under the
/// pointer.
pub fn drop_cross(ui: &Ui, center: Pos2, hovered: Option<DropZone>) -> Option<DropZone> {
    let c = colors(ui);
    let p = ui
        .ctx()
        .layer_painter(egui::LayerId::new(
            egui::Order::Foreground,
            ui.scope_id().with("drop_cross"),
        ))
        .with_clip_rect(ui.clip_rect());
    let pointer = ui.ctx().pointer_hover_pos();
    let mut under = None;
    for z in DropZone::ALL {
        let r = cross_cell(center, z);
        let hot = hovered == Some(z) || pointer.is_some_and(|q| r.contains(q));
        if pointer.is_some_and(|q| r.contains(q)) {
            under = Some(z);
        }
        p.add(Elevation::Popover.shadow(c.dark).as_shape(r, radius::FIELD));
        if hot {
            p.rect_filled(r, radius::FIELD, c.accent_fill);
            icon::paint(
                &p,
                Rect::from_center_size(r.center(), Vec2::splat(18.0)),
                z.icon(),
                c.on_accent,
            );
        } else {
            p.rect(
                r,
                radius::FIELD,
                c.glass,
                Stroke::new(1.0, c.accent_tint),
                StrokeKind::Inside,
            );
            p.rect_stroke(
                r,
                radius::FIELD,
                Stroke::new(1.0, crate::tokens::mix(c.accent, c.surface_raised, 0.6)),
                StrokeKind::Inside,
            );
            icon::paint(
                &p,
                Rect::from_center_size(r.center(), Vec2::splat(18.0)),
                z.icon(),
                c.accent,
            );
        }
    }
    under
}

/// The preview of where a pane would land: a tinted, dashed rectangle.
pub fn drop_preview(ui: &Ui, rect: Rect) {
    let c = colors(ui);
    let p = ui
        .ctx()
        .layer_painter(egui::LayerId::new(
            egui::Order::Foreground,
            ui.scope_id().with("drop_preview"),
        ))
        .with_clip_rect(ui.clip_rect());
    p.rect_filled(rect, radius::CARD, c.accent.gamma_multiply(0.08));
    let r = rect.shrink(1.0);
    let pts = [
        r.left_top(),
        r.right_top(),
        r.right_bottom(),
        r.left_bottom(),
        r.left_top(),
    ];
    p.extend(egui::Shape::dashed_line(
        &pts,
        Stroke::new(2.0, c.accent),
        6.0,
        4.0,
    ));
}

/// A pane header button (undock, maximise).
pub fn pane_button(ui: &mut Ui, icon_: Icon, tip: &str) -> Response {
    ui.add(
        super::button::IconButton::new(icon_, tip)
            .size(Vec2::splat(26.0))
            .icon_size(14.0)
            .color(colors(ui).text_secondary),
    )
}

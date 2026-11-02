//! Surfaces and layout: cards, titled sections, collapsible sections,
//! inset blocks, form rows (a label column and the control), dividers and
//! section titles.

use egui::collapsing_header::CollapsingState;
use egui::{
    Align, CornerRadius, Frame, Id, Layout, Margin, Rect, Response, Role, Sense, Stroke, Ui, Vec2,
    pos2, vec2,
};

use crate::icon::{self, Icon};
use crate::state;
use crate::text::{self, Type};
use crate::theme::colors;
use crate::tokens::{Elevation, radius, size, space};

/// A card: a raised, rounded surface on a panel.
#[derive(Debug, Clone, Copy)]
#[must_use = "call `.show(ui, …)`"]
pub struct Card {
    padding: Margin,
    corners: CornerRadius,
    elevation: Elevation,
    gap: f32,
}

impl Default for Card {
    fn default() -> Self {
        Card {
            padding: Margin::symmetric(space::XXL as i8, space::XL as i8),
            corners: CornerRadius::same(radius::CARD),
            elevation: Elevation::Flat,
            gap: space::LG,
        }
    }
}

impl Card {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn padding(mut self, padding: impl Into<Margin>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Corners (a header attached to tabs has square top corners).
    pub fn corners(mut self, corners: impl Into<CornerRadius>) -> Self {
        self.corners = corners.into();
        self
    }

    pub fn elevation(mut self, e: Elevation) -> Self {
        self.elevation = e;
        self
    }

    /// Vertical gap between the children.
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn show<R>(self, ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
        let c = colors(ui);
        let mut frame = Frame::new()
            .fill(c.surface_raised)
            .corner_radius(self.corners)
            .inner_margin(self.padding)
            .shadow(self.elevation.shadow(c.dark));
        if self.elevation.ring() {
            frame = frame.stroke(Stroke::new(1.0, c.ring));
        }
        frame
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = self.gap;
                    add(ui)
                })
                .inner
            })
            .inner
    }
}

/// The upper-case title of a section.
pub fn section_title(ui: &mut Ui, title: &str) -> Response {
    let c = colors(ui);
    text::label(ui, title, Type::Overline, c.text_secondary)
}

/// A card with a title and something on the right of the title (a chip,
/// a note, a switch).
pub fn section<R>(
    ui: &mut Ui,
    title: &str,
    trailing: impl FnOnce(&mut Ui),
    body: impl FnOnce(&mut Ui) -> R,
) -> R {
    Card::new().show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.set_min_height(20.0);
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                trailing(ui);
                ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                    section_title(ui, title);
                });
            });
        });
        body(ui)
    })
}

/// A card whose body folds away when its title is clicked (or Enter /
/// Space pressed on it); the state is remembered per `id`, and the body
/// slides open and closed.
pub fn collapsible<R>(
    ui: &mut Ui,
    id: impl std::hash::Hash + std::fmt::Debug,
    title: &str,
    default_open: bool,
    body: impl FnOnce(&mut Ui) -> R,
) -> Option<R> {
    let id = Id::unique(("opencad_ui_collapsible", id));
    let mut st = CollapsingState::load_with_default_open(ui.ctx(), id, default_open);
    let out = Card::new().show(ui, |ui| {
        let c = colors(ui);
        let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 20.0), Sense::click());
        state::a11y_selected(&resp, Role::Button, st.is_open(), title);
        let s = state::state(ui, &resp);
        if resp.clicked() {
            st.toggle(ui);
        }
        let open = st.openness(ui.ctx());
        let p = ui.painter();
        let g = text::truncated(
            p,
            title,
            Type::Overline,
            c.text_secondary,
            rect.width() - 24.0,
        );
        text::paint_in(p, rect, g, Align::Min, c.text_secondary);
        // The chevron turns from right to down.
        let center = pos2(rect.right() - 7.0, rect.center().y);
        let rot = egui::emath::Rot2::from_angle(std::f32::consts::FRAC_PI_2 * open);
        let pts: Vec<egui::Pos2> = [vec2(-2.0, -4.0), vec2(2.0, 0.0), vec2(-2.0, 4.0)]
            .iter()
            .map(|v| center + rot * *v)
            .collect();
        let fg = if s.hovered { c.text } else { c.text_secondary };
        p.add(egui::Shape::line(pts, Stroke::new(1.5, fg)));
        if s.focused {
            state::focus_ring(ui, rect.expand(2.0), radius::ROW, c);
        }
        st.show_body_unindented(ui, |ui| {
            ui.spacing_mut().item_spacing.y = space::LG;
            body(ui)
        })
    });
    st.store(ui.ctx());
    out.map(|r| r.inner)
}

/// An inset block inside a card (heights, previews).
pub fn inset<R>(ui: &mut Ui, body: impl FnOnce(&mut Ui) -> R) -> R {
    let c = colors(ui);
    Frame::new()
        .fill(c.surface_sunken)
        .corner_radius(radius::TILE)
        .inner_margin(Margin::same(space::LG as i8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = space::SM;
            body(ui)
        })
        .inner
}

/// A thin divider across the available width.
pub fn divider(ui: &mut Ui) {
    let c = colors(ui);
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 1.0), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, c.divider);
}

/// The label column width of a form row in a card `width` wide.
pub fn label_width(width: f32) -> f32 {
    (width * 0.4).clamp(56.0, size::LABEL_W)
}

/// A form row: the label in a column (truncated, full text on hover) and
/// the control beside it.
pub fn form_row<R>(ui: &mut Ui, label: &str, add: impl FnOnce(&mut Ui) -> R) -> R {
    let c = colors(ui);
    let lw = label_width(ui.available_width());
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = space::LG;
        let (rect, resp) = ui.allocate_exact_size(vec2(lw, size::FIELD_H), Sense::hover());
        let fg = if ui.is_enabled() {
            c.text_secondary
        } else {
            c.text_disabled
        };
        let g = text::truncated(ui.painter(), label, Type::Body, fg, lw);
        let elided = g.elided;
        text::paint_in(ui.painter(), rect, g, Align::Min, fg);
        if elided {
            resp.on_hover_text(label);
        }
        ui.allocate_ui_with_layout(
            vec2(ui.available_width(), size::FIELD_H),
            Layout::left_to_right(Align::Center),
            |ui| {
                ui.spacing_mut().item_spacing = vec2(space::SM, space::SM);
                add(ui)
            },
        )
        .inner
    })
    .inner
}

/// The header card of a palette or inspector: a tinted icon square, a
/// title and a secondary line.
pub fn header(ui: &mut Ui, icon: Icon, title: &str, subtitle: &str, corners: CornerRadius) {
    let c = colors(ui);
    Card::new()
        .corners(corners)
        .padding(Margin {
            left: space::XXL as i8,
            right: space::XXL as i8,
            top: space::XXL as i8,
            bottom: space::XL as i8,
        })
        .show(ui, |ui| {
            let (rect, resp) =
                ui.allocate_exact_size(vec2(ui.available_width(), 40.0), Sense::hover());
            let p = ui.painter();
            let square = Rect::from_min_size(rect.min, Vec2::splat(40.0));
            p.rect_filled(square, radius::MENU, c.accent_tint);
            icon::paint(
                p,
                Rect::from_center_size(square.center(), Vec2::splat(24.0)),
                icon,
                c.accent,
            );
            let x = rect.left() + 52.0;
            let tw = rect.right() - x;
            let tg = text::truncated(p, title, Type::Title, c.text, tw);
            let sg = text::truncated(p, subtitle, Type::Body, c.text_secondary, tw);
            let elided = tg.elided || sg.elided;
            let total = tg.size().y + 3.0 + sg.size().y;
            let top = rect.center().y - total / 2.0;
            let th = tg.size().y;
            p.galley(pos2(x, top), tg, c.text);
            p.galley(pos2(x, top + th + 3.0), sg, c.text_secondary);
            if elided {
                resp.on_hover_text(format!("{title}\n{subtitle}"));
            }
        });
}

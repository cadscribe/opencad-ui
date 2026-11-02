//! Toolbars: a row of tool buttons with separators, the floating toolbar
//! pill over a viewport, and the ribbon (groups of large and small tool
//! buttons with a group title).

use egui::{
    Align, Frame, Layout, Margin, Rect, Response, Role, Sense, Stroke, Ui, Vec2, pos2, vec2,
};

use crate::icon::{self, Icon};
use crate::state;
use crate::text::{self, Type};
use crate::theme::colors;
use crate::tokens::{Elevation, radius, space};
use crate::widgets::tooltip;

/// A toolbar row.
pub fn toolbar<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 2.0;
        add(ui)
    })
    .inner
}

/// A vertical separator between toolbar groups (horizontal in a
/// vertical toolbar).
pub fn separator(ui: &mut Ui) {
    if ui.layout().main_dir().is_vertical() {
        return separator_h(ui);
    }
    let c = colors(ui);
    let (rect, _) = ui.allocate_exact_size(vec2(9.0, 28.0), Sense::hover());
    ui.painter().vline(
        rect.center().x,
        rect.center().y - 9.0..=rect.center().y + 9.0,
        Stroke::new(1.0, c.divider),
    );
}

/// A horizontal separator between groups of a vertical toolbar.
pub fn separator_h(ui: &mut Ui) {
    let c = colors(ui);
    // As wide as the tools above it.
    let w = ui.min_rect().width().max(18.0);
    let (rect, _) = ui.allocate_exact_size(vec2(w, 9.0), Sense::hover());
    ui.painter().hline(
        rect.center().x - 9.0..=rect.center().x + 9.0,
        rect.center().y,
        Stroke::new(1.0, c.divider),
    );
}

/// The frame of the floating toolbar pill.
fn pill_frame(ui: &Ui) -> Frame {
    let c = colors(ui);
    Frame::new()
        .fill(c.glass)
        .stroke(Stroke::new(1.0, c.ring))
        .corner_radius(radius::OVERLAY)
        .inner_margin(Margin::same(space::XS as i8))
        .shadow(Elevation::Popover.shadow(c.dark))
}

/// The floating toolbar over a viewport: a translucent pill with a soft
/// shadow.
pub fn floating<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    pill_frame(ui).show(ui, |ui| toolbar(ui, add)).inner
}

/// The floating toolbar pill laid over a viewport `area`, placed by
/// `align` with `margin` from the edges; `vertical` stacks the tools.
/// The pill takes the pointer only inside its own bounds (also between
/// its buttons, so a click there does not reach the viewport below);
/// everywhere else the viewport keeps its input. Call it after the
/// viewport has allocated its response. Returns the content's value and
/// the pill's rect.
pub fn overlay<R>(
    ui: &mut Ui,
    id_salt: impl std::hash::Hash + std::fmt::Debug,
    area: Rect,
    align: egui::Align2,
    margin: Vec2,
    vertical: bool,
    add: impl FnOnce(&mut Ui) -> R,
) -> (R, Rect) {
    let id = egui::Id::unique(("opencad_ui_toolbar_overlay", id_salt));
    let size: Option<Vec2> = ui.data(|d| d.get_temp(id));
    let inner_area = area.shrink2(margin);
    let rect = align.align_size_within_rect(size.unwrap_or(Vec2::ZERO), inner_area);
    // Wider (taller) than the viewport: it starts at the edge and is cut
    // at the other end, so the first tools stay reachable.
    let rect = rect.translate(vec2(
        (inner_area.left() - rect.left()).max(0.0),
        (inner_area.top() - rect.top()).max(0.0),
    ));
    // The pill's background swallows the pointer; registered before the
    // buttons so that they stay on top of it.
    if size.is_some() {
        ui.interact(rect, id.with("background"), Sense::click_and_drag());
    }
    let mut child = ui.new_child(
        egui::UiBuilder::new()
            .id_salt(id)
            .max_rect(Rect::from_min_size(rect.min, inner_area.size()))
            .layout(Layout::top_down(Align::Min)),
    );
    child.set_clip_rect(area.intersect(ui.clip_rect()));
    let shown = pill_frame(&child).show(&mut child, |ui| {
        if vertical {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                add(ui)
            })
            .inner
        } else {
            toolbar(ui, add)
        }
    });
    let r = shown.response.rect;
    if size != Some(r.size()) {
        ui.data_mut(|d| d.insert_temp(id, r.size()));
        ui.ctx().request_repaint();
    }
    (shown.inner, r)
}

/// A toolbar button: an icon, optionally a label and a dropdown chevron
/// ("Iso VL ▾"); toggled buttons are tinted.
#[derive(Debug, Clone)]
#[must_use = "add it with `ui.add(…)`"]
pub struct ToolButton<'a> {
    icon: Option<Icon>,
    label: &'a str,
    tip: &'a str,
    selected: bool,
    dropdown: bool,
    note: &'a str,
    min_width: f32,
    height: f32,
}

impl<'a> ToolButton<'a> {
    pub fn icon(icon: Icon, tip: &'a str) -> Self {
        ToolButton {
            icon: Some(icon),
            label: "",
            tip,
            selected: false,
            dropdown: false,
            note: "",
            min_width: 0.0,
            height: 28.0,
        }
    }

    pub fn text(label: &'a str) -> Self {
        ToolButton {
            icon: None,
            label,
            tip: "",
            selected: false,
            dropdown: false,
            note: "",
            min_width: 0.0,
            height: 28.0,
        }
    }

    /// The chevron half of a split button: only a narrow chevron that
    /// opens the menu of the toggle beside it.
    pub fn chevron(tip: &'a str) -> Self {
        ToolButton {
            icon: None,
            label: "",
            tip,
            selected: false,
            dropdown: true,
            note: "",
            min_width: 0.0,
            height: 28.0,
        }
    }

    /// A tooltip (text buttons).
    pub fn tip(mut self, tip: &'a str) -> Self {
        self.tip = tip;
        self
    }

    /// A secondary note after the label, in tabular figures ("+2,70").
    pub fn note(mut self, note: &'a str) -> Self {
        self.note = note;
        self
    }

    /// The smallest width (equal buttons in a vertical toolbar).
    pub fn min_width(mut self, w: f32) -> Self {
        self.min_width = w;
        self
    }

    /// Height (default 28; compact toolbars in a tab header less). An
    /// icon button is square.
    pub fn height(mut self, h: f32) -> Self {
        self.height = h;
        self
    }

    pub fn label(mut self, label: &'a str) -> Self {
        self.label = label;
        self
    }

    pub fn selected(mut self, on: bool) -> Self {
        self.selected = on;
        self
    }

    /// A chevron: the button opens a menu.
    pub fn dropdown(mut self) -> Self {
        self.dropdown = true;
        self
    }
}

impl egui::Widget for ToolButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let c = colors(ui);
        let h = self.height;
        let lw = if self.label.is_empty() {
            0.0
        } else {
            text::galley(ui.painter(), self.label, Type::Body, c.text)
                .size()
                .x
        };
        let nw = if self.note.is_empty() {
            0.0
        } else {
            text::galley(ui.painter(), self.note, Type::Caption, c.text)
                .size()
                .x
                + space::SM
        };
        let chevron_only = self.icon.is_none() && lw == 0.0 && self.dropdown;
        let w = if lw == 0.0 && !self.dropdown {
            h + 2.0
        } else if chevron_only {
            16.0
        } else {
            2.0 * space::LG
                + if self.icon.is_some() {
                    14.0 + space::SM
                } else {
                    0.0
                }
                + lw
                + nw
                + if self.dropdown { 10.0 + space::XS } else { 0.0 }
        };
        let w = w.max(self.min_width);
        let (rect, resp) = ui.allocate_exact_size(vec2(w, h), Sense::click());
        crate::widgets::floatbar::note_item(ui, rect);
        let name = if self.label.is_empty() {
            self.tip
        } else {
            self.label
        };
        state::a11y_selected(&resp, Role::Button, self.selected, name);
        let s = state::state(ui, &resp);
        let p = ui.painter();
        if self.selected {
            p.rect_filled(rect, radius::TILE, c.accent_tint);
        }
        let hover = state::hover_fill(ui, resp.id, s, c);
        if hover.a() > 0 {
            p.rect_filled(rect, radius::TILE, hover);
        }
        let fg = if s.disabled { c.text_disabled } else { c.text };
        let ic = if s.disabled {
            c.text_disabled
        } else if self.selected {
            c.accent
        } else {
            c.icon
        };
        if chevron_only {
            icon::paint(
                p,
                Rect::from_center_size(rect.center(), Vec2::splat(10.0)),
                Icon::ChevronDown,
                c.text_secondary,
            );
        } else if lw == 0.0 && !self.dropdown {
            if let Some(i) = self.icon {
                icon::paint(
                    p,
                    Rect::from_center_size(rect.center(), Vec2::splat(15.0)),
                    i,
                    ic,
                );
            }
        } else {
            let mut x = rect.left() + space::LG;
            if let Some(i) = self.icon {
                icon::paint(
                    p,
                    Rect::from_center_size(pos2(x + 7.0, rect.center().y), Vec2::splat(14.0)),
                    i,
                    ic,
                );
                x += 14.0 + space::SM;
            }
            let g = text::galley(p, self.label, Type::Body, fg);
            text::paint_in(
                p,
                Rect::from_min_max(pos2(x, rect.top()), pos2(x + lw, rect.bottom())),
                g,
                Align::Min,
                fg,
            );
            x += lw + space::XS;
            if !self.note.is_empty() {
                // Right-aligned, so notes line up in a vertical toolbar.
                let tone = if self.selected {
                    c.accent
                } else {
                    c.text_tertiary
                };
                let g = text::galley(p, self.note, Type::Caption, tone);
                let right = rect.right() - space::LG;
                text::paint_in(
                    p,
                    Rect::from_min_max(
                        pos2(right - g.size().x, rect.top()),
                        pos2(right, rect.bottom()),
                    ),
                    g,
                    Align::Min,
                    tone,
                );
            }
            if self.dropdown {
                icon::paint(
                    p,
                    Rect::from_center_size(pos2(x + 5.0, rect.center().y), Vec2::splat(10.0)),
                    Icon::ChevronDown,
                    c.text_secondary,
                );
            }
        }
        if s.focused {
            state::focus_ring(ui, rect, radius::TILE, c);
        }
        if self.tip.is_empty() {
            resp
        } else {
            resp.on_hover_text(self.tip)
        }
    }
}

/// A ribbon group: tools side by side, the group title underneath and a
/// divider on the right.
pub fn ribbon_group<R>(ui: &mut Ui, title: &str, add: impl FnOnce(&mut Ui) -> R) -> R {
    let c = colors(ui);
    let top = ui.cursor().top();
    let r = ui.allocate_ui_with_layout(
        vec2(0.0, RIBBON_GROUP_H),
        Layout::top_down(Align::Center),
        |ui| {
            let inner = ui
                .horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = space::XS;
                    ui.add_space(space::LG);
                    let r = add(ui);
                    ui.add_space(space::LG);
                    r
                })
                .inner;
            ui.add_space(space::XS);
            let g = text::galley(ui.painter(), title, Type::Caption, c.text_tertiary);
            let (tr, _) = ui.allocate_exact_size(vec2(g.size().x, 14.0), Sense::hover());
            text::paint_in(ui.painter(), tr, g, Align::Center, c.text_tertiary);
            inner
        },
    );
    let rect = r.response.rect;
    ui.painter().vline(
        rect.right(),
        top + 2.0..=rect.bottom() - 2.0,
        Stroke::new(1.0, c.divider),
    );
    r.inner
}

/// The width of a [`ribbon_group`] whose tools are `content` wide
/// (their widths and the gaps between them).
pub fn ribbon_group_width(ui: &Ui, title: &str, content: f32) -> f32 {
    let c = colors(ui);
    let tw = text::galley(ui.painter(), title, Type::Caption, c.text_tertiary)
        .size()
        .x;
    (content + 2.0 * space::LG).max(tw)
}

/// A column of small ribbon tools.
pub fn ribbon_column<R>(ui: &mut Ui, add: impl FnOnce(&mut Ui) -> R) -> R {
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = 1.0;
        add(ui)
    })
    .inner
}

/// Size of a large ribbon tool.
pub const RIBBON_LARGE: Vec2 = vec2(66.0, 74.0);
/// Height of a small ribbon tool (three fit beside a large one).
pub const RIBBON_SMALL_H: f32 = 23.0;
/// Height of a ribbon group: the tools, a gap and the title.
pub const RIBBON_GROUP_H: f32 = RIBBON_LARGE.y + space::XS + 14.0;

/// A ribbon tool: large (icon over the label and its shortcut) or small
/// (icon, label and shortcut in a row; compact: the icon only). The
/// tooltip shows the label, the shortcut and an optional description,
/// also while the tool is disabled.
#[derive(Debug, Clone)]
#[must_use = "add it with `ui.add(…)`"]
pub struct RibbonTool<'a> {
    icon: Icon,
    label: &'a str,
    shortcut: &'a str,
    description: &'a str,
    large: bool,
    compact: bool,
    active: bool,
}

impl<'a> RibbonTool<'a> {
    pub fn large(icon: Icon, label: &'a str, shortcut: &'a str) -> Self {
        RibbonTool {
            icon,
            label,
            shortcut,
            description: "",
            large: true,
            compact: false,
            active: false,
        }
    }

    pub fn small(icon: Icon, label: &'a str, shortcut: &'a str) -> Self {
        RibbonTool {
            large: false,
            ..Self::large(icon, label, shortcut)
        }
    }

    /// The running tool.
    pub fn active(mut self, on: bool) -> Self {
        self.active = on;
        self
    }

    /// A small tool shows only its icon (a narrow ribbon); the label is
    /// in the tooltip.
    pub fn compact(mut self, on: bool) -> Self {
        self.compact = on;
        self
    }

    /// A line under the title of the tooltip (how to use the tool, why
    /// it is not available).
    pub fn description(mut self, text: &'a str) -> Self {
        self.description = text;
        self
    }

    /// The size the tool takes (to lay out a ribbon before drawing it).
    pub fn size(&self, ui: &Ui) -> Vec2 {
        if self.large {
            return RIBBON_LARGE;
        }
        if self.compact {
            return vec2(2.0 * space::SM + 16.0, RIBBON_SMALL_H);
        }
        let c = colors(ui);
        let lw = text::galley(ui.painter(), self.label, Type::Body, c.text)
            .size()
            .x;
        let kw = if self.shortcut.is_empty() {
            0.0
        } else {
            text::galley(ui.painter(), self.shortcut, Type::Caption, c.text)
                .size()
                .x
                + space::SM
        };
        vec2(2.0 * space::MD + 16.0 + 7.0 + lw + kw, RIBBON_SMALL_H)
    }
}

impl egui::Widget for RibbonTool<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let c = colors(ui);
        let (rect, resp) = ui.allocate_exact_size(self.size(ui), Sense::click());
        state::a11y_selected(&resp, Role::Button, self.active, self.label);
        let s = state::state(ui, &resp);
        let p = ui.painter();
        let r = if self.large {
            radius::MENU
        } else {
            radius::FIELD
        };
        if self.active {
            p.rect_filled(rect, r, c.accent_tint);
        }
        let hover = state::hover_fill(ui, resp.id, s, c);
        if hover.a() > 0 {
            p.rect_filled(rect, r, hover);
        }
        let fg = if s.disabled {
            c.text_disabled
        } else if self.active && self.large {
            c.accent_text
        } else {
            c.text
        };
        let ic = if s.disabled {
            c.text_disabled
        } else if self.active {
            c.accent
        } else {
            c.icon
        };
        let kc = if s.disabled {
            c.text_disabled
        } else {
            c.text_tertiary
        };
        if self.large {
            let icon_r = Rect::from_center_size(
                pos2(rect.center().x, rect.top() + 8.0 + 13.0),
                Vec2::splat(26.0),
            );
            icon::paint(p, icon_r, self.icon, ic);
            let g = text::wrapped(p, self.label, Type::Caption, fg, rect.width() - 4.0);
            let gy = icon_r.bottom() + 5.0;
            let gs = g.size();
            p.galley(pos2(rect.center().x - gs.x / 2.0, gy), g, fg);
            // The shortcut under the label, unless the label wrapped.
            if !self.shortcut.is_empty() && gy + gs.y + 12.0 <= rect.bottom() {
                let k = text::galley(p, self.shortcut, Type::Caption, kc);
                let ks = k.size();
                p.galley(pos2(rect.center().x - ks.x / 2.0, gy + gs.y - 1.0), k, kc);
            }
        } else if self.compact {
            icon::paint(
                p,
                Rect::from_center_size(rect.center(), Vec2::splat(16.0)),
                self.icon,
                ic,
            );
        } else {
            let mut x = rect.left() + space::MD;
            icon::paint(
                p,
                Rect::from_center_size(pos2(x + 8.0, rect.center().y), Vec2::splat(16.0)),
                self.icon,
                ic,
            );
            x += 16.0 + 7.0;
            let g = text::galley(p, self.label, Type::Body, fg);
            let gw = g.size().x;
            text::paint_in(
                p,
                Rect::from_min_max(pos2(x, rect.top()), pos2(x + gw, rect.bottom())),
                g,
                Align::Min,
                fg,
            );
            if !self.shortcut.is_empty() {
                let k = text::galley(p, self.shortcut, Type::Caption, kc);
                text::paint_in(
                    p,
                    Rect::from_min_max(pos2(x + gw + space::SM, rect.top()), rect.max),
                    k,
                    Align::Min,
                    kc,
                );
            }
        }
        if s.focused {
            state::focus_ring(ui, rect, r, c);
        }
        let (label, shortcut, description) = (self.label, self.shortcut, self.description);
        resp.on_hover_ui(|ui| tooltip::content(ui, label, shortcut, description))
            .on_disabled_hover_ui(|ui| tooltip::content(ui, label, shortcut, description))
    }
}

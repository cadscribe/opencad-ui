//! Menus and popovers in the macOS style: a raised surface with a soft
//! shadow and a hairline, rows highlighted in the accent with white
//! text, a checkmark column, icons, right-aligned shortcuts, submenus,
//! separators and section headers.
//!
//! [`menu_popup`] and [`context_menu`] open egui popups styled like this;
//! [`panel`] draws the same surface inline (the gallery).

use egui::containers::menu::SubMenu;
use egui::style::StyleModifier;
use egui::{
    Align, Frame, Margin, Popup, Rect, Response, Role, Sense, Stroke, Ui, Vec2, pos2, vec2,
};

use crate::icon::{self, Icon};
use crate::state;
use crate::text::{self, Type};
use crate::theme::{self, colors};
use crate::tokens::{Colors, Elevation, radius, size, space};

/// The narrowest menu row.
const MENU_ROW_MIN_W: f32 = 120.0;

/// The width of a full-width element of a menu (separator, header): none
/// in the popup's sizing pass, so it does not widen the menu.
fn fill_width(ui: &Ui) -> f32 {
    if ui.is_sizing_pass() {
        0.0
    } else {
        ui.available_width()
    }
}

/// The frame of menus.
pub fn menu_frame(c: &Colors) -> Frame {
    Frame::new()
        .fill(c.surface_raised)
        .stroke(Stroke::new(0.5, c.border_strong))
        .corner_radius(radius::MENU)
        .inner_margin(Margin::same(space::XS as i8 + 1))
        .shadow(Elevation::Popover.shadow(c.dark))
}

/// The frame of popovers (richer content than a menu).
pub fn popover_frame(c: &Colors) -> Frame {
    Frame::new()
        .fill(c.surface_raised)
        .stroke(Stroke::new(0.5, c.border_strong))
        .corner_radius(radius::OVERLAY)
        .inner_margin(Margin::same(space::LG as i8))
        .shadow(Elevation::Popover.shadow(c.dark))
}

/// The style of a menu or popover opened from a `ui` in the given theme.
fn style_modifier(dark: bool) -> StyleModifier {
    StyleModifier::new(move |style: &mut egui::Style| {
        style.visuals = theme::visuals(Colors::of(dark));
        egui::containers::menu::menu_style(style);
        style.spacing.item_spacing = vec2(space::XS, 0.0);
        style.spacing.button_padding = vec2(space::MD, 3.0);
        style.spacing.menu_margin = Margin::same(space::XS as i8 + 1);
        style.visuals.menu_corner_radius = egui::CornerRadius::same(radius::MENU);
        style.visuals.window_stroke = Stroke::new(0.5, Colors::of(dark).border_strong);
        style.visuals.window_fill = Colors::of(dark).surface_raised;
    })
}

/// A menu that opens when `response` is clicked, below it.
pub fn menu_popup<'a>(ui: &Ui, response: &'a Response) -> Popup<'a> {
    let dark = ui.visuals().dark_mode;
    Popup::menu(response)
        .style(style_modifier(dark))
        .frame(menu_frame(Colors::of(dark)))
}

/// A context menu (secondary click) on `response`.
pub fn context_menu(ui: &Ui, response: &Response, add: impl FnOnce(&mut Ui)) {
    let dark = ui.visuals().dark_mode;
    Popup::context_menu(response)
        .style(style_modifier(dark))
        .frame(menu_frame(Colors::of(dark)))
        .show(|ui| {
            ui.set_min_width(size::MENU_W);
            add(ui)
        });
}

/// A context menu of `response` that also opens when `open` (a right
/// click the response itself did not get, e.g. on a tool over it).
pub fn context_menu_forced(ui: &Ui, response: &Response, open: bool, add: impl FnOnce(&mut Ui)) {
    let dark = ui.visuals().dark_mode;
    Popup::context_menu(response)
        .open_memory(if open || response.secondary_clicked() {
            Some(egui::SetOpenCommand::Bool(true))
        } else {
            None
        })
        .style(style_modifier(dark))
        .frame(menu_frame(Colors::of(dark)))
        .show(|ui| {
            ui.set_min_width(size::MENU_W_SM);
            add(ui)
        });
}

/// A popover with free content that opens when `response` is clicked
/// (type picker, colour picker). It closes on a click outside.
pub fn popover<'a>(ui: &Ui, response: &'a Response) -> Popup<'a> {
    let dark = ui.visuals().dark_mode;
    Popup::menu(response)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .style(StyleModifier::new(move |style: &mut egui::Style| {
            style.visuals = theme::visuals(Colors::of(dark));
        }))
        .frame(popover_frame(Colors::of(dark)))
}

/// The menu surface drawn inline (gallery, previews), `width` wide.
pub fn panel<R>(ui: &mut Ui, width: f32, add: impl FnOnce(&mut Ui) -> R) -> R {
    let c = colors(ui);
    menu_frame(c)
        .show(ui, |ui| {
            ui.set_width(width);
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing = vec2(space::XS, 0.0);
                add(ui)
            })
            .inner
        })
        .inner
}

/// A row of a menu.
#[derive(Debug, Clone)]
#[must_use = "call `.show(ui)`"]
pub struct MenuItem<'a> {
    label: &'a str,
    icon: Option<Icon>,
    shortcut: &'a str,
    note: &'a str,
    check: Option<bool>,
    enabled: bool,
    destructive: bool,
    submenu: bool,
    highlighted: bool,
    icon_column: bool,
}

impl<'a> MenuItem<'a> {
    pub fn new(label: &'a str) -> Self {
        MenuItem {
            label,
            icon: None,
            shortcut: "",
            note: "",
            check: None,
            enabled: true,
            destructive: false,
            submenu: false,
            highlighted: false,
            icon_column: false,
        }
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// The keyboard shortcut, right-aligned ("⌘Z").
    pub fn shortcut(mut self, shortcut: &'a str) -> Self {
        self.shortcut = shortcut;
        self
    }

    /// A secondary text after the label ("Architecture › Elements"),
    /// truncated first when the row is narrow.
    pub fn note(mut self, note: &'a str) -> Self {
        self.note = note;
        self
    }

    /// Reserves the checkmark column; `true` shows the checkmark.
    pub fn checked(mut self, on: bool) -> Self {
        self.check = Some(on);
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Red text (delete, discard).
    pub fn destructive(mut self) -> Self {
        self.destructive = true;
        self
    }

    /// Keeps the icon column empty, so the text lines up with rows that
    /// have icons.
    pub fn icon_space(mut self) -> Self {
        self.icon_column = true;
        self
    }

    /// Shows the row highlighted (gallery: the keyboard row).
    pub fn highlighted(mut self, on: bool) -> Self {
        self.highlighted = on;
        self
    }

    /// The width of the row's content with its paddings.
    fn natural_width(&self, ui: &Ui) -> f32 {
        let c = colors(ui);
        let p = ui.painter();
        let measure = |t: &str, ty: Type| text::galley(p, t, ty, c.text).size().x;
        let mut w = space::MD * 2.0 + measure(self.label, Type::Label);
        if self.check.is_some() {
            w += 18.0;
        }
        if self.icon.is_some() || self.icon_column {
            w += 24.0;
        }
        if self.submenu {
            w += 16.0;
        } else if !self.shortcut.is_empty() {
            w += measure(self.shortcut, Type::Label) + space::XL;
        }
        if !self.note.is_empty() {
            w += space::MD + measure(self.note, Type::Body);
        }
        w
    }

    fn submenu_arrow(mut self) -> Self {
        self.submenu = true;
        self
    }

    /// Shows the row. A click closes the menu (egui's menu behaviour).
    pub fn show(self, ui: &mut Ui) -> Response {
        let c = colors(ui);
        // In the popup's sizing pass a row asks only for its content, so
        // the menu is as wide as its longest row (not the 600 pt default
        // of an egui area); afterwards every row fills that width.
        let w = if ui.is_sizing_pass() {
            self.natural_width(ui)
        } else {
            ui.available_width()
        }
        .max(MENU_ROW_MIN_W);
        let sense = if self.enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let (rect, resp) = ui.allocate_exact_size(vec2(w, size::MENU_ROW), sense);
        let role = match self.check {
            Some(_) => Role::MenuItemCheckBox,
            None => Role::MenuItem,
        };
        let enabled = self.enabled;
        let label = self.label;
        let check = self.check;
        resp.widget_info(|| {
            let mut info = egui::WidgetInfo::labeled(role, enabled, label);
            info.selected = check;
            info
        });
        if !ui.is_rect_visible(rect) {
            return resp;
        }
        let s = state::state(ui, &resp);
        let hot = self.enabled && (s.hovered || s.focused || self.highlighted);
        let p = ui.painter();
        if hot {
            p.rect_filled(rect, radius::ROW, c.highlight);
        }
        let (fg, dim) = if !self.enabled || s.disabled {
            (c.text_disabled, c.text_disabled)
        } else if hot {
            (c.on_highlight, c.on_highlight)
        } else if self.destructive {
            (c.danger.text, c.text_tertiary)
        } else {
            (c.text, c.text_tertiary)
        };
        let mut x = rect.left() + space::MD;
        if let Some(on) = self.check {
            if on {
                icon::paint(
                    p,
                    Rect::from_center_size(pos2(x + 6.0, rect.center().y), Vec2::splat(14.0)),
                    Icon::Check,
                    fg,
                );
            }
            x += 18.0;
        }
        if let Some(i) = self.icon {
            icon::paint(
                p,
                Rect::from_center_size(pos2(x + 8.0, rect.center().y), Vec2::splat(16.0)),
                i,
                if hot {
                    fg
                } else if self.enabled {
                    c.icon
                } else {
                    fg
                },
            );
            x += 24.0;
        } else if self.icon_column {
            x += 24.0;
        }
        let mut right = rect.right() - space::MD;
        if self.submenu {
            icon::paint(
                p,
                Rect::from_center_size(pos2(right - 5.0, rect.center().y), Vec2::splat(12.0)),
                Icon::ChevronRight,
                dim,
            );
            right -= 16.0;
        } else if !self.shortcut.is_empty() {
            let g = text::galley(p, self.shortcut, Type::Label, dim);
            let gw = g.size().x;
            text::paint_in(
                p,
                Rect::from_min_max(pos2(right - gw, rect.top()), pos2(right, rect.bottom())),
                g,
                Align::Min,
                dim,
            );
            right -= gw + space::XL;
        }
        let g = text::truncated(p, self.label, Type::Label, fg, right - x);
        let gw = g.size().x;
        text::paint_in(
            p,
            Rect::from_min_max(pos2(x, rect.top()), pos2(right, rect.bottom())),
            g,
            Align::Min,
            fg,
        );
        let nx = x + gw + space::MD;
        if !self.note.is_empty() && right - nx > 24.0 {
            let n = text::truncated(p, self.note, Type::Body, dim, right - nx);
            text::paint_in(
                p,
                Rect::from_min_max(pos2(nx, rect.top()), pos2(right, rect.bottom())),
                n,
                Align::Min,
                dim,
            );
        }
        resp
    }
}

/// A row (`item`) that opens a submenu on hover or with the right arrow
/// key.
pub fn submenu<R>(ui: &mut Ui, item: MenuItem<'_>, add: impl FnOnce(&mut Ui) -> R) -> Response {
    let resp = item.submenu_arrow().show(ui);
    let dark = ui.visuals().dark_mode;
    SubMenu::new()
        .config(egui::containers::menu::MenuConfig::new().style(style_modifier(dark)))
        .show(ui, &resp, |ui| {
            ui.set_min_width(size::MENU_W - 40.0);
            add(ui)
        });
    resp
}

/// A thin line between groups of rows.
pub fn separator(ui: &mut Ui) {
    let c = colors(ui);
    let (rect, _) = ui.allocate_exact_size(vec2(fill_width(ui), 9.0), Sense::hover());
    ui.painter().hline(
        rect.x_range().shrink(space::MD),
        rect.center().y,
        Stroke::new(1.0, c.divider),
    );
}

/// A small title above a group of rows (with an optional count).
pub fn section_header(ui: &mut Ui, title: &str, count: Option<usize>) {
    let c = colors(ui);
    let (rect, _) = ui.allocate_exact_size(vec2(fill_width(ui), 22.0), Sense::hover());
    let p = ui.painter();
    let inner = rect.shrink2(vec2(space::MD, 0.0));
    let g = text::truncated(
        p,
        title,
        Type::Overline,
        c.text_tertiary,
        inner.width() - 30.0,
    );
    text::paint_in(p, inner, g, Align::Min, c.text_tertiary);
    if let Some(n) = count {
        text::paint_tabular(
            p,
            inner,
            &n.to_string(),
            Type::Caption,
            c.text_tertiary,
            Align::Max,
        );
    }
}

//! Applying the tokens to egui: `Visuals` and `Spacing` of both themes,
//! so egui's own widgets (scroll bars, text edits, windows, tooltips)
//! match the foundation's widgets.

use egui::style::WidgetVisuals;
use egui::{Color32, CornerRadius, Margin, Stroke, TextStyle, Theme, Ui, Visuals};

use crate::text::Type;
use crate::tokens::{Colors, Elevation, motion, radius, space};

/// The colours of the theme `ui` is drawn in.
pub fn colors(ui: &Ui) -> &'static Colors {
    Colors::of(ui.visuals().dark_mode)
}

/// The colours of the theme of a style.
pub fn colors_of(style: &egui::Style) -> &'static Colors {
    Colors::of(style.visuals.dark_mode)
}

fn widget(fill: Color32, stroke: Stroke, fg: Color32, r: u8) -> WidgetVisuals {
    WidgetVisuals {
        bg_fill: fill,
        weak_bg_fill: fill,
        bg_stroke: stroke,
        corner_radius: CornerRadius::same(r),
        fg_stroke: Stroke::new(1.0, fg),
        expansion: 0.0,
    }
}

/// egui visuals from the tokens.
pub fn visuals(c: &Colors) -> Visuals {
    let mut v = if c.dark {
        Visuals::dark()
    } else {
        Visuals::light()
    };
    let border = Stroke::new(1.0, c.border);
    v.widgets.noninteractive = widget(
        c.surface,
        Stroke::new(1.0, c.divider),
        c.text,
        radius::FIELD,
    );
    v.widgets.inactive = widget(c.field, border, c.text, radius::FIELD);
    v.widgets.hovered = widget(
        c.field,
        Stroke::new(1.0, c.border_mixed),
        c.text,
        radius::FIELD,
    );
    v.widgets.active = widget(
        crate::tokens::over(c.field, c.pressed),
        Stroke::new(1.0, c.accent),
        c.text,
        radius::FIELD,
    );
    v.widgets.open = widget(
        c.field,
        Stroke::new(1.0, c.border_mixed),
        c.text,
        radius::FIELD,
    );
    v.selection.bg_fill = c.text_selection;
    v.selection.stroke = Stroke::new(1.0, c.accent);
    v.hyperlink_color = c.accent;
    v.faint_bg_color = c.surface_sunken;
    v.extreme_bg_color = c.field;
    v.text_edit_bg_color = Some(c.field);
    v.code_bg_color = c.surface_sunken;
    v.weak_text_color = Some(c.text_secondary);
    v.override_text_color = None;
    v.warn_fg_color = c.warning.text;
    v.error_fg_color = c.danger.text;
    v.panel_fill = c.surface;
    v.window_fill = c.surface_raised;
    v.window_stroke = Stroke::new(1.0, c.ring);
    v.window_corner_radius = CornerRadius::same(radius::OVERLAY);
    v.window_shadow = Elevation::Dialog.shadow(c.dark);
    v.menu_corner_radius = CornerRadius::same(radius::MENU);
    v.popup_shadow = Elevation::Popover.shadow(c.dark);
    v.text_cursor.stroke = Stroke::new(1.5, c.accent);
    v.collapsing_header_frame = false;
    v.indent_has_left_vline = false;
    v.button_frame = true;
    v.striped = false;
    v.slider_trailing_fill = true;
    v.interact_cursor = None;
    v
}

/// Spacing and text styles from the tokens.
pub fn apply_metrics(style: &mut egui::Style) {
    style.text_styles = [
        (TextStyle::Small, Type::Caption.font()),
        (TextStyle::Body, Type::Body.font()),
        (TextStyle::Button, Type::Body.font()),
        (TextStyle::Heading, Type::Title.font()),
        (TextStyle::Monospace, Type::Mono.font()),
    ]
    .into();
    let s = &mut style.spacing;
    s.item_spacing = egui::vec2(space::MD, space::SM);
    s.button_padding = egui::vec2(space::MD, 3.0);
    s.interact_size = egui::vec2(24.0, crate::tokens::size::FIELD_H);
    s.menu_margin = Margin::same(space::XS as i8 + 1);
    s.window_margin = Margin::same(space::XXL as i8);
    s.indent = space::XXL;
    s.icon_width = 14.0;
    s.icon_width_inner = 8.0;
    s.icon_spacing = space::SM;
    s.tooltip_width = 320.0;
    s.scroll.bar_width = 8.0;
    s.scroll.floating = true;
    // egui's edge fade is off; screens use `widgets::scroll::edges`.
    s.scroll.fade.strength = 0.0;
    style.interaction.tooltip_delay = 0.5;
    style.interaction.selectable_labels = false;
    style.compact_menu_style = true;
    style.animation_time = motion::NORMAL;
}

/// The style of a theme.
pub fn style(dark: bool) -> egui::Style {
    let mut style = egui::Style {
        visuals: visuals(Colors::of(dark)),
        ..Default::default()
    };
    apply_metrics(&mut style);
    style
}

/// Installs the visuals and metrics of both themes.
pub fn install(ctx: &egui::Context) {
    for (theme, dark) in [(Theme::Light, false), (Theme::Dark, true)] {
        ctx.style_mut_of(theme, |s| {
            s.visuals = visuals(Colors::of(dark));
            apply_metrics(s);
        });
    }
}

/// Draws `add` in the given theme (the gallery shows both side by side).
pub fn scope_theme<R>(ui: &mut Ui, dark: bool, add: impl FnOnce(&mut Ui) -> R) -> R {
    ui.scope(|ui| {
        let mut s = (**ui.style()).clone();
        s.visuals = visuals(Colors::of(dark));
        ui.set_style(s);
        add(ui)
    })
    .inner
}

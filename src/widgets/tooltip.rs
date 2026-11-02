//! Tooltips: a short text, or a title with a shortcut and a description
//! (tools in the ribbon). egui positions and delays them; the frame comes
//! from the theme's popup visuals.

use egui::{Align, Response, Ui};

use crate::text::{self, Type};
use crate::theme::colors;
use crate::tokens::space;

/// The content of a rich tooltip (also drawn inline by the gallery).
pub fn content(ui: &mut Ui, title: &str, shortcut: &str, description: &str) {
    let c = colors(ui);
    ui.set_max_width(280.0);
    ui.spacing_mut().item_spacing.y = space::XS;
    ui.horizontal(|ui| {
        ui.label(text::rich(title, Type::BodyStrong, c.text));
        if !shortcut.is_empty() {
            ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                ui.label(text::rich(shortcut, Type::Body, c.text_tertiary));
            });
        }
    });
    if !description.is_empty() {
        ui.label(text::rich(description, Type::Body, c.text_secondary));
    }
}

/// A plain tooltip.
pub fn tip(response: Response, text: &str) -> Response {
    let t = text.to_owned();
    response.on_hover_ui(move |ui| {
        let c = colors(ui);
        ui.label(text::rich(&t, Type::Body, c.text));
    })
}

/// A tooltip with a title, a shortcut and a description.
pub fn rich(response: Response, title: &str, shortcut: &str, description: &str) -> Response {
    let (t, s, d) = (
        title.to_owned(),
        shortcut.to_owned(),
        description.to_owned(),
    );
    response.on_hover_ui(move |ui| content(ui, &t, &s, &d))
}

/// The tooltip surface drawn inline (gallery).
pub fn panel(ui: &mut Ui, add: impl FnOnce(&mut Ui)) {
    let c = colors(ui);
    egui::Frame::new()
        .fill(c.surface_raised)
        .stroke(egui::Stroke::new(0.5, c.border_strong))
        .corner_radius(crate::tokens::radius::FIELD)
        .inner_margin(egui::Margin::symmetric(space::MD as i8, space::SM as i8))
        .shadow(crate::tokens::Elevation::Popover.shadow(c.dark))
        .show(ui, add);
}

//! Typography: the type scale, text layout helpers (truncation) and
//! tabular figures.
//!
//! egui has no OpenType features, so `tnum` cannot be switched on. Numbers
//! that must line up (tables, coordinates, field values outside editing)
//! are painted with [`paint_tabular`] / [`tabular`]: every digit in a cell
//! as wide as the widest digit.

use std::sync::Arc;

use egui::text::{LayoutJob, TextFormat, TextWrapping};
use egui::{
    Align, Color32, FontId, Galley, Painter, Pos2, Rect, Response, RichText, Sense, Ui, vec2,
};

use crate::fonts::Weight;

/// A style of the type scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    /// 11 pt: hints, chip text, table headers, ribbon labels.
    Caption,
    /// 11 pt medium: chips, small buttons.
    CaptionStrong,
    /// 11 pt semibold, upper case, tracked: section titles.
    Overline,
    /// 12 pt: palette and panel text, fields.
    Body,
    /// 12 pt medium: anchors, emphasised values.
    BodyMedium,
    /// 12 pt semibold: selected tabs, names in lists.
    BodyStrong,
    /// 13 pt: dialogs, menus, wizard text.
    Label,
    /// 13 pt semibold: buttons in dialogs, dialog list headers.
    LabelStrong,
    /// 14 pt semibold: card and panel titles.
    Title,
    /// 20 pt semibold: dialog and wizard headings, empty states.
    Headline,
    /// 26 pt semibold: large values (U-values).
    Display,
    /// 12 pt monospace: code, commands.
    Mono,
}

impl Type {
    pub const ALL: [Type; 12] = [
        Type::Caption,
        Type::CaptionStrong,
        Type::Overline,
        Type::Body,
        Type::BodyMedium,
        Type::BodyStrong,
        Type::Label,
        Type::LabelStrong,
        Type::Title,
        Type::Headline,
        Type::Display,
        Type::Mono,
    ];

    /// Size in points.
    pub const fn size(self) -> f32 {
        match self {
            Type::Caption | Type::CaptionStrong | Type::Overline => 11.0,
            Type::Body | Type::BodyMedium | Type::BodyStrong | Type::Mono => 12.0,
            Type::Label | Type::LabelStrong => 13.0,
            Type::Title => 14.0,
            Type::Headline => 20.0,
            Type::Display => 26.0,
        }
    }

    pub fn weight(self) -> Weight {
        match self {
            Type::Caption | Type::Body | Type::Label | Type::Mono => Weight::Regular,
            Type::CaptionStrong | Type::BodyMedium => Weight::Medium,
            Type::Overline
            | Type::BodyStrong
            | Type::LabelStrong
            | Type::Title
            | Type::Headline
            | Type::Display => Weight::Semibold,
        }
    }

    /// Extra letter spacing in points.
    pub fn tracking(self) -> f32 {
        match self {
            Type::Overline => 0.45,
            Type::Headline | Type::Display => -0.2,
            _ => 0.0,
        }
    }

    /// Whether the text is shown in upper case.
    pub fn upper(self) -> bool {
        matches!(self, Type::Overline)
    }

    pub fn font(self) -> FontId {
        match self {
            Type::Mono => FontId::monospace(self.size()),
            _ => FontId::new(self.size(), self.weight().family()),
        }
    }

    /// A short name for the gallery.
    pub fn name(self) -> &'static str {
        match self {
            Type::Caption => "caption",
            Type::CaptionStrong => "caption-strong",
            Type::Overline => "overline",
            Type::Body => "body",
            Type::BodyMedium => "body-medium",
            Type::BodyStrong => "body-strong",
            Type::Label => "label",
            Type::LabelStrong => "label-strong",
            Type::Title => "title",
            Type::Headline => "headline",
            Type::Display => "display",
            Type::Mono => "mono",
        }
    }

    fn apply(self, text: &str) -> String {
        if self.upper() {
            text.to_uppercase()
        } else {
            text.to_owned()
        }
    }

    pub fn format(self, color: Color32) -> TextFormat {
        TextFormat {
            font_id: self.font(),
            extra_letter_spacing: self.tracking(),
            color,
            ..Default::default()
        }
    }
}

/// `text` in the style `t` and `color`, for egui's own widgets.
pub fn rich(text: impl Into<String>, t: Type, color: Color32) -> RichText {
    RichText::new(t.apply(&text.into()))
        .font(t.font())
        .extra_letter_spacing(t.tracking())
        .color(color)
}

/// One line of `text`, not wrapped.
pub fn galley(painter: &Painter, text: &str, t: Type, color: Color32) -> Arc<Galley> {
    let job = LayoutJob::single_section(t.apply(text), t.format(color));
    painter.layout_job(job)
}

/// One line of `text`, cut with "…" at `width` (check `galley.elided`).
pub fn truncated(
    painter: &Painter,
    text: &str,
    t: Type,
    color: Color32,
    width: f32,
) -> Arc<Galley> {
    let mut job = LayoutJob::single_section(t.apply(text), t.format(color));
    job.wrap = TextWrapping::truncate_at_width(width.max(1.0));
    painter.layout_job(job)
}

/// `text` wrapped at `width`.
pub fn wrapped(painter: &Painter, text: &str, t: Type, color: Color32, width: f32) -> Arc<Galley> {
    let mut job = LayoutJob::single_section(t.apply(text), t.format(color));
    job.wrap = TextWrapping::wrap_at_width(width.max(1.0));
    painter.layout_job(job)
}

/// `text` wrapped at `width` in at most `rows` lines (the last one cut off
/// with "…"), each centred on x = 0: painted at the middle of a place.
pub fn centred(
    painter: &Painter,
    text: &str,
    t: Type,
    color: Color32,
    width: f32,
    rows: usize,
) -> Arc<Galley> {
    let mut job = LayoutJob::single_section(t.apply(text), t.format(color));
    job.wrap = TextWrapping {
        max_width: width.max(1.0),
        max_rows: rows,
        ..TextWrapping::default()
    };
    job.halign = egui::Align::Center;
    painter.layout_job(job)
}

/// Paints `galley` vertically centred in `rect`, aligned by `align`.
pub fn paint_in(painter: &Painter, rect: Rect, galley: Arc<Galley>, align: Align, color: Color32) {
    let x = match align {
        Align::Min => rect.left(),
        Align::Center => rect.center().x - galley.size().x / 2.0,
        Align::Max => rect.right() - galley.size().x,
    };
    let y = rect.center().y - galley.size().y / 2.0;
    painter.galley(Pos2::new(x, y), galley, color);
}

/// A label in a style of the type scale (no wrapping, truncated with a
/// tooltip when it does not fit).
pub fn label(ui: &mut Ui, text: &str, t: Type, color: Color32) -> Response {
    let w = ui.available_width();
    let g = truncated(ui.painter(), text, t, color, w);
    let elided = g.elided;
    let (rect, resp) = ui.allocate_exact_size(g.size(), Sense::hover());
    ui.painter().galley(rect.min, g, color);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Label, true, text));
    if elided {
        resp.on_hover_text(text)
    } else {
        resp
    }
}

/// The width of the widest digit of `font`.
fn digit_width(painter: &Painter, font: &FontId) -> f32 {
    painter.fonts_mut(|f| {
        ('0'..='9')
            .map(|c| f.glyph_width(font, c))
            .fold(0.0, f32::max)
    })
}

/// Width of `text` with tabular figures.
pub fn tabular_width(painter: &Painter, text: &str, t: Type) -> f32 {
    let font = t.font();
    let dw = digit_width(painter, &font);
    painter.fonts_mut(|f| {
        text.chars()
            .map(|c| {
                if c.is_ascii_digit() {
                    dw
                } else {
                    f.glyph_width(&font, c)
                }
            })
            .sum::<f32>()
            + t.tracking() * text.chars().count() as f32
    })
}

/// Paints `text` with tabular figures (every digit in a cell as wide as
/// the widest digit), vertically centred in `rect`, aligned by `align`.
pub fn paint_tabular(
    painter: &Painter,
    rect: Rect,
    text: &str,
    t: Type,
    color: Color32,
    align: Align,
) {
    let font = t.font();
    let dw = digit_width(painter, &font);
    let total = tabular_width(painter, text, t);
    let mut x = match align {
        Align::Min => rect.left(),
        Align::Center => rect.center().x - total / 2.0,
        Align::Max => rect.right() - total,
    };
    let row_h = painter.fonts_mut(|f| f.row_height(&font));
    let y = rect.center().y - row_h / 2.0;
    let mut run = String::new();
    let flush = |run: &mut String, x: &mut f32| {
        if !run.is_empty() {
            let g = galley(painter, run, t, color);
            let w = g.size().x;
            painter.galley(Pos2::new(*x, y), g, color);
            *x += w;
            run.clear();
        }
    };
    for c in text.chars() {
        if c.is_ascii_digit() {
            flush(&mut run, &mut x);
            let g = galley(painter, &c.to_string(), t, color);
            let gx = x + (dw - g.size().x) / 2.0;
            painter.galley(Pos2::new(gx, y), g, color);
            x += dw + t.tracking();
        } else {
            run.push(c);
        }
    }
    flush(&mut run, &mut x);
}

/// A number label with tabular figures, right-aligned in `width` (or its
/// own width when `width` is `None`).
pub fn tabular(ui: &mut Ui, text: &str, t: Type, color: Color32, width: Option<f32>) -> Response {
    let w = width.unwrap_or_else(|| tabular_width(ui.painter(), text, t));
    let h = ui.ctx().fonts_mut(|f| f.row_height(&t.font()));
    let (rect, resp) = ui.allocate_exact_size(vec2(w, h), Sense::hover());
    paint_tabular(ui.painter(), rect, text, t, color, Align::Max);
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Label, true, text));
    resp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_is_ordered_and_overline_is_upper_case() {
        assert!(Type::Caption.size() < Type::Body.size());
        assert!(Type::Body.size() < Type::Title.size());
        assert!(Type::Title.size() < Type::Headline.size());
        assert_eq!(Type::Overline.apply("Schichten"), "SCHICHTEN");
        assert_eq!(Type::Body.apply("Schichten"), "Schichten");
    }

    #[test]
    fn tabular_digits_have_equal_width() {
        let ctx = egui::Context::default();
        crate::fonts::install(&ctx, |_| {});
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            let p = ui.painter();
            let a = tabular_width(p, "1111", Type::Body);
            let b = tabular_width(p, "8888", Type::Body);
            assert!((a - b).abs() < 1e-3, "{a} vs {b}");
        });
        out.textures_delta.clear();
    }
}

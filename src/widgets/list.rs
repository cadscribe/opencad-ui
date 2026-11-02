//! List rows (one or two lines, a leading icon, swatch or painted
//! thumbnail, trailing meta text and a tag) and list group headers.

use egui::{
    Align, Color32, Painter, Rect, Response, Role, Sense, Stroke, StrokeKind, Ui, Vec2, pos2, vec2,
};

use crate::icon::{self, Icon};
use crate::state;
use crate::text::{self, Type};
use crate::theme::colors;
use crate::tokens::{Tone, radius, size, space};

/// How a selected row looks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Selection {
    /// Accent fill with white text (pickers, menus).
    #[default]
    Strong,
    /// A soft tint (sidebars, trees, tables).
    Soft,
}

/// Paints a thumbnail into a rectangle.
pub type ThumbPainter<'a> = Box<dyn FnOnce(&Painter, Rect) + 'a>;

/// What leads a row.
pub enum Leading<'a> {
    None,
    Icon(Icon),
    /// A colour swatch.
    Swatch(Color32),
    /// A painted thumbnail of the given size (layer stacks, previews).
    Thumb(f32, ThumbPainter<'a>),
}

impl std::fmt::Debug for Leading<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Leading::None => write!(f, "None"),
            Leading::Icon(i) => write!(f, "Icon({i:?})"),
            Leading::Swatch(c) => write!(f, "Swatch({c:?})"),
            Leading::Thumb(s, _) => write!(f, "Thumb({s})"),
        }
    }
}

/// A list row.
#[derive(Debug)]
#[must_use = "call `.show(ui)`"]
pub struct ListRow<'a> {
    title: &'a str,
    secondary: &'a str,
    leading: Leading<'a>,
    meta: &'a str,
    tag: Option<(&'a str, Tone)>,
    selected: bool,
    selection: Selection,
}

impl<'a> ListRow<'a> {
    pub fn new(title: &'a str) -> Self {
        ListRow {
            title,
            secondary: "",
            leading: Leading::None,
            meta: "",
            tag: None,
            selected: false,
            selection: Selection::Strong,
        }
    }

    /// A second, secondary line.
    pub fn secondary(mut self, text: &'a str) -> Self {
        self.secondary = text;
        self
    }

    pub fn leading(mut self, leading: Leading<'a>) -> Self {
        self.leading = leading;
        self
    }

    pub fn icon(self, icon: Icon) -> Self {
        self.leading(Leading::Icon(icon))
    }

    /// A thumbnail `size` points square, painted by `paint`.
    pub fn thumb(self, size: f32, paint: impl FnOnce(&Painter, Rect) + 'a) -> Self {
        self.leading(Leading::Thumb(size, Box::new(paint)))
    }

    /// Trailing meta text (tabular, right-aligned: "335 mm").
    pub fn meta(mut self, meta: &'a str) -> Self {
        self.meta = meta;
        self
    }

    /// A trailing tag ("U 0,21").
    pub fn tag(mut self, text: &'a str, tone: Tone) -> Self {
        self.tag = Some((text, tone));
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub fn selection(mut self, selection: Selection) -> Self {
        self.selection = selection;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let c = colors(ui);
        let two = !self.secondary.is_empty();
        let thumb = matches!(self.leading, Leading::Thumb(..));
        let h = if two || thumb {
            size::ROW_TWO_LINE_H
        } else {
            size::ROW_H + 4.0
        };
        let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), h), Sense::click());
        state::a11y_selected(&resp, Role::ListItem, self.selected, self.title);
        if !ui.is_rect_visible(rect) {
            return resp;
        }
        let s = state::state(ui, &resp);
        let strong = self.selected && self.selection == Selection::Strong;
        let p = ui.painter().clone();
        if strong {
            p.rect_filled(rect, radius::ROW, c.highlight);
        } else if self.selected {
            p.rect_filled(rect, radius::ROW, c.selection);
        } else {
            let hover = state::hover_fill(ui, resp.id, s, c);
            if hover.a() > 0 {
                p.rect_filled(rect, radius::ROW, hover);
            }
        }
        let (fg, fg2) = if strong {
            (c.on_highlight, c.on_highlight.gamma_multiply(0.82))
        } else if s.disabled {
            (c.text_disabled, c.text_disabled)
        } else {
            (c.text, c.text_tertiary)
        };
        let mut x = rect.left() + space::XXL;
        match self.leading {
            Leading::None => {}
            Leading::Icon(i) => {
                let ic = if strong {
                    fg
                } else if self.selected {
                    c.accent
                } else {
                    c.icon
                };
                icon::paint(
                    &p,
                    Rect::from_center_size(pos2(x + 8.0, rect.center().y), Vec2::splat(16.0)),
                    i,
                    ic,
                );
                x += 16.0 + space::LG;
            }
            Leading::Swatch(col) => {
                let r = Rect::from_center_size(pos2(x + 6.0, rect.center().y), Vec2::splat(12.0));
                p.rect(
                    r,
                    radius::XS,
                    col,
                    Stroke::new(1.0, c.border_strong),
                    StrokeKind::Inside,
                );
                x += 12.0 + space::LG;
            }
            Leading::Thumb(sz, paint) => {
                let r =
                    Rect::from_center_size(pos2(x + sz / 2.0, rect.center().y), Vec2::splat(sz));
                paint(&p, r);
                let border = if strong {
                    Color32::from_white_alpha(150)
                } else {
                    c.border
                };
                p.rect_stroke(r, radius::ROW, Stroke::new(1.0, border), StrokeKind::Inside);
                x += sz + space::LG;
            }
        }
        let mut right = rect.right() - space::XXL;
        if let Some((t, tone)) = self.tag {
            let tw = 54.0;
            let tr = Rect::from_min_max(
                pos2(right - tw, rect.center().y - 9.0),
                pos2(right, rect.center().y + 9.0),
            );
            let tc = c.tone(tone);
            let (bg, tf) = if strong {
                (Color32::from_white_alpha(56), fg)
            } else {
                (tc.tint, tc.text)
            };
            p.rect_filled(tr, radius::PILL, bg);
            text::paint_tabular(&p, tr, t, Type::Caption, tf, Align::Center);
            right -= tw + space::LG;
        }
        if !self.meta.is_empty() {
            let mw = 52.0f32.max(text::tabular_width(&p, self.meta, Type::Body));
            let mr = Rect::from_min_max(pos2(right - mw, rect.top()), pos2(right, rect.bottom()));
            text::paint_tabular(
                &p,
                mr,
                self.meta,
                Type::Body,
                if strong { fg } else { c.icon },
                Align::Max,
            );
            right -= mw + space::LG;
        }
        let tw = (right - x).max(10.0);
        let t1 = text::truncated(
            &p,
            self.title,
            if two || thumb {
                Type::BodyStrong
            } else {
                Type::Body
            },
            fg,
            tw,
        );
        let elided = t1.elided;
        if two {
            let t2 = text::truncated(&p, self.secondary, Type::Body, fg2, tw);
            let total = t1.size().y + 2.0 + t2.size().y;
            let top = rect.center().y - total / 2.0;
            let h1 = t1.size().y;
            p.galley(pos2(x, top), t1, fg);
            p.galley(pos2(x, top + h1 + 2.0), t2, fg2);
        } else {
            text::paint_in(
                &p,
                Rect::from_min_max(pos2(x, rect.top()), pos2(right, rect.bottom())),
                t1,
                Align::Min,
                fg,
            );
        }
        if s.focused {
            state::focus_ring(ui, rect, radius::ROW, c);
        }
        if elided {
            resp.on_hover_text(self.title)
        } else {
            resp
        }
    }
}

/// A group header above list rows: an upper-case title and a count.
pub fn group_header(ui: &mut Ui, title: &str, count: Option<usize>) {
    let c = colors(ui);
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 24.0), Sense::hover());
    let inner = rect.shrink2(vec2(space::XXL, 0.0));
    let p = ui.painter();
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

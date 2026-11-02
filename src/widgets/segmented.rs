//! The segmented control: a track with a raised thumb on the chosen
//! segment (text or icon segments), the thumb sliding between segments.

use egui::{Align, Key, Rect, Response, Role, Sense, Ui, Vec2, pos2, vec2};

use crate::icon::{self, Icon};
use crate::state;
use crate::text::{self, Type};
use crate::theme::colors;
use crate::tokens::{Elevation, motion, radius, size};

/// One segment: a label and/or an icon, and a tooltip.
#[derive(Debug, Clone, Copy)]
pub struct Segment<'a, T> {
    pub value: T,
    pub label: &'a str,
    pub icon: Option<Icon>,
    pub tip: &'a str,
}

impl<'a, T> Segment<'a, T> {
    pub fn text(value: T, label: &'a str) -> Self {
        Segment {
            value,
            label,
            icon: None,
            tip: "",
        }
    }

    /// An icon segment; `tip` is its accessible name.
    pub fn icon(value: T, icon: Icon, tip: &'a str) -> Self {
        Segment {
            value,
            label: "",
            icon: Some(icon),
            tip,
        }
    }

    pub fn with_icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn tip(mut self, tip: &'a str) -> Self {
        self.tip = tip;
        self
    }
}

/// A segmented control.
#[derive(Debug)]
#[must_use = "call `.show(ui)`"]
pub struct Segmented<'a, T> {
    value: &'a mut T,
    segments: &'a [Segment<'a, T>],
    fill_width: bool,
    segment_width: Option<f32>,
    height: f32,
}

impl<'a, T: PartialEq + Copy> Segmented<'a, T> {
    pub fn new(value: &'a mut T, segments: &'a [Segment<'a, T>]) -> Self {
        Segmented {
            value,
            segments,
            fill_width: true,
            segment_width: None,
            height: size::FIELD_H + 4.0,
        }
    }

    /// Segments as wide as their content (else the control fills the
    /// available width).
    pub fn compact(mut self) -> Self {
        self.fill_width = false;
        self
    }

    /// Every segment this wide (icon segments: 30).
    pub fn segment_width(mut self, w: f32) -> Self {
        self.segment_width = Some(w);
        self.fill_width = false;
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.height = h;
        self
    }

    /// Shows the control; `changed()` on the response when the value did.
    pub fn show(self, ui: &mut Ui) -> Response {
        let c = colors(ui);
        let n = self.segments.len().max(1);
        let gap = 2.0;
        let natural: Vec<f32> = self
            .segments
            .iter()
            .map(|s| {
                if let Some(w) = self.segment_width {
                    return w;
                }
                let tw = if s.label.is_empty() {
                    0.0
                } else {
                    text::galley(ui.painter(), s.label, Type::BodyStrong, c.text)
                        .size()
                        .x
                };
                let iw = if s.icon.is_some() {
                    16.0 + if tw > 0.0 { 6.0 } else { 0.0 }
                } else {
                    0.0
                };
                tw + iw + 24.0
            })
            .collect();
        let total_natural: f32 = natural.iter().sum::<f32>() + gap * (n - 1) as f32 + 4.0;
        let w = if self.fill_width {
            ui.available_width().max(40.0)
        } else {
            total_natural.min(ui.available_width().max(40.0))
        };
        let (rect, mut resp) = ui.allocate_exact_size(vec2(w, self.height), Sense::hover());
        let inner = rect.shrink(2.0);
        let widths: Vec<f32> = if self.fill_width {
            vec![(inner.width() - gap * (n - 1) as f32) / n as f32; n]
        } else {
            let scale = (inner.width() - gap * (n - 1) as f32)
                / (total_natural - 4.0 - gap * (n - 1) as f32);
            natural.iter().map(|x| x * scale.min(1.0)).collect()
        };
        let mut rects = Vec::with_capacity(n);
        let mut x = inner.left();
        for sw in &widths {
            rects.push(Rect::from_min_size(
                pos2(x, inner.top()),
                vec2(*sw, inner.height()),
            ));
            x += sw + gap;
        }
        let p = ui.painter().clone();
        p.rect_filled(rect, radius::CONTROL, c.track);
        // The thumb slides to the chosen segment.
        let chosen = self.segments.iter().position(|s| s.value == *self.value);
        if let Some(i) = chosen {
            let target = rects[i];
            let left = ui.ctx().animate_value_with_time(
                resp.id.with("thumb_l"),
                target.left(),
                motion::NORMAL,
            );
            let right = ui.ctx().animate_value_with_time(
                resp.id.with("thumb_r"),
                target.right(),
                motion::NORMAL,
            );
            let thumb = Rect::from_min_max(pos2(left, target.top()), pos2(right, target.bottom()));
            if !ui.is_enabled() {
                p.rect_filled(thumb, radius::ROW, c.surface_sunken);
            } else {
                p.add(
                    Elevation::Raised
                        .shadow(c.dark)
                        .as_shape(thumb, radius::ROW),
                );
                p.rect_filled(thumb, radius::ROW, c.thumb);
            }
        }
        let mut changed = false;
        for (i, (seg, r)) in self.segments.iter().zip(&rects).enumerate() {
            let mut sr = ui.interact(*r, resp.id.with(i), Sense::click());
            let selected = chosen == Some(i);
            let name = if seg.label.is_empty() {
                seg.tip
            } else {
                seg.label
            };
            state::a11y_selected(&sr, Role::RadioButton, selected, name);
            let s = state::state(ui, &sr);
            if !selected && s.hovered {
                p.rect_filled(*r, radius::ROW, c.hover);
            }
            let fg = if s.disabled {
                c.text_disabled
            } else if selected {
                c.text
            } else {
                c.text_secondary
            };
            let t = if selected {
                Type::BodyStrong
            } else {
                Type::Body
            };
            let iw = if seg.icon.is_some() { 16.0 } else { 0.0 };
            let g = (!seg.label.is_empty())
                .then(|| text::truncated(&p, seg.label, t, fg, r.width() - 8.0 - iw));
            let gw = g.as_ref().map_or(0.0, |g| g.size().x);
            let content = iw + gw + if iw > 0.0 && gw > 0.0 { 6.0 } else { 0.0 };
            let mut cx = r.center().x - content / 2.0;
            if let Some(ic) = seg.icon {
                let ifg = if selected && !s.disabled {
                    c.accent
                } else {
                    fg
                };
                icon::paint(
                    &p,
                    Rect::from_center_size(pos2(cx + 8.0, r.center().y), Vec2::splat(16.0)),
                    ic,
                    ifg,
                );
                cx += iw + 6.0;
            }
            if let Some(g) = g {
                let elided = g.elided;
                text::paint_in(
                    &p,
                    Rect::from_min_max(pos2(cx, r.top()), pos2(cx + gw, r.bottom())),
                    g,
                    Align::Min,
                    fg,
                );
                if elided && seg.tip.is_empty() {
                    sr = sr.on_hover_text(seg.label);
                }
            }
            if !seg.tip.is_empty() {
                sr = sr.on_hover_text(seg.tip);
            }
            if s.focused {
                state::focus_ring(ui, *r, radius::ROW, c);
                // Arrow keys move the choice.
                let (left, right) = ui.input(|i| {
                    (
                        i.key_pressed(Key::ArrowLeft),
                        i.key_pressed(Key::ArrowRight),
                    )
                });
                let to = if left && i > 0 {
                    Some(i - 1)
                } else if right && i + 1 < n {
                    Some(i + 1)
                } else {
                    None
                };
                if let Some(j) = to {
                    *self.value = self.segments[j].value;
                    changed = true;
                    ui.memory_mut(|m| m.request_focus(resp.id.with(j)));
                }
            }
            if sr.clicked() && !selected {
                *self.value = seg.value;
                changed = true;
            }
        }
        if changed {
            resp.mark_changed();
        }
        resp
    }
}

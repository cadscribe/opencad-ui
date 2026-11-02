//! Scroll edges: the cue that a scroll area has more content above or
//! below ([`edges`]), or to the left or right ([`side_fades`], the
//! ribbon body).
//!
//! egui's own edge fade (`Spacing::scroll.fade`) is switched off in the
//! theme: it is 20 pt tall, takes its colour from the nearest frame up
//! the `Ui` stack (often a card or the window chrome rather than what is
//! under the scrolled content) and runs to the bottom of the area, over a
//! window's resize grip. Instead, after `ScrollArea::show`, a screen
//! calls [`edges`] with the output: the cue is clipped to the viewport,
//! leaves the scroll bar free, only appears in a direction with more
//! content, and is either a short fade into the area's own background or
//! a hairline (for areas over mixed surfaces).

use egui::{Color32, Mesh, Rect, Stroke, Ui, pos2, scroll_area::ScrollAreaOutput};

use crate::theme::colors;

/// The height of the fade (pt).
pub const FADE: f32 = 14.0;

/// The cue at the scrolled edges.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Edge {
    /// A fade into the colour of the surface the area is drawn on.
    Fade(Color32),
    /// A hairline divider with a faint shadow, as on macOS.
    Hairline,
}

/// The strength (0…1) of the top and bottom cue of a vertical scroll
/// area: 0 when there is nothing more in that direction, ramping up over
/// the first [`FADE`] points scrolled.
pub fn edge_strength(offset: f32, content: f32, viewport: f32) -> (f32, f32) {
    let overflow = content - viewport;
    if overflow <= 0.5 {
        return (0.0, 0.0);
    }
    let top = (offset / FADE).clamp(0.0, 1.0);
    let bottom = ((overflow - offset) / FADE).clamp(0.0, 1.0);
    (top, bottom)
}

/// Paints the cues of a vertical scroll area after it was shown.
pub fn edges<R>(ui: &Ui, out: &ScrollAreaOutput<R>, edge: Edge) {
    let view = out.inner_rect;
    let (top, bottom) = edge_strength(
        out.state.clamped_offset().y,
        out.content_size.y,
        view.height(),
    );
    if top <= 0.0 && bottom <= 0.0 {
        return;
    }
    // Leave the (floating) scroll bar alone.
    let s = &ui.spacing().scroll;
    let bar = s.bar_inner_margin + s.bar_width + s.bar_outer_margin;
    let rect = Rect::from_min_max(view.min, pos2(view.right() - bar, view.bottom()));
    let p = ui.painter_at(view);
    let c = colors(ui);
    for (t, at_top) in [(top, true), (bottom, false)] {
        if t <= 0.0 {
            continue;
        }
        match edge {
            Edge::Fade(bg) => {
                let band = if at_top {
                    Rect::from_min_max(rect.min, pos2(rect.right(), rect.top() + FADE))
                } else {
                    Rect::from_min_max(pos2(rect.left(), rect.bottom() - FADE), rect.max)
                };
                let solid = bg.gamma_multiply(t);
                let (upper, lower) = if at_top {
                    (solid, Color32::TRANSPARENT)
                } else {
                    (Color32::TRANSPARENT, solid)
                };
                p.add(gradient(band, upper, lower));
            }
            Edge::Hairline => {
                let y = if at_top { view.top() } else { view.bottom() }
                    + if at_top { 0.5 } else { -0.5 };
                p.hline(
                    view.left()..=view.right(),
                    y,
                    Stroke::new(1.0, c.divider.gamma_multiply(t)),
                );
                let shadow = c.shadow.gamma_multiply(0.5 * t);
                let band = if at_top {
                    Rect::from_min_max(pos2(view.left(), y), pos2(view.right(), y + 4.0))
                } else {
                    Rect::from_min_max(pos2(view.left(), y - 4.0), pos2(view.right(), y))
                };
                let (upper, lower) = if at_top {
                    (shadow, Color32::TRANSPARENT)
                } else {
                    (Color32::TRANSPARENT, shadow)
                };
                p.add(gradient(band, upper, lower));
            }
        }
    }
}

/// Paints fades into `bg` at the left and right of a horizontal scroll
/// area after it was shown, where there is more content that way.
pub fn side_fades<R>(ui: &Ui, out: &ScrollAreaOutput<R>, bg: Color32) {
    let view = out.inner_rect;
    let (left, right) = edge_strength(
        out.state.clamped_offset().x,
        out.content_size.x,
        view.width(),
    );
    let p = ui.painter_at(view);
    if left > 0.0 {
        let band = Rect::from_min_max(view.min, pos2(view.left() + FADE, view.bottom()));
        p.add(side_gradient(
            band,
            bg.gamma_multiply(left),
            Color32::TRANSPARENT,
        ));
    }
    if right > 0.0 {
        let band = Rect::from_min_max(pos2(view.right() - FADE, view.top()), view.max);
        p.add(side_gradient(
            band,
            Color32::TRANSPARENT,
            bg.gamma_multiply(right),
        ));
    }
}

/// A horizontal gradient over `rect`.
fn side_gradient(rect: Rect, left: Color32, right: Color32) -> Mesh {
    let mut m = Mesh::default();
    m.colored_vertex(rect.left_top(), left);
    m.colored_vertex(rect.right_top(), right);
    m.colored_vertex(rect.left_bottom(), left);
    m.colored_vertex(rect.right_bottom(), right);
    m.add_triangle(0, 1, 2);
    m.add_triangle(1, 3, 2);
    m
}

/// A vertical gradient over `rect`.
fn gradient(rect: Rect, top: Color32, bottom: Color32) -> Mesh {
    let mut m = Mesh::default();
    m.colored_vertex(rect.left_top(), top);
    m.colored_vertex(rect.right_top(), top);
    m.colored_vertex(rect.left_bottom(), bottom);
    m.colored_vertex(rect.right_bottom(), bottom);
    m.add_triangle(0, 1, 2);
    m.add_triangle(1, 3, 2);
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_cue_without_overflow() {
        assert_eq!(edge_strength(0.0, 300.0, 400.0), (0.0, 0.0));
        assert_eq!(edge_strength(0.0, 400.0, 400.0), (0.0, 0.0));
    }

    #[test]
    fn cue_only_towards_more_content() {
        // At the top: only the bottom cue.
        assert_eq!(edge_strength(0.0, 1000.0, 400.0), (0.0, 1.0));
        // At the bottom: only the top cue.
        assert_eq!(edge_strength(600.0, 1000.0, 400.0), (1.0, 0.0));
        // In between: both.
        assert_eq!(edge_strength(300.0, 1000.0, 400.0), (1.0, 1.0));
        // Ramping in over the first points.
        let (t, b) = edge_strength(FADE / 2.0, 1000.0, 400.0);
        assert!((t - 0.5).abs() < 1e-6 && (b - 1.0).abs() < 1e-6);
    }
}

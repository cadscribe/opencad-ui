//! The icon system: SVG path data drawn as egui shapes at any size.
//!
//! Every [`Icon`] is a list of layers in a square view box (16, 18 or
//! 24 units): stroked paths (round caps and joins), dashed strokes,
//! thinner or thicker strokes and filled paths. Paths are parsed once
//! ([`crate::svg`]) and cached; painting only transforms points, so icons
//! are crisp at 1× and 2× and in every size. The stroke width follows the
//! drawn size ([`stroke_width`]), like an icon font's optical sizes.

use std::sync::OnceLock;

use egui::epaint::{Mesh, PathShape, PathStroke};
use egui::{Color32, Painter, Pos2, Rect, Response, Sense, Shape, Stroke, Ui, Vec2, pos2, vec2};

use crate::svg::{self, SubPath};

/// One layer of an icon.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Layer {
    /// A stroked path.
    S(&'static str),
    /// A stroked path, the width scaled by the factor.
    W(&'static str, f32),
    /// A dashed path: dash and gap in view-box units.
    D(&'static str, f32, f32),
    /// A filled path (simple polygons; concave ones are triangulated).
    F(&'static str),
}

impl Layer {
    pub fn d(self) -> &'static str {
        match self {
            Layer::S(d) | Layer::W(d, _) | Layer::D(d, _, _) | Layer::F(d) => d,
        }
    }
}

/// An icon's geometry: its view box size and layers.
#[derive(Debug, Clone, Copy)]
pub struct Def {
    pub view: f32,
    pub layers: &'static [Layer],
}

macro_rules! icons {
    ($( $(#[$m:meta])* $name:ident = $view:literal [ $($layer:expr),* $(,)? ] ; )*) => {
        /// The icon set (tools from the ribbon mockups, interface icons).
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub enum Icon {
            $( $(#[$m])* $name, )*
        }

        impl Icon {
            /// Every icon, in declaration order.
            pub const ALL: &'static [Icon] = &[$( Icon::$name, )*];

            /// The variant name (gallery, diagnostics).
            pub fn name(self) -> &'static str {
                match self {
                    $( Icon::$name => stringify!($name), )*
                }
            }

            /// The icon's geometry.
            pub fn def(self) -> Def {
                use Layer::*;
                match self {
                    $( Icon::$name => Def { view: $view as f32, layers: &[$($layer),*] }, )*
                }
            }
        }
    };
}

include!("icon_data.rs");

/// Parsed layers of every icon (in view-box units).
fn parsed() -> &'static Vec<Vec<Vec<SubPath>>> {
    static CACHE: OnceLock<Vec<Vec<Vec<SubPath>>>> = OnceLock::new();
    CACHE.get_or_init(|| {
        Icon::ALL
            .iter()
            .map(|icon| {
                let def = icon.def();
                def.layers
                    .iter()
                    .map(|l| svg::parse(l.d(), def.view / 480.0).unwrap_or_default())
                    .collect()
            })
            .collect()
    })
}

/// The stroke width in points of an icon drawn `size` points tall:
/// about 1.25 at 16, 1.6 at 26 (the mockups' 1.3–1.8 view-box units).
pub fn stroke_width(size: f32) -> f32 {
    (0.6 + size * 0.04).clamp(1.0, 2.4)
}

/// Paints `icon` into `rect` (centred, square) in `color`.
pub fn paint(painter: &Painter, rect: Rect, icon: Icon, color: Color32) {
    let size = rect.width().min(rect.height());
    paint_with(painter, rect, icon, color, stroke_width(size));
}

/// [`paint`] with an explicit stroke width in points.
pub fn paint_with(painter: &Painter, rect: Rect, icon: Icon, color: Color32, width: f32) {
    let def = icon.def();
    let size = rect.width().min(rect.height());
    let scale = size / def.view;
    // Snap the origin to physical pixels so that straight lines on the
    // grid stay crisp.
    let ppp = painter.pixels_per_point();
    let o = rect.center() - Vec2::splat(size / 2.0);
    let origin = pos2((o.x * ppp).round() / ppp, (o.y * ppp).round() / ppp);
    let map = |p: Pos2| origin + p.to_vec2() * scale;
    let layers = &parsed()[icon as usize];
    for (layer, paths) in def.layers.iter().zip(layers) {
        match *layer {
            Layer::S(_) => stroke(painter, paths, &map, width, color, None),
            Layer::W(_, f) => stroke(painter, paths, &map, width * f, color, None),
            Layer::D(_, on, off) => stroke(
                painter,
                paths,
                &map,
                width,
                color,
                Some((on * scale, off * scale)),
            ),
            Layer::F(_) => {
                for p in paths {
                    fill(
                        painter,
                        &p.points.iter().map(|q| map(*q)).collect::<Vec<_>>(),
                        color,
                    );
                }
            }
        }
    }
}

fn stroke(
    painter: &Painter,
    paths: &[SubPath],
    map: &impl Fn(Pos2) -> Pos2,
    width: f32,
    color: Color32,
    dash: Option<(f32, f32)>,
) {
    let st = Stroke::new(width, color);
    for p in paths {
        let pts: Vec<Pos2> = p.points.iter().map(|q| map(*q)).collect();
        if let Some((on, off)) = dash {
            let mut line = pts.clone();
            if p.closed {
                line.push(pts[0]);
            }
            painter.extend(Shape::dashed_line(&line, st, on, off));
            continue;
        }
        let tiny = pts.len() == 2 && (pts[1] - pts[0]).length() < width * 0.5;
        if tiny {
            // A dot ("M8 12h.01" with round caps).
            painter.circle_filled(pts[0], width * 0.5 * 1.15, color);
            continue;
        }
        if p.closed {
            painter.add(PathShape::closed_line(
                pts.clone(),
                PathStroke::new(width, color),
            ));
        } else {
            painter.add(PathShape::line(pts.clone(), PathStroke::new(width, color)));
        }
        // Round caps and joins: a disc on every corner where the outline
        // turns sharply, and on open ends.
        let r = width * 0.5;
        let n = pts.len();
        for i in 0..n {
            if !p.corners[i] {
                continue;
            }
            let end = !p.closed && (i == 0 || i + 1 == n);
            let sharp = if end {
                true
            } else {
                let prev = pts[(i + n - 1) % n];
                let next = pts[(i + 1) % n];
                let a = (pts[i] - prev).normalized();
                let b = (next - pts[i]).normalized();
                a.dot(b) < 0.5
            };
            if sharp {
                painter.circle_filled(pts[i], r, color);
            }
        }
    }
}

fn area2(pts: &[Pos2]) -> f32 {
    let mut a = 0.0;
    for i in 0..pts.len() {
        let p = pts[i];
        let q = pts[(i + 1) % pts.len()];
        a += p.x * q.y - q.x * p.y;
    }
    a
}

fn convex(pts: &[Pos2]) -> bool {
    let n = pts.len();
    let mut sign = 0i8;
    for i in 0..n {
        let a = pts[i];
        let b = pts[(i + 1) % n];
        let c = pts[(i + 2) % n];
        let cross = (b - a).x * (c - b).y - (b - a).y * (c - b).x;
        if cross.abs() > 1e-6 {
            let s = if cross > 0.0 { 1 } else { -1 };
            if sign != 0 && s != sign {
                return false;
            }
            sign = s;
        }
    }
    true
}

/// Triangulates a simple polygon (ear clipping); indices into `pts`.
pub fn triangulate(pts: &[Pos2]) -> Vec<[usize; 3]> {
    let n = pts.len();
    if n < 3 {
        return Vec::new();
    }
    let ccw = area2(pts) > 0.0;
    let mut idx: Vec<usize> = (0..n).collect();
    let mut out = Vec::with_capacity(n - 2);
    let cross = |a: Pos2, b: Pos2, c: Pos2| (b - a).x * (c - a).y - (b - a).y * (c - a).x;
    let inside = |p: Pos2, a: Pos2, b: Pos2, c: Pos2| {
        let d1 = cross(a, b, p);
        let d2 = cross(b, c, p);
        let d3 = cross(c, a, p);
        let neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
        let pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
        !(neg && pos)
    };
    let mut guard = 0;
    while idx.len() > 3 && guard < n * n {
        guard += 1;
        let m = idx.len();
        let mut clipped = false;
        for i in 0..m {
            let (ia, ib, ic) = (idx[(i + m - 1) % m], idx[i], idx[(i + 1) % m]);
            let (a, b, c) = (pts[ia], pts[ib], pts[ic]);
            let c2 = cross(a, b, c);
            let is_convex = if ccw { c2 > 0.0 } else { c2 < 0.0 };
            if !is_convex {
                continue;
            }
            let blocked = idx
                .iter()
                .any(|&j| j != ia && j != ib && j != ic && inside(pts[j], a, b, c));
            if blocked {
                continue;
            }
            out.push([ia, ib, ic]);
            idx.remove(i);
            clipped = true;
            break;
        }
        if !clipped {
            break;
        }
    }
    if idx.len() == 3 {
        out.push([idx[0], idx[1], idx[2]]);
    }
    out
}

/// Fills a simple polygon.
pub fn fill(painter: &Painter, pts: &[Pos2], color: Color32) {
    if pts.len() < 3 {
        return;
    }
    if convex(pts) {
        painter.add(PathShape::convex_polygon(pts.to_vec(), color, Stroke::NONE));
        return;
    }
    let mut mesh = Mesh::default();
    for p in pts {
        mesh.colored_vertex(*p, color);
    }
    for [a, b, c] in triangulate(pts) {
        mesh.add_triangle(a as u32, b as u32, c as u32);
    }
    painter.add(Shape::mesh(mesh));
    // Anti-aliased edge.
    painter.add(PathShape::closed_line(
        pts.to_vec(),
        PathStroke::new(0.6, color),
    ));
}

/// An icon as a widget of `size` points (not interactive).
pub fn icon(ui: &mut Ui, icon: Icon, size: f32, color: Color32) -> Response {
    let (rect, resp) = ui.allocate_exact_size(vec2(size, size), Sense::hover());
    paint(ui.painter(), rect, icon, color);
    resp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_icon_parses_and_fits_its_view_box() {
        for icon in Icon::ALL {
            let def = icon.def();
            assert!(!def.layers.is_empty(), "{icon:?}");
            for layer in def.layers {
                let paths = svg::parse(layer.d(), 0.05)
                    .unwrap_or_else(|e| panic!("{icon:?}: {e:?} in {:?}", layer.d()));
                assert!(!paths.is_empty(), "{icon:?}");
                let b = svg::bounds(&paths);
                let v = def.view;
                assert!(
                    b.min.x >= -0.01
                        && b.min.y >= -0.01
                        && b.max.x <= v + 0.01
                        && b.max.y <= v + 0.01,
                    "{icon:?} {b:?} outside 0..{v}"
                );
            }
        }
        assert_eq!(parsed().len(), Icon::ALL.len());
        for (i, icon) in Icon::ALL.iter().enumerate() {
            assert_eq!(*icon as usize, i);
        }
    }

    #[test]
    fn names_are_unique() {
        let mut names: Vec<_> = Icon::ALL.iter().map(|i| i.name()).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), Icon::ALL.len());
    }

    #[test]
    fn triangulation_of_a_concave_polygon() {
        // An L shape: 6 vertices, 4 triangles.
        let l = [
            pos2(0.0, 0.0),
            pos2(2.0, 0.0),
            pos2(2.0, 1.0),
            pos2(1.0, 1.0),
            pos2(1.0, 2.0),
            pos2(0.0, 2.0),
        ];
        assert!(!convex(&l));
        let t = triangulate(&l);
        assert_eq!(t.len(), 4);
        let area: f32 = t
            .iter()
            .map(|[a, b, c]| area2(&[l[*a], l[*b], l[*c]]).abs() / 2.0)
            .sum();
        assert!((area - 3.0).abs() < 1e-4, "{area}");
    }

    #[test]
    fn stroke_width_grows_with_size() {
        assert!(stroke_width(16.0) < stroke_width(26.0));
        assert!((stroke_width(16.0) - 1.24).abs() < 0.05);
    }
}

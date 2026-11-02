//! A small SVG path parser: the `d` attribute (M L H V A C S Q T Z, upper
//! and lower case) to flattened polylines in view-box units.

use egui::{Pos2, Vec2, pos2, vec2};

/// One subpath: its points, which of them are corners (command end
/// points, as opposed to points inside a flattened curve) and whether it
/// is closed.
#[derive(Debug, Clone, PartialEq)]
pub struct SubPath {
    pub points: Vec<Pos2>,
    pub corners: Vec<bool>,
    pub closed: bool,
}

/// A parse error: the byte offset and what was expected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Error {
    pub at: usize,
    pub what: &'static str,
}

struct Lexer<'a> {
    s: &'a [u8],
    i: usize,
}

impl Lexer<'_> {
    fn skip(&mut self) {
        while self.i < self.s.len() && matches!(self.s[self.i], b' ' | b',' | b'\n' | b'\t' | b'\r')
        {
            self.i += 1;
        }
    }

    fn command(&mut self) -> Option<u8> {
        self.skip();
        let c = *self.s.get(self.i)?;
        if c.is_ascii_alphabetic() && c != b'e' && c != b'E' {
            self.i += 1;
            Some(c)
        } else {
            None
        }
    }

    fn at_number(&mut self) -> bool {
        self.skip();
        self.s
            .get(self.i)
            .is_some_and(|c| c.is_ascii_digit() || matches!(c, b'-' | b'+' | b'.'))
    }

    fn number(&mut self) -> Result<f32, Error> {
        self.skip();
        let start = self.i;
        let s = self.s;
        let mut i = self.i;
        if i < s.len() && matches!(s[i], b'-' | b'+') {
            i += 1;
        }
        let mut dot = false;
        let mut digits = false;
        while i < s.len() {
            match s[i] {
                b'0'..=b'9' => digits = true,
                b'.' if !dot => dot = true,
                _ => break,
            }
            i += 1;
        }
        if digits && i < s.len() && matches!(s[i], b'e' | b'E') {
            let mut j = i + 1;
            if j < s.len() && matches!(s[j], b'-' | b'+') {
                j += 1;
            }
            if j < s.len() && s[j].is_ascii_digit() {
                while j < s.len() && s[j].is_ascii_digit() {
                    j += 1;
                }
                i = j;
            }
        }
        if !digits {
            return Err(Error {
                at: start,
                what: "a number",
            });
        }
        self.i = i;
        std::str::from_utf8(&s[start..i])
            .ok()
            .and_then(|t| t.parse().ok())
            .ok_or(Error {
                at: start,
                what: "a number",
            })
    }

    /// An arc flag: a single `0` or `1`, separators optional.
    fn flag(&mut self) -> Result<bool, Error> {
        self.skip();
        match self.s.get(self.i) {
            Some(b'0') => {
                self.i += 1;
                Ok(false)
            }
            Some(b'1') => {
                self.i += 1;
                Ok(true)
            }
            _ => Err(Error {
                at: self.i,
                what: "an arc flag",
            }),
        }
    }

    fn point(&mut self) -> Result<Vec2, Error> {
        let x = self.number()?;
        let y = self.number()?;
        Ok(vec2(x, y))
    }
}

struct Builder {
    paths: Vec<SubPath>,
    current: Option<SubPath>,
    pos: Pos2,
    start: Pos2,
    /// Reflection of the last control point (S / T).
    last_cubic: Option<Pos2>,
    last_quad: Option<Pos2>,
    tolerance: f32,
}

impl Builder {
    fn move_to(&mut self, p: Pos2) {
        self.finish();
        self.current = Some(SubPath {
            points: vec![p],
            corners: vec![true],
            closed: false,
        });
        self.pos = p;
        self.start = p;
    }

    fn push(&mut self, p: Pos2, corner: bool) {
        let path = self.current.get_or_insert_with(|| SubPath {
            points: vec![self.pos],
            corners: vec![true],
            closed: false,
        });
        path.points.push(p);
        path.corners.push(corner);
    }

    fn line_to(&mut self, p: Pos2) {
        self.push(p, true);
        self.pos = p;
    }

    fn segments(&self, length: f32) -> usize {
        ((length / (self.tolerance * 12.0)).ceil() as usize).clamp(4, 64)
    }

    fn cubic_to(&mut self, c1: Pos2, c2: Pos2, p: Pos2) {
        let p0 = self.pos;
        let len = (c1 - p0).length() + (c2 - c1).length() + (p - c2).length();
        let n = self.segments(len);
        for i in 1..=n {
            let t = i as f32 / n as f32;
            let u = 1.0 - t;
            let q = p0.to_vec2() * (u * u * u)
                + c1.to_vec2() * (3.0 * u * u * t)
                + c2.to_vec2() * (3.0 * u * t * t)
                + p.to_vec2() * (t * t * t);
            self.push(q.to_pos2(), i == n);
        }
        self.pos = p;
        self.last_cubic = Some(c2);
    }

    fn quad_to(&mut self, c: Pos2, p: Pos2) {
        let p0 = self.pos;
        let len = (c - p0).length() + (p - c).length();
        let n = self.segments(len);
        for i in 1..=n {
            let t = i as f32 / n as f32;
            let u = 1.0 - t;
            let q = p0.to_vec2() * (u * u) + c.to_vec2() * (2.0 * u * t) + p.to_vec2() * (t * t);
            self.push(q.to_pos2(), i == n);
        }
        self.pos = p;
        self.last_quad = Some(c);
    }

    /// Elliptical arc, SVG implementation notes F.6.5 (endpoint to centre
    /// parameterisation).
    fn arc_to(&mut self, rx: f32, ry: f32, rot_deg: f32, large: bool, sweep: bool, p: Pos2) {
        let p0 = self.pos;
        if (p - p0).length() < 1e-6 {
            return;
        }
        let (mut rx, mut ry) = (rx.abs(), ry.abs());
        if rx < 1e-6 || ry < 1e-6 {
            self.line_to(p);
            return;
        }
        let phi = rot_deg.to_radians();
        let (sin, cos) = phi.sin_cos();
        let d = (p0 - p) / 2.0;
        let x1 = cos * d.x + sin * d.y;
        let y1 = -sin * d.x + cos * d.y;
        let lambda = (x1 * x1) / (rx * rx) + (y1 * y1) / (ry * ry);
        if lambda > 1.0 {
            let s = lambda.sqrt();
            rx *= s;
            ry *= s;
        }
        let num = rx * rx * ry * ry - rx * rx * y1 * y1 - ry * ry * x1 * x1;
        let den = rx * rx * y1 * y1 + ry * ry * x1 * x1;
        let mut k = (num / den).max(0.0).sqrt();
        if large == sweep {
            k = -k;
        }
        let cx1 = k * rx * y1 / ry;
        let cy1 = -k * ry * x1 / rx;
        let mid = (p0.to_vec2() + p.to_vec2()) / 2.0;
        let c = pos2(cos * cx1 - sin * cy1 + mid.x, sin * cx1 + cos * cy1 + mid.y);
        let angle = |u: Vec2, v: Vec2| {
            let a = u.x * v.y - u.y * v.x;
            let b = u.dot(v);
            a.atan2(b)
        };
        let u = vec2((x1 - cx1) / rx, (y1 - cy1) / ry);
        let v = vec2((-x1 - cx1) / rx, (-y1 - cy1) / ry);
        let theta1 = angle(vec2(1.0, 0.0), u);
        let mut delta = angle(u, v);
        if !sweep && delta > 0.0 {
            delta -= std::f32::consts::TAU;
        } else if sweep && delta < 0.0 {
            delta += std::f32::consts::TAU;
        }
        let n = self
            .segments(delta.abs() * rx.max(ry))
            .max((delta.abs() / 0.2).ceil() as usize);
        for i in 1..=n {
            let t = theta1 + delta * i as f32 / n as f32;
            let (st, ct) = t.sin_cos();
            let q = if i == n {
                p
            } else {
                pos2(
                    c.x + rx * ct * cos - ry * st * sin,
                    c.y + rx * ct * sin + ry * st * cos,
                )
            };
            self.push(q, i == n);
        }
        self.pos = p;
    }

    fn close(&mut self) {
        if let Some(path) = &mut self.current {
            // Drop a duplicate end point; the outline closes anyway.
            if path.points.len() > 1
                && path
                    .points
                    .last()
                    .is_some_and(|l| (*l - self.start).length() < 1e-4)
            {
                path.points.pop();
                path.corners.pop();
            }
            path.closed = true;
        }
        self.finish();
        self.pos = self.start;
    }

    fn finish(&mut self) {
        if let Some(path) = self.current.take() {
            if path.points.len() > 1 || path.closed {
                self.paths.push(path);
            } else if path.points.len() == 1 {
                // A lone move-to is dropped.
            }
        }
    }
}

/// Parses and flattens `d`; `tolerance` is the flattening step scale in
/// view-box units (smaller = more segments).
pub fn parse(d: &str, tolerance: f32) -> Result<Vec<SubPath>, Error> {
    let mut lx = Lexer {
        s: d.as_bytes(),
        i: 0,
    };
    let mut b = Builder {
        paths: Vec::new(),
        current: None,
        pos: Pos2::ZERO,
        start: Pos2::ZERO,
        last_cubic: None,
        last_quad: None,
        tolerance: tolerance.max(1e-3),
    };
    let mut cmd: Option<u8> = None;
    loop {
        let c = match lx.command() {
            Some(c) => c,
            None => {
                lx.skip();
                if lx.i >= lx.s.len() {
                    break;
                }
                // Implicit repetition of the last command.
                match cmd {
                    Some(b'M') => b'L',
                    Some(b'm') => b'l',
                    Some(c) if !matches!(c, b'Z' | b'z') && lx.at_number() => c,
                    _ => {
                        return Err(Error {
                            at: lx.i,
                            what: "a command",
                        });
                    }
                }
            }
        };
        cmd = Some(c);
        let rel = c.is_ascii_lowercase();
        let base = if rel { b.pos.to_vec2() } else { Vec2::ZERO };
        let (keep_cubic, keep_quad) = (b.last_cubic.take(), b.last_quad.take());
        match c.to_ascii_uppercase() {
            b'M' => {
                let p = (lx.point()? + base).to_pos2();
                b.move_to(p);
            }
            b'L' => {
                let p = (lx.point()? + base).to_pos2();
                b.line_to(p);
            }
            b'H' => {
                let x = lx.number()? + base.x;
                b.line_to(pos2(x, b.pos.y));
            }
            b'V' => {
                let y = lx.number()? + base.y;
                b.line_to(pos2(b.pos.x, y));
            }
            b'C' => {
                let c1 = (lx.point()? + base).to_pos2();
                let c2 = (lx.point()? + base).to_pos2();
                let p = (lx.point()? + base).to_pos2();
                b.cubic_to(c1, c2, p);
            }
            b'S' => {
                let c1 = keep_cubic.map_or(b.pos, |k| b.pos + (b.pos - k));
                let c2 = (lx.point()? + base).to_pos2();
                let p = (lx.point()? + base).to_pos2();
                b.cubic_to(c1, c2, p);
            }
            b'Q' => {
                let c1 = (lx.point()? + base).to_pos2();
                let p = (lx.point()? + base).to_pos2();
                b.quad_to(c1, p);
            }
            b'T' => {
                let c1 = keep_quad.map_or(b.pos, |k| b.pos + (b.pos - k));
                let p = (lx.point()? + base).to_pos2();
                b.quad_to(c1, p);
            }
            b'A' => {
                let rx = lx.number()?;
                let ry = lx.number()?;
                let rot = lx.number()?;
                let large = lx.flag()?;
                let sweep = lx.flag()?;
                let p = (lx.point()? + base).to_pos2();
                b.arc_to(rx, ry, rot, large, sweep, p);
            }
            b'Z' => b.close(),
            _ => {
                return Err(Error {
                    at: lx.i.saturating_sub(1),
                    what: "a known command",
                });
            }
        }
    }
    b.finish();
    Ok(b.paths)
}

/// The bounding box of the parsed paths (tests, layout checks).
pub fn bounds(paths: &[SubPath]) -> egui::Rect {
    let mut r = egui::Rect::NOTHING;
    for p in paths {
        for q in &p.points {
            r.extend_with(*q);
        }
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: Pos2, b: Pos2) -> bool {
        (a - b).length() < 1e-3
    }

    #[test]
    fn lines_and_relative_commands() {
        let p = parse("M3 8h18v8H3zM3 12h18M9 8v4", 0.05).unwrap();
        assert_eq!(p.len(), 3);
        assert!(p[0].closed);
        assert_eq!(p[0].points.len(), 4);
        assert!(close(p[0].points[2], pos2(21.0, 16.0)));
        assert!(!p[1].closed);
        assert!(close(p[2].points[1], pos2(9.0, 12.0)));
    }

    #[test]
    fn implicit_line_after_move_and_compact_numbers() {
        let p = parse("M6 3l12 7-5 1.5L11 18z", 0.05).unwrap();
        assert_eq!(p[0].points.len(), 4);
        assert!(close(p[0].points[2], pos2(13.0, 11.5)));
        let p = parse("M8 12h.01", 0.05).unwrap();
        assert!(close(p[0].points[1], pos2(8.01, 12.0)));
        let p = parse("M1.5.5l-1-1", 0.05).unwrap();
        assert!(close(p[0].points[0], pos2(1.5, 0.5)));
        assert!(close(p[0].points[1], pos2(0.5, -0.5)));
    }

    #[test]
    fn arcs_reach_their_end_and_stay_on_the_circle() {
        // A full circle from two half arcs (the mockups' circle idiom).
        let p = parse("M4 12a8 8 0 1 0 16 0a8 8 0 1 0-16 0", 0.05).unwrap();
        let c = pos2(12.0, 12.0);
        for q in &p[0].points {
            assert!(((*q - c).length() - 8.0).abs() < 0.05, "{q:?}");
        }
        assert!(close(*p[0].points.last().unwrap(), pos2(4.0, 12.0)));
        let b = bounds(&p);
        assert!(
            (b.width() - 16.0).abs() < 0.1 && (b.height() - 16.0).abs() < 0.1,
            "{b:?}"
        );
        // Compact flags: "a3 3 0 0 1-2.4-5.3".
        let p = parse("M11 20a7 7 0 0 0-2.4-5.3", 0.05).unwrap();
        assert!(close(*p[0].points.last().unwrap(), pos2(8.6, 14.7)));
    }

    #[test]
    fn curves() {
        let p = parse("M3 17c4-12 8 8 12-4s4-6 6-6", 0.05).unwrap();
        assert!(close(*p[0].points.last().unwrap(), pos2(21.0, 7.0)));
        let corners = p[0].corners.iter().filter(|c| **c).count();
        assert_eq!(corners, 3);
        let p = parse("M2 2Q6 0 10 2T18 2", 0.05).unwrap();
        assert!(close(*p[0].points.last().unwrap(), pos2(18.0, 2.0)));
    }

    #[test]
    fn errors() {
        assert!(parse("M3", 0.05).is_err());
        assert!(parse("X1 2", 0.05).is_err());
        assert!(parse("M1 1A1 1 0 2 0 3 3", 0.05).is_err());
        assert_eq!(parse("", 0.05).unwrap(), vec![]);
    }
}
